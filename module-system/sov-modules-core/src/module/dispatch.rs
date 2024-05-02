use crate::CallResponse;

/// A trait that needs to be implemented for any call message
pub trait DispatchCall: Default {
    /// The context of the call
    type Context;

    /// The concreate type that will decode into the call message of the module.
    type Decodable: Send + Sync;

    /// Decodes serialized call message
    fn decode_call(serialized_message: &[u8]) -> anyhow::Result<Self::Decodable>;

    // Dispatches a call message to the appropriate module.
    fn dispatch_call(&self, message: Self::Decodable) -> anyhow::Result<CallResponse>;
}
