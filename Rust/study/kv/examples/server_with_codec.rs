use anyhow::Result;
use futures::{SinkExt, StreamExt};
use kv::{CommandRequest, MemTable, Service, ServiceInner};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

/// tokio-util 库
/// 已经帮我们处理了和 frame 相关的封包解包的主要需求，
/// LengthDelimitedCodec:
/// LinesDelimited（处理 \r\n 分隔符）
/// 和 LengthDelimited（处理长度分隔符

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); // 日志监听

    let service: Service = ServiceInner::new(MemTable::new()).into();
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).await?;

    info!("启动服务: {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("来自客户端: {:?} 的请求", addr);
        let svc = service.clone(); // 注意所有权机制,在循环体每次结尾处生命周期终止

        tokio::spawn(async move {
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                let cmd = CommandRequest::decode(&buf[..]).unwrap();
                info!("Got`得到 a`一个 new`新 command`命令");
                let res = svc.execute(cmd);
                buf.clear();
                res.encode(&mut buf).unwrap();
                stream.send(buf.freeze()).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
