use anyhow::Result;
use halfmill_common::{config::config, Database};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn futures() -> Result<()> {
    dotenv::dotenv().ok();
    let _ = config();
    let database = Database::new().await?;
    Ok(())
}

fn main() -> Result<()> {
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(32)
        .build()
        .unwrap();
    tokio_runtime.block_on(futures())?;
    Ok(())
}
