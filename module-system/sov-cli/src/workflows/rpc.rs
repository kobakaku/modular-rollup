/// Query the current state of the rollup and send transactions
#[derive(clap::Subcommand)]
pub enum RpcWorkFlows {
    /// Sign all transactions from the current batch and submit them to the rollup.
    /// Nonces will be set automatically.
    SubmitBatch,
}

impl RpcWorkFlows {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            RpcWorkFlows::SubmitBatch => {
                todo!()
            }
        }
    }
}
