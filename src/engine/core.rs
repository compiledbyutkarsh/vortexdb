use std::fs;
use std::path::Path;
use std::sync::Arc;

use tokio::runtime::Handle;

use crate::protocol::command::Command;
use crate::protocol::frame::{
    Frame,
    OPCODE_SET,
};

use crate::replication::replica::ReplicaNode;

use crate::storage::memtable::MemTable;
use crate::storage::segment::Segment;

use crate::wal::log::WriteAheadLog;

#[derive(Clone)]
pub struct DatabaseEngine {
    storage: MemTable,
    wal: WriteAheadLog,
    segment: Arc<Segment>,
    replicas: Arc<Vec<ReplicaNode>>,
}

impl DatabaseEngine {
    pub fn new() -> Self {
        let storage = MemTable::new();

        let replicas = vec![
            ReplicaNode::new(
                "127.0.0.1:9001"
                    .to_string(),
            ),
        ];

        let engine = Self {
            storage: storage.clone(),

            wal: WriteAheadLog::new(
                "vortex.wal"
            ),

            segment: Arc::new(
                Segment::new(
                    "vortex.segment"
                )
            ),

            replicas: Arc::new(
                replicas
            ),
        };

        engine.recover();

        engine
    }

    fn recover(&self) {
        let wal_path =
            Path::new("vortex.wal");

        if !wal_path.exists() {
            return;
        }

        let contents =
            fs::read_to_string(
                wal_path
            )
            .expect(
                "wal recovery read failure"
            );

        for line in contents.lines() {
            let command =
                Command::parse(line);

            if let Ok(
                Command::Set {
                    key,
                    value,
                }
            ) = command
            {
                self.storage.put(
                    key,
                    value,
                );
            }
        }
    }

    pub fn flush_wal(&self) {
        self.wal.flush();
    }

    pub fn compact_segments(
        &self
    ) {
        self.segment.compact();
    }

    pub fn execute(
        &self,
        command: Command,
    ) -> String {
        match command {
            Command::Set {
                key,
                value,
            } => {
                let value_string =
                    String::from_utf8_lossy(
                        &value
                    );

                let wal_entry =
                    format!(
                        "SET {} {}",
                        key,
                        value_string
                    );

                self.wal.append(
                    &wal_entry
                );

                self.segment.append(
                    &key,
                    &value,
                );

                self.storage.put(
                    key.clone(),
                    value.clone(),
                );

                let payload =
                    format!(
                        "{} {}",
                        key,
                        value_string
                    )
                    .into_bytes();

                let frame = Frame {
                    opcode:
                        OPCODE_SET,

                    payload,
                };

                let serialized =
                    frame.serialize();

                for replica in
                    self.replicas.iter()
                {
                    let replica =
                        replica.clone();

                    let payload =
                        serialized.clone();

                    Handle::current()
                        .spawn(
                            async move {
                                let _ =
                                    replica
                                        .replicate(
                                            &payload
                                        )
                                        .await;
                            },
                        );
                }

                "OK\n".to_string()
            }

            Command::Get { key } => {
                match self.storage.get(&key)
                {
                    Some(value) => {
                        format!(
                            "{}\n",
                            String::from_utf8_lossy(
                                &value
                            )
                        )
                    }

                    None => {
                        "NULL\n"
                            .to_string()
                    }
                }
            }
        }
    }
}