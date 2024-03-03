use sov_modules_api::{
    default_context::DefaultContext, default_signature::private_key::DefaultPrivateKey, Context,
    Module as _, WorkingSet,
};
use sov_nft::{address::CreatorAddress, call::CallMessage, NonFungibleToken};
use sov_prover_storage_manager::new_orphan_storage;
use sov_state::DefaultStorageSpec;

const PK1: [u8; 32] = [
    199, 23, 116, 41, 227, 173, 69, 178, 7, 24, 164, 151, 88, 149, 52, 187, 102, 167, 163, 248, 38,
    86, 207, 66, 87, 81, 56, 66, 211, 150, 208, 155,
];
const PK2: [u8; 32] = [
    92, 136, 187, 3, 235, 27, 9, 215, 232, 93, 24, 78, 85, 255, 234, 60, 152, 21, 139, 246, 151,
    129, 152, 227, 231, 204, 38, 84, 159, 129, 71, 143,
];

#[test]
fn nft_tests() {
    let sender_pk = DefaultPrivateKey::try_from(&PK1[..]).unwrap();
    let sequencer_pk = DefaultPrivateKey::try_from(&PK2[..]).unwrap();

    let sender_address = sender_pk.default_address();
    let sequencer_address = sequencer_pk.default_address();
    let collection_name = "Test Collection";
    let collection_uri = "http://foo.bar/test_collection";
    let new_collection_uri = "http://foo.bar/test_new_collection";

    let tmpdir = tempfile::tempdir().unwrap();
    let storage = new_orphan_storage::<DefaultStorageSpec>(tmpdir.path()).unwrap();
    let mut working_set = WorkingSet::new(storage);

    let context = DefaultContext::new(sender_address, sequencer_address, 1);
    let nft = NonFungibleToken::default();

    let create_collection_message = CallMessage::CreateCollection {
        collection_name: collection_name.to_string(),
        collection_uri: collection_uri.to_string(),
    };

    // Create Collection
    nft.call(create_collection_message, &context, &mut working_set)
        .expect("Creating Collection failed");

    let actual_collection = nft
        .get_collection(
            CreatorAddress::new(&sender_pk.default_address()),
            collection_name,
            &mut working_set,
        )
        .unwrap();

    assert_eq!(actual_collection.collection_name, collection_name);
    assert_eq!(actual_collection.creator.get_address().clone(), sender_address);
    assert!(!actual_collection.frozen);
    assert_eq!(actual_collection.supply, 0);
    assert_eq!(actual_collection.collection_uri, collection_uri);

    let update_collection_message = CallMessage::UpdateCollection{
        collection_name: collection_name.to_string(),
        collection_uri: new_collection_uri.to_string(),
    };

    // Update Collection
    nft.call(update_collection_message,&context, &mut working_set)
        .expect("Updating Collection failed");

    let actual_collection = nft
        .get_collection(
            CreatorAddress::new(&sender_pk.default_address()),
            collection_name,
            &mut working_set,
        )
        .unwrap();

    assert_eq!(actual_collection.collection_name, collection_name);
    assert_eq!(actual_collection.creator.get_address().clone(), sender_address);
    assert!(!actual_collection.frozen);
    assert_eq!(actual_collection.supply, 0);
    assert_eq!(actual_collection.collection_uri, new_collection_uri);

}
