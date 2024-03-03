use sov_modules_api::Context;

use crate::address::{CollectionAddress, OwnerAddress};

pub type TokenId = u64;

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
pub struct NftIdentifier<C: Context>(pub TokenId, pub CollectionAddress<C>);

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
pub struct Nft<C: Context> {
    _token_id: TokenId,
    _collection_address: CollectionAddress<C>,
    _owner: OwnerAddress<C>,
    _frozen: bool,
    _token_uri: String,
}
