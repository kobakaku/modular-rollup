use helpers::*;
use sov_bank::{get_token_address, Bank, CallMessage};
use sov_modules_api::utils::generate_address;
use sov_modules_api::{Context, Event, Module, WorkingSet};
use sov_prover_storage_manager::new_orphan_storage;
use sov_state::DefaultStorageSpec;

mod helpers;

#[test]
fn initial_and_deployed_token() {
    let sender_address = generate_address::<C>("sender");
    let sequencer_address = generate_address::<C>("sequencer");
    let minter_address = generate_address::<C>("minter");

    // let bank_config = create_bank_config_with_token(1, 100);
    let tmpdir = tempfile::tempdir().unwrap();
    let storage = new_orphan_storage::<DefaultStorageSpec>(tmpdir.path()).unwrap();
    let mut working_set = WorkingSet::new(storage);
    // bank.genesis(&bank_config, &mut working_set).unwrap();

    let context = C::new(sender_address, sequencer_address, 1);
    let bank = Bank::default();

    let initial_balance = 500;
    let token_name = "Token1".to_owned();
    let salt = 1;
    let token_address = get_token_address::<C>(&token_name, sender_address.as_ref(), salt);

    let create_token_message = CallMessage::CreateToken::<C> {
        salt,
        token_name: token_name.clone(),
        initial_balance,
        minter_address,
    };

    bank.call(create_token_message, &context, &mut working_set)
        .expect("Failed to create token");

    assert_eq!(
        working_set.events()[0],
        Event::new(
            "Create Token",
            &format!("A token with token_address {token_address} was created")
        )
    );

    let sender_balance = bank.get_balance_of(&sender_address, &token_address, &mut working_set);
    assert!(sender_balance.is_none());

    let observed_token_name = bank
        .get_token_name(&token_address, &mut working_set)
        .expect("Token is missing its name");
    assert_eq!(&token_name, &observed_token_name);

    let minter_balance = bank.get_balance_of(&minter_address, &token_address, &mut working_set);
    assert_eq!(Some(initial_balance), minter_balance);

    let total_supply = bank
        .get_total_supply_of(&token_address, &mut working_set)
        .unwrap();
    assert_eq!(initial_balance, total_supply);
}

// balance_and_addresssを複数いれることができるようになったらテストする
// #[test]
// /// Currently integer overflow happens on bank genesis
// fn overflow_max_supply() {
//     let sender_address = generate_address::<C>("sender");
//     let sequencer_address = generate_address::<C>("sequencer");
//     let minter_address = generate_address::<C>("minter");

//     let tmpdir = tempfile::tempdir().unwrap();
//     let storage = new_orphan_storage::<DefaultStorageSpec>(tmpdir.path()).unwrap();
//     let mut working_set = WorkingSet::new(storage);

//     let context = C::new(sender_address, sequencer_address, 1);
//     let bank = Bank::default();

//     #[deny(arithmetic_overflow)]
//     let initial_balance = u64::MAX;
//     let token_name = "Token1".to_owned();
//     let salt = 2;

//     let create_token_message = CallMessage::CreateToken::<C> {
//         salt,
//         token_name: token_name.clone(),
//         initial_balance: initial_balance,
//         minter_address,
//     };

//     let result = bank.call(create_token_message, &context, &mut working_set);

//     assert!(result.is_err());

//     assert_eq!("Total supply overflow", result.unwrap_err().to_string());
// }
