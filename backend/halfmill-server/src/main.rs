use anyhow::Result;
use halfmill_common::config::config;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn futures() -> Result<()> {
    dotenv::dotenv().ok();
    let config = config();

    Ok(())
}

fn main() -> Result<()> {
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(32)
        .build()
        .unwrap();
    let _ = tokio_runtime.block_on(futures());
    Ok(())
}
