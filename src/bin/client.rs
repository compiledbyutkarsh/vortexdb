use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const OPCODE_SET: u8 = 1;
const OPCODE_GET: u8 = 2;

#[tokio::main]
async fn main() -> Result<()> {
    let mut socket =
        TcpStream::connect(
            "127.0.0.1:8080"
        )
        .await?;

    send_set(
        &mut socket,
        "language",
        "rust"
    )
    .await?;

    read_response(&mut socket)
        .await?;

    send_get(
        &mut socket,
        "language"
    )
    .await?;

    read_response(&mut socket)
        .await?;

    Ok(())
}

async fn send_set(
    socket: &mut TcpStream,
    key: &str,
    value: &str,
) -> Result<()> {
    let payload =
        format!("{} {}", key, value);

    let mut frame =
        BytesMut::with_capacity(
            5 + payload.len()
        );

    frame.put_u8(OPCODE_SET);

    frame.put_u32(
        payload.len() as u32
    );

    frame.extend_from_slice(
        payload.as_bytes()
    );

    socket.write_all(&frame).await?;

    Ok(())
}

async fn send_get(
    socket: &mut TcpStream,
    key: &str,
) -> Result<()> {
    let mut frame =
        BytesMut::with_capacity(
            5 + key.len()
        );

    frame.put_u8(OPCODE_GET);

    frame.put_u32(
        key.len() as u32
    );

    frame.extend_from_slice(
        key.as_bytes()
    );

    socket.write_all(&frame).await?;

    Ok(())
}

async fn read_response(
    socket: &mut TcpStream
) -> Result<()> {
    let mut header = [0u8; 5];

    socket
        .read_exact(&mut header)
        .await?;

    let payload_length =
        u32::from_be_bytes([
            header[1],
            header[2],
            header[3],
            header[4],
        ]) as usize;

    let mut payload =
        vec![0u8; payload_length];

    socket
        .read_exact(&mut payload)
        .await?;

    println!(
        "{}",
        String::from_utf8_lossy(
            &payload
        )
    );

    Ok(())
}