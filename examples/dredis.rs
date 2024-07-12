use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis 监听：{}", addr);
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("监听连接：{}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                info!("处理错误：{}", e);
            }
        });
    }
    Ok(())
}
