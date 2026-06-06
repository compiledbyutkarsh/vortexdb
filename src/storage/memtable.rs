use dashmap::DashMap;
use std::sync::Arc;

const TOTAL_SHARDS: usize = 64;

#[derive(Clone)]
pub struct MemTable {
    shards: Arc<Vec<DashMap<
        String,
        Vec<u8>,
    >>>,
}

impl MemTable {
    pub fn new() -> Self {
        let mut shards =
            Vec::with_capacity(
                TOTAL_SHARDS
            );

        for _ in 0..TOTAL_SHARDS {
            shards.push(
                DashMap::new()
            );
        }

        Self {
            shards: Arc::new(
                shards
            ),
        }
    }

    fn shard_for(
        &self,
        key: &str,
    ) -> usize {
        let mut hash: usize = 5381;

        for byte in key.bytes() {
            hash = ((hash << 5)
                + hash)
                + byte as usize;
        }

        hash % TOTAL_SHARDS
    }

    pub fn put(
        &self,
        key: String,
        value: Vec<u8>,
    ) {
        let shard =
            self.shard_for(&key);

        self.shards[shard]
            .insert(key, value);
    }

    pub fn get(
        &self,
        key: &str,
    ) -> Option<Vec<u8>> {
        let shard =
            self.shard_for(key);

        self.shards[shard]
            .get(key)
            .map(|entry| {
                entry.clone()
            })
    }
}