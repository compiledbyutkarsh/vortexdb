use std::sync::Arc;

use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::engine::core::DatabaseEngine;

use crate::network::connection::ConnectionHandler;

pub struct DatabaseServer {
    listener: TcpListener,
    engine: Arc<DatabaseEngine>,
}

impl DatabaseServer {
    pub async fn new(
        address: &str
    ) -> Self {
        let listener =
            TcpListener::bind(address)
                .await
                .expect(
                    "tcp bind failure"
                );

        Self {
            listener,

            engine: Arc::new(
                DatabaseEngine::new()
            ),
        }
    }

    pub async fn run(
        &self
    ) -> Result<()> {
        info!(
            "vortexdb listening on 127.0.0.1:8080"
        );

        let wal_engine =
            Arc::clone(
                &self.engine
            );

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(
                    std::time::Duration::from_millis(
                        250
                    )
                );

            loop {
                interval.tick().await;

                wal_engine.flush_wal();
            }
        });

        let compact_engine =
            Arc::clone(
                &self.engine
            );

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(
                    std::time::Duration::from_secs(
                        30
                    )
                );

            loop {
                interval.tick().await;

                compact_engine
                    .compact_segments();
            }
        });

        loop {
            let (
                socket,
                address,
            ) = self
                .listener
                .accept()
                .await?;

            info!(
                "accepted peer {}",
                address
            );

            let engine =
                Arc::clone(
                    &self.engine
                );

            tokio::spawn(
                async move {
                    let mut handler =
                        ConnectionHandler::new(
                            socket,
                            engine,
                        );

                    if let Err(err) =
                        handler
                            .process()
                            .await
                    {
                        error!(
                            "connection terminated: {:?}",
                            err
                        );
                    }
                },
            );
        }
    }
}