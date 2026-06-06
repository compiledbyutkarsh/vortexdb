use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Barrier;

const OPCODE_SET: u8 = 1;

const TOTAL_CLIENTS: usize = 100;
const REQUESTS_PER_CLIENT: usize = 1000;

const PIPELINE_BATCH: usize = 32;

#[tokio::main]
async fn main() -> Result<()> {
    let barrier =
        Arc::new(
            Barrier::new(
                TOTAL_CLIENTS
            )
        );

    let started_at =
        Instant::now();

    let mut handles =
        Vec::new();

    for client_id in 0..TOTAL_CLIENTS {
        let barrier =
            Arc::clone(
                &barrier
            );

        let handle =
            tokio::spawn(
                async move {
                    run_client(
                        client_id,
                        barrier,
                    )
                    .await
                },
            );

        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    let elapsed =
        started_at.elapsed();

    let total_requests =
        TOTAL_CLIENTS
            * REQUESTS_PER_CLIENT;

    let throughput =
        total_requests as f64
            / elapsed.as_secs_f64();

    println!(
        "completed_requests={}",
        total_requests
    );

    println!(
        "elapsed={:.2?}",
        elapsed
    );

    println!(
        "throughput={:.2} req/sec",
        throughput
    );

    Ok(())
}

async fn run_client(
    client_id: usize,
    barrier: Arc<Barrier>,
) -> Result<()> {
    let mut socket =
        TcpStream::connect(
            "127.0.0.1:8080"
        )
        .await?;

    barrier.wait().await;

    let mut pending = 0;

    for request_id in
        0..REQUESTS_PER_CLIENT
    {
        let key = format!(
            "client:{}:{}",
            client_id,
            request_id
        );

        let value = format!(
            "payload:{}",
            request_id
        );

        send_set(
            &mut socket,
            &key,
            &value,
        )
        .await?;

        pending += 1;

        if pending
            >= PIPELINE_BATCH
        {
            for _ in 0..pending {
                read_response(
                    &mut socket
                )
                .await?;
            }

            pending = 0;
        }
    }

    for _ in 0..pending {
        read_response(
            &mut socket
        )
        .await?;
    }

    Ok(())
}

async fn send_set(
    socket: &mut TcpStream,
    key: &str,
    value: &str,
) -> Result<()> {
    let payload =
        format!(
            "{} {}",
            key,
            value
        );

    let mut frame =
        BytesMut::with_capacity(
            5 + payload.len()
        );

    frame.put_u8(
        OPCODE_SET
    );

    frame.put_u32(
        payload.len() as u32
    );

    frame.extend_from_slice(
        payload.as_bytes()
    );

    socket
        .write_all(&frame)
        .await?;

    Ok(())
}

async fn read_response(
    socket: &mut TcpStream
) -> Result<()> {
    let mut header =
        [0u8; 5];

    socket
        .read_exact(
            &mut header
        )
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
        .read_exact(
            &mut payload
        )
        .await?;

    Ok(())
}