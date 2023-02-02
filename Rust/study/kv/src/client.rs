use anyhow::Result;
use tokio::net::TcpStream;
use tracing::info;

use kv::{CommandRequest, ProstClientStream};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 服务器地址
    let addr = "127.0.0.1:7878";
    // 链接服务器
    let stream = TcpStream::connect(addr).await?;

    let mut client = ProstClientStream::new(stream);

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "wrold".to_string().into());

    // 发送命令
    let data = client.execute(cmd).await;

    info!("Got`得到 response {:?}", data);

    Ok(())
}
