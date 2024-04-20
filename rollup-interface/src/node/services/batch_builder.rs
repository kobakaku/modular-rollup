/// This trait is responsible for managing mempool and building batches.
pub trait BatchBuilder: Send + Sync + 'static {
    /// Accept a new transaction.
    /// Can return error if transation is invalid or mempool is full.
    fn accept_tx(&mut self, tx: Vec<u8>) -> anyhow::Result<()>;

    /// Build a new batch out of transactions in mempool.
    /// Logic of which transactions and how many of them is included in batch is up to implementation.
    fn get_next_blob(&mut self) -> anyhow::Result<Vec<Vec<u8>>>;
}
