use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

use kv::{MemTable, ProstServerStream, Service, ServiceInner};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:7878";

    let service: Service = ServiceInner::new(MemTable::new()).into();
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening`启动监听 on 服务地址: http://{}", addr);
    loop {
        // 接受请求
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let stream = ProstServerStream::new(stream, service.clone());
        tokio::spawn(async move { stream.process().await });
    }

    Ok(())
}
