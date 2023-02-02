use bytes::{Buf, BufMut, BytesMut};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use prost::Message;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::debug;

use crate::{CommandRequest, CommandResponse, KvError};

/// 长度占用 4字节
pub const LEN_LEN: usize = 4;
/// 长度占 31位bit,所以最大的 frame是2G()
const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;
/// 如果 payload超过了 1436字节, 就做压缩
const COMPRESSION_LIMIT: usize = 1436;
/// 代表压缩的 bit, 长度4字节的最高位
const COMPRESSION_BIT: usize = 1 << 31;

pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    /// 把 Req/Res Message`消息封装/encode`编码成一个 frame, 如果需要会进行压缩,
    /// 并写入 buf: ByteMut中
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        let size = self.encoded_len();
        // 如果超过最大 frame 2G,则抛出错误
        if size >= MAX_FRAME {
            return Err(KvError::FrameError(size, COMPRESSION_LIMIT));
        }
        // 我们先写入长度,如果需要压缩,在重写压缩后的长度
        buf.put_u32(size as _);

        // 超过 1436字节, 压缩
        if size > COMPRESSION_LIMIT {
            let mut buf1 = Vec::with_capacity(size);
            self.encode(&mut buf1)?;

            // BytesMut 支持逻辑上的 split(之后还能 unSplit)
            // 所以我们先把长度这 4字节拿走, 然后清除
            let payload = buf.split_off(LEN_LEN);
            buf.clear();

            // 处理 gzip压缩, 具体可以参考 flate2文档
            let mut encoder = GzEncoder::new(payload.writer(), Compression::default());
            // 消息内容写入到 gzip
            encoder.write_all(&buf1[..])?;

            // 压缩完成(finish)后，从 gzip encoder中把 BytesMut再拿回来(into_inner)
            let payload = encoder.finish()?.into_inner(); // finish`完成
            debug!(
                "Encode a frame: size {}({}`编码的大小)",
                size,
                payload.len()
            );

            // 写入压缩后的长度
            buf.put_u32((payload.len() | COMPRESSION_BIT) as _);
            // 把BytesMut 再合并回来
            buf.unsplit(payload);
        } else {
            self.encode(buf)?
        }

        Ok(())
    }

    /// buf是封装/encode`编码后的 frame
    /// 将入参 buf: ByteMut decode`解码成 Req/Res Message
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, KvError> {
        // 先取 4字节, 从中拿出长度并 compression`压缩 bit
        let header = buf.get_u32() as usize;
        let (len, compressed) = decode_header(header);
        debug!(
            "Got`得到 a`一个 frame`帧`框架: msg`消息 len`长度为: {}, compression`压缩 {}",
            len, compressed
        );

        if compressed {
            // 解压缩,创建后立即解压
            let mut decoder = GzDecoder::new(&buf[..len]);
            let mut buf1 = Vec::with_capacity(len * 2);
            // 将解压缩的数据写入到 buf,如果没有遇到错误中断,返回写入的字节数
            decoder.read_to_end(&mut buf1)?;
            // 移动 buf的前进指针 +len,返回新的起点 buf的切片
            buf.advance(len);

            // decode 成相应的消息
            Ok(Self::decode(&buf1[..buf1.len()])?)
        } else {
            let msg = Self::decode(&buf[..len])?;
            buf.advance(len);
            Ok(msg)
        }
    }
}

fn decode_header(header: usize) -> (usize, bool) {
    let len = header & !COMPRESSION_BIT;
    // 是否需要压缩, 如果 header大于等于
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (len, compressed)
}

impl FrameCoder for CommandRequest {}
impl FrameCoder for CommandResponse {}

/// 从 stream中读取一个完整的 frame, 写入到 buf中
pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), KvError>
where
    // AsyncRead tikio下的 Trait, 实现即可使用 AsyncReadExt提供的方法做异步读取
    // ` impl<R: AsyncRead + ?Sized> AsyncReadExt for R `
    // Send: 需要能在不同线程间移动所有权
    // Unpin: 编译器抱怨一个泛型参数 “cannot be unpinned``不能取消固定” ，一般来说，这个泛型参数需要加 Unpin 的约束
    S: AsyncRead + Unpin + Send,
{
    let header = stream.read_u32().await? as usize;
    let (len, _compressed) = decode_header(header);
    // 如果没有这么大的内存; 分配至少一个 freame的内存,保证它可用
    buf.reserve(LEN_LEN + len);
    buf.put_u32(header as _);
    //  移动 buf内部的指针, 返回移动后的切片, advance_mut是 unsafe的,它不保证移动后的位置是已初始化的
    // 当前环境下是安全的: 从当前位置 pos到 pos + len
    // 这段内存目前没有初始化,我们就是为了 resevere这段内存
    // 然后从 stream里读取, 它就是初始化的, 所以, 我们这么用是安全的
    unsafe { buf.advance_mut(len) };
    stream.read_exact(&mut buf[LEN_LEN..]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use crate::{CommandRequest, CommandResponse, FrameCoder, Value};

    use super::COMPRESSION_LIMIT;

    #[test]
    /// should work`工作```应该管用
    fn command_request_encode_decode_should_work() {
        let mut buf = BytesMut::new();
        let cmd = CommandRequest::new_hdel("t1", "k1");
        cmd.encode_frame(&mut buf).unwrap();

        assert_eq!(is_compressed(&buf), false);

        let cmd1 = CommandRequest::decode_frame(&mut buf).unwrap();
        assert_eq!(cmd, cmd1);
    }

    #[test]
    fn command_response_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let value: Vec<Value> = vec![1.into(), "hello".into(), b"data".into()];
        let res: CommandResponse = value.into();
        res.encode_frame(&mut buf).unwrap();

        assert_eq!(is_compressed(&buf), false);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();

        assert_eq!(res, res1);
    }

    #[test]
    fn command_response_compressed_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let value: Value = Bytes::from(vec![0u8; COMPRESSION_LIMIT + 1]).into();
        let res: CommandResponse = value.into();
        res.encode_frame(&mut buf).unwrap();

        assert_eq!(is_compressed(&buf), true);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();

        assert_eq!(res, res1);
    }

    /// 有没有设置最高位
    fn is_compressed(data: &[u8]) -> bool {
        if let &[v] = &data[..1] {
            v >> 7 == 1
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test_read_frame {
    use bytes::BytesMut;
    use tokio::io::AsyncRead;

    use crate::{read_frame, CommandRequest, FrameCoder};

    struct DummyStream {
        buf: BytesMut,
    }

    impl AsyncRead for DummyStream {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            // 查看 ReadBuf需要多大的数据
            let len = buf.capacity();

            // split出 len大小的数据
            // 返回 0..len的切片
            let data = self.get_mut().buf.split_to(len);

            // 拷贝给 ReadBuf
            buf.put_slice(&data);

            std::task::Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn read_frame_should_work() {
        let mut buf = BytesMut::new();
        let cmd = CommandRequest::new_hdel("t1", "k1");
        cmd.encode_frame(&mut buf).unwrap();
        let mut stream = DummyStream { buf };

        let mut buf = BytesMut::new();
        read_frame(&mut stream, &mut buf).await.unwrap();

        let cmd1 = CommandRequest::decode_frame(&mut buf).unwrap();
        assert_eq!(cmd, cmd1);
    }
}
