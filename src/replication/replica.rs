use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Clone)]
pub struct ReplicaNode {
    address: String,
}

impl ReplicaNode {
    pub fn new(
        address: String
    ) -> Self {
        Self { address }
    }

    pub async fn replicate(
        &self,
        payload: &[u8],
    ) -> Result<()> {
        let mut stream =
            TcpStream::connect(
                &self.address
            )
            .await?;

        stream
            .write_all(payload)
            .await?;

        Ok(())
    }
}