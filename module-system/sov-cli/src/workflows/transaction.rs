/// Import, Clean and List the transactions
#[derive(clap::Subcommand)]
pub enum TransactionWorkFlows {
    /// Import a transaction
    Import,
    /// Delete the currenct batch of transactions
    Clean,
    /// List the currenct batch of transactions
    List,
}

impl TransactionWorkFlows {
    /// Run the transaction workflow
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            TransactionWorkFlows::Import => {
                todo!()
            }
            TransactionWorkFlows::Clean => {
                todo!()
            }
            TransactionWorkFlows::List => {
                todo!()
            }
        }
    }
}
