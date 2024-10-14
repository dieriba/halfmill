use anyhow::Result;
use halfmill_common::{config::config, Database};
use std::env;

async fn futures() -> Result<()> {
    tracing_subscriber::fmt::init();
    let curr_dir = env::current_dir()?;
    let env_path = curr_dir.parent().unwrap().join(".env");
    dotenvy::from_path(env_path)?;

    let _ = config();
    let database = Database::connect().await?;
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    halfmill_api::start_server(database, rx).await?;
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
