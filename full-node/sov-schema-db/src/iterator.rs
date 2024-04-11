use std::marker::PhantomData;

use crate::schema::{KeyCodec, Schema, ValueCodec};

pub(crate) enum IteratorDirection {
    Forward,
    Backward,
}

pub struct SchemaIterator<'a, S> {
    db_iter: rocksdb::DBRawIterator<'a>,
    direction: IteratorDirection,
    phantom: PhantomData<S>,
}

impl<'a, S> SchemaIterator<'a, S>
where
    S: Schema,
{
    pub(crate) fn new(db_iter: rocksdb::DBRawIterator<'a>, direction: IteratorDirection) -> Self {
        Self {
            db_iter,
            direction,
            phantom: PhantomData,
        }
    }

    pub fn seek_to_first(&mut self) {
        self.db_iter.seek_to_first()
    }

    pub fn seek_to_last(&mut self) {
        self.db_iter.seek_to_last()
    }

    fn next_impl(&mut self) -> anyhow::Result<Option<IteratorOutput<S::Key, S::Value>>> {
        if !self.db_iter.valid() {
            return Ok(None);
        }

        let raw_key = self.db_iter.key().expect("Failed to get key of iter");
        let raw_value = self.db_iter.value().expect("Failed to get value of iter");

        let key = <S::Key as KeyCodec<S>>::decode_key(raw_key)?;
        let value = <S::Value as ValueCodec<S>>::decode_value(raw_value)?;

        match self.direction {
            IteratorDirection::Forward => self.db_iter.next(),
            IteratorDirection::Backward => self.db_iter.prev(),
        }

        Ok(Some(IteratorOutput { key, value }))
    }
}

/// The output of [`SchemaIterator`]'s newxt_impl
pub struct IteratorOutput<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> IteratorOutput<K, V> {
    pub fn into_tupple(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<'a, S> Iterator for SchemaIterator<'a, S>
where
    S: Schema,
{
    type Item = anyhow::Result<IteratorOutput<S::Key, S::Value>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_impl().transpose()
    }
}
