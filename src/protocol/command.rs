use anyhow::{anyhow, Result};

use crate::protocol::frame::{
    Frame,
    OPCODE_GET,
    OPCODE_SET,
};

#[derive(Debug)]
pub enum Command {
    Set {
        key: String,
        value: Vec<u8>,
    },

    Get {
        key: String,
    },
}

impl Command {
    pub fn parse(
        input: &str
    ) -> Result<Self> {
        let segments: Vec<&str> =
            input.trim()
                .split_whitespace()
                .collect();

        if segments.is_empty() {
            return Err(anyhow!(
                "empty command"
            ));
        }

        match segments[0] {
            "SET" => {
                if segments.len() < 3 {
                    return Err(anyhow!(
                        "invalid set command"
                    ));
                }

                let key =
                    segments[1].to_string();

                let value =
                    segments[2..]
                        .join(" ")
                        .into_bytes();

                Ok(Command::Set {
                    key,
                    value,
                })
            }

            "GET" => {
                if segments.len() != 2 {
                    return Err(anyhow!(
                        "invalid get command"
                    ));
                }

                Ok(Command::Get {
                    key: segments[1]
                        .to_string(),
                })
            }

            _ => Err(anyhow!(
                "unsupported command"
            )),
        }
    }

    pub fn from_frame(
        frame: Frame
    ) -> Result<Self> {
        match frame.opcode {
            OPCODE_SET => {
                let payload =
                    String::from_utf8(
                        frame.payload
                    )?;

                let segments:
                    Vec<&str> =
                    payload
                        .splitn(2, ' ')
                        .collect();

                if segments.len() != 2 {
                    return Err(anyhow!(
                        "invalid set payload"
                    ));
                }

                Ok(Command::Set {
                    key: segments[0]
                        .to_string(),

                    value: segments[1]
                        .as_bytes()
                        .to_vec(),
                })
            }

            OPCODE_GET => {
                let key =
                    String::from_utf8(
                        frame.payload
                    )?;

                Ok(Command::Get {
                    key,
                })
            }

            _ => Err(anyhow!(
                "unsupported opcode"
            )),
        }
    }
}