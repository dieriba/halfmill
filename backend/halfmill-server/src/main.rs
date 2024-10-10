use anyhow::Result;
use halfmill_common::{config::config, Database};
use std::env;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn futures() -> Result<()> {
    let curr_dir = env::current_dir()?;
    let env_path = curr_dir.parent().unwrap().join(".env");
    dotenvy::from_path(env_path)?;

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
