use sov_schema_db::{KeyCodec, Schema, ValueCodec};

pub struct MyTable {}
impl Schema for MyTable {
    type Key = MyKey;

    type Value = MyValue;
}

pub struct MyKey {}
pub struct MyValue {}

impl KeyCodec<MyTable> for MyKey {
    fn encode_key(&self) -> anyhow::Result<Vec<u8>> {
        todo!()
    }

    fn decode_key(data: &[u8]) -> anyhow::Result<Self> {
        todo!()
    }
}

impl ValueCodec<MyTable> for MyValue {
    fn encode_value(&self) -> anyhow::Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> anyhow::Result<Self> {
        todo!()
    }
}
