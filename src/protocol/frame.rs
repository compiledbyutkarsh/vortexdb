use anyhow::{anyhow, Result};
use bytes::{Buf, BufMut, BytesMut};

pub const OPCODE_SET: u8 = 1;
pub const OPCODE_GET: u8 = 2;

pub struct Frame {
    pub opcode: u8,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn serialize(&self) -> BytesMut {
        let payload_length = self.payload.len();

        let mut buffer =
            BytesMut::with_capacity(5 + payload_length);

        buffer.put_u8(self.opcode);

        buffer.put_u32(
            payload_length as u32
        );

        buffer.extend_from_slice(&self.payload);

        buffer
    }

    pub fn deserialize(
        buffer: &mut BytesMut
    ) -> Result<Option<Self>> {
        if buffer.len() < 5 {
            return Ok(None);
        }

        let opcode = buffer[0];

        let payload_length =
            u32::from_be_bytes([
                buffer[1],
                buffer[2],
                buffer[3],
                buffer[4],
            ]) as usize;

        if buffer.len() < 5 + payload_length {
            return Ok(None);
        }

        buffer.advance(5);

        let payload = buffer
            .split_to(payload_length)
            .to_vec();

        match opcode {
            OPCODE_SET | OPCODE_GET => {
                Ok(Some(Self {
                    opcode,
                    payload,
                }))
            }

            _ => Err(anyhow!(
                "invalid opcode"
            )),
        }
    }
}