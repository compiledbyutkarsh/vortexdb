use std::fs::{
    read_to_string,
    OpenOptions,
};
use std::io::Write;
use std::path::PathBuf;

pub struct Segment {
    path: PathBuf,
}

impl Segment {
    pub fn new(
        path: impl Into<PathBuf>
    ) -> Self {
        Self {
            path: path.into(),
        }
    }

    pub fn append(
        &self,
        key: &str,
        value: &[u8],
    ) {
        let mut file =
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
                .expect(
                    "segment open failure"
                );

        let value_string =
            String::from_utf8_lossy(
                value
            );

        let entry = format!(
            "{}={}\n",
            key,
            value_string
        );

        file.write_all(
            entry.as_bytes()
        )
        .expect(
            "segment write failure"
        );
    }

    pub fn compact(&self) {
        let contents =
            match read_to_string(
                &self.path
            ) {
                Ok(contents) => {
                    contents
                }

                Err(_) => {
                    return;
                }
            };

        let mut latest =
            std::collections::HashMap::<
                String,
                String,
            >::new();

        for line in contents.lines() {
            if let Some(
                (key, value)
            ) = line.split_once('=')
            {
                latest.insert(
                    key.to_string(),
                    value.to_string(),
                );
            }
        }

        let mut compacted =
            String::new();

        for (
            key,
            value,
        ) in latest
        {
            compacted.push_str(
                &format!(
                    "{}={}\n",
                    key,
                    value
                ),
            );
        }

        std::fs::write(
            &self.path,
            compacted,
        )
        .expect(
            "segment compaction failure"
        );
    }
}