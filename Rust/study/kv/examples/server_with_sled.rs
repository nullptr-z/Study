use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse, Service, ServiceInner, SledDb};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 开启日志
    tracing_subscriber::fmt::init();

    // 创建持久化存储数据库
    let service: Service<SledDb> = ServiceInner::new(SledDb::new("/tmp/kvserver"))
        // 服务器发送之前, 对 CommandResponse调整
        .fn_before_send(|res| match res.message.as_ref() {
            "" => res.message = "更改, 原始数据是空的".into(),
            s => res.message = format!("更改为: {}", s),
        })
        .into();
    let addr = "127.0.0.1:7878";
    // 创建服务
    let listener = TcpListener::bind(addr).await?;
    info!("Start`开始 listening`听 on`动词;在..之上,持续 {} ", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client http://{:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            // 从请求服务列队中取出,求情执行的命令
            while let Some(Ok(cmd)) = stream.next().await {
                info!("Got a new command: {:?}", cmd);
                // 命令执行结果发送给客户端
                let res = svc.execute(cmd);
                stream.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
