mod network;
mod storage;
mod protocol;
mod engine;
mod wal;
mod replication;

use anyhow::Result;
use network::server::DatabaseServer;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("booting vortexdb runtime");

    let server = DatabaseServer::new("127.0.0.1:8080").await;

    match server.run().await {
        Ok(_) => info!("runtime shutdown complete"),
        Err(err) => error!("runtime failure: {:?}", err),
    }

    Ok(())
}