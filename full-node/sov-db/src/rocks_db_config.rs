use rocksdb::Options;

pub fn gen_rocks_db_options(readonly: bool) -> Options {
    let mut opts = Options::default();
    if !readonly {
        opts.create_if_missing(true);
    }
    opts
}
