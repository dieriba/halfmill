use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn halfmill_main() {

}

fn main() -> Result<()> {
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(32)
        .build()
        .unwrap();
    tokio_runtime.block_on(halfmill_main());
    Ok(())
}
