mod iterator;
mod schema;

use iterator::IteratorDirection;
pub use iterator::SchemaIterator;
use rocksdb;
pub use schema::{KeyCodec, Schema, ValueCodec};
use std::path::Path;
use tracing::info;

pub struct DB {
    // name: &'static str,
    name: String,
    inner: rocksdb::DB,
}

impl DB {
    pub fn open<P: AsRef<Path>>(
        path: P,
        opts: &rocksdb::Options,
        name: &str,
    ) -> anyhow::Result<Self> {
        let inner = rocksdb::DB::open(opts, path)?;
        Self::log_creating_db(name);
        Ok(DB {
            name: name.to_string(),
            inner,
        })
    }

    fn log_creating_db(name: &str) {
        info!(rocksdb_name = name, "Opened RocksDB");
    }

    fn iter_with_direction<S: Schema>(
        &self,
        direction: IteratorDirection,
    ) -> anyhow::Result<SchemaIterator<S>> {
        Ok(SchemaIterator::new(self.inner.raw_iterator(), direction))
    }

    pub fn iter<S: Schema>(&self) -> anyhow::Result<SchemaIterator<S>> {
        self.iter_with_direction(IteratorDirection::Forward)
    }
}
