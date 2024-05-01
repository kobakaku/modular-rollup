/// A trait that needs to be implemented for any call message
pub trait DispatchCall {
    /// The context of the call
    type Context;

    /// The concreate type that will decode into the call message of the module.
    type Decodable: Send + Sync;
}
