mod call;
mod dispatch;
mod spec;

pub use call::*;
pub use dispatch::*;
pub use spec::*;

/// The core trait implemented by all modules. This trait defined how a module is initialized at genesis,
/// and how it handles user transactions (if applicable).
pub trait Module {
    type Context;

    /// Module defined argument to the call message.
    type CallMessage;

    /// Call allows interaction with the module and invokes state changes.
    fn call(&mut self, message: Self::CallMessage) -> anyhow::Result<CallResponse>;
}
