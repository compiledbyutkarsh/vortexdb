use std::sync::Arc;

use anyhow::Result;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::engine::core::DatabaseEngine;
use crate::protocol::command::Command;
use crate::protocol::frame::{
    Frame,
    OPCODE_GET,
    
};

pub struct ConnectionHandler {
    socket: TcpStream,
    engine: Arc<DatabaseEngine>,
}

impl ConnectionHandler {
    pub fn new(
        socket: TcpStream,
        engine: Arc<DatabaseEngine>,
    ) -> Self {
        Self {
            socket,
            engine,
        }
    }

    pub async fn process(
        &mut self
    ) -> Result<()> {
        let mut read_buffer =
            BytesMut::with_capacity(8192);

        loop {
            let bytes_read = self
                .socket
                .read_buf(&mut read_buffer)
                .await?;

            if bytes_read == 0 {
                return Ok(());
            }

            while let Some(frame) =
                Frame::deserialize(
                    &mut read_buffer
                )?
            {
                let command =
                    Command::from_frame(frame)?;

                let response =
                    self.engine.execute(command);

                let response_frame = Frame {
                    opcode: OPCODE_GET,
                    payload: response.into_bytes(),
                };

                let serialized =
                    response_frame.serialize();

                self.socket
                    .write_all(&serialized)
                    .await?;
            }
        }
    }
}