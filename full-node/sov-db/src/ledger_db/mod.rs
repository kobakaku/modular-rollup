use std::path::Path;

use sov_schema_db::DB;

use crate::rocks_db_config::gen_rocks_db_options;

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
}
