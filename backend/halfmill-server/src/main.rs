use anyhow::Result;
use halfmill_common::{
    config::{config, Config},
    Database, SUPPORTED_SCRIPTS,
};
use std::{
    env,
    path::{Path, PathBuf},
};
use tokio::fs::DirBuilder;

pub async fn create_directory(directory_path: &str, path: &Path) {
    let directory_name = path.file_name().unwrap().to_str().unwrap();
    let parent_directory = path.parent().unwrap().to_str().unwrap();
    DirBuilder::new()
        .recursive(true)
        .create(directory_path)
        .await
        .unwrap_or_else(|_| panic!("could not create the following: {}", directory_name));
    tracing::info!(
        "directory named {} successfully created within {}",
        directory_name,
        parent_directory
    );
}

async fn create_all_required_directory(server_directory: &str) {
    let mut path = PathBuf::from(server_directory);
    create_directory(server_directory, &path).await;
    for directory in SUPPORTED_SCRIPTS {
        path.push(directory);
        create_directory(path.as_path().to_str().unwrap(), &path).await;
        path.pop();
    }
}

async fn futures() -> Result<()> {
    tracing_subscriber::fmt::init();
    let curr_dir = env::current_dir()?;
    let env_path = curr_dir.parent().unwrap().join(".env");
    dotenvy::from_path(env_path)?;
    let Config {
        server_directory, ..
    } = config();
    create_all_required_directory(server_directory).await;
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
