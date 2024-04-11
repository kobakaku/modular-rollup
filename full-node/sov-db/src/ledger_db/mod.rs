use std::path::Path;

use sov_schema_db::{KeyCodec, Schema, SchemaIterator, ValueCodec, DB};

use crate::{
    rocks_db_config::gen_rocks_db_options,
    schema::table::{MyKey, MyTable, MyValue},
};

const LEDGER_DB_NAME: &'static str = "ledger-db";

pub struct LedgerDB {
    db: DB,
}

impl LedgerDB {
    pub fn open_ledger_db<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let opts = gen_rocks_db_options(false);
        let path = path.as_ref().join(LEDGER_DB_NAME);
        let db = DB::open(path, &opts, LEDGER_DB_NAME)?;
        Ok(Self { db })
    }

    /// Get the most recent commited slot, if any
    pub fn get_head_slot(&self) -> anyhow::Result<Option<(MyKey, MyValue)>> {
        let mut iter = self.db.iter::<MyTable>()?;
        iter.seek_to_last();

        match iter.next() {
            Some(Ok(data)) => Ok(Some(data.into_tupple())),
            Some(Err(e)) => Err(e),
            _ => Ok(None),
        }
    }
}
