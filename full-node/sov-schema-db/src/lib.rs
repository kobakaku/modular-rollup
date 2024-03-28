use rocksdb;
use tracing::info;

pub struct DB {
    // name: &'static str,
    name: String,
    inner: rocksdb::DB,
}

impl DB {
    pub fn open(opts: &rocksdb::Options, path: &str, name: &str) -> anyhow::Result<Self> {
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
}
