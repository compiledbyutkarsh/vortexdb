#[path = "../protocol/mod.rs"]
mod protocol;

use anyhow::Result;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use protocol::frame::Frame;

#[tokio::main]
async fn main() -> Result<()> {
    let listener =
        TcpListener::bind(
            "127.0.0.1:9001"
        )
        .await?;

    println!(
        "replica listening on 127.0.0.1:9001"
    );

    loop {
        let (
            mut socket,
            address,
        ) = listener.accept().await?;

        println!(
            "replica accepted {}",
            address
        );

        tokio::spawn(async move {
            let mut buffer =
                BytesMut::with_capacity(
                    8192
                );

            loop {
                let read =
                    socket
                        .read_buf(
                            &mut buffer
                        )
                        .await;

                let bytes_read =
                    match read {
                        Ok(value) => value,

                        Err(_) => {
                            return;
                        }
                    };

                if bytes_read == 0 {
                    return;
                }

                loop {
                    let frame =
                        Frame::deserialize(
                            &mut buffer
                        );

                    match frame {
                        Ok(Some(frame)) => {
                            println!(
                                "replicated frame opcode={} bytes={}",
                                frame.opcode,
                                frame.payload.len()
                            );
                        }

                        Ok(None) => {
                            break;
                        }

                        Err(_) => {
                            return;
                        }
                    }
                }
            }
        });
    }
}