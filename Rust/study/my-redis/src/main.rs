use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let ip_addr = "127.0.0.1:6379";
    let mut client = client::connect(ip_addr).await?;

    client.set("Hello", "World".into()).await?;

    let result = client.get("Hello").await?;

    println!("得到`got value 从中`from the server; result={:?}", result);

    Ok(())
}
