pub trait Schema: Sized {
    type Key: KeyCodec<Self>;
    type Value: ValueCodec<Self>;
}

pub trait KeyCodec<S: Schema>: Sized {
    fn encode_key(&self) -> anyhow::Result<Vec<u8>>;
    fn decode_key(data: &[u8]) -> anyhow::Result<Self>;
}

pub trait ValueCodec<S: Schema>: Sized {
    fn encode_value(&self) -> anyhow::Result<Vec<u8>>;
    fn decode_value(data: &[u8]) -> anyhow::Result<Self>;
}
