use jsonrpsee::server::RpcModule;
use tracing::info;

pub fn register_rpc() -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _| {
        info!("say_hello method called!");
        "Hello there!!"
    })?;
    Ok(module)
}
