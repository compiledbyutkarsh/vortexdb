use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;

use parking_lot::Mutex;

#[derive(Clone)]
pub struct WriteAheadLog {
    writer: Arc<Mutex<BufWriter<File>>>,
}

impl WriteAheadLog {
    pub fn new(
        path: impl AsRef<Path>
    ) -> Self {
        let file =
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect(
                    "wal open failure"
                );

        Self {
            writer: Arc::new(
                Mutex::new(
                    BufWriter::with_capacity(
                        1024 * 1024,
                        file,
                    )
                )
            ),
        }
    }

    pub fn append(
        &self,
        entry: &str,
    ) {
        let mut writer =
            self.writer.lock();

        writer
            .write_all(
                entry.as_bytes()
            )
            .expect(
                "wal write failure"
            );

        writer
            .write_all(b"\n")
            .expect(
                "wal newline failure"
            );
    }

    pub fn flush(&self) {
        let mut writer =
            self.writer.lock();

        writer
            .flush()
            .expect(
                "wal flush failure"
            );
    }
}