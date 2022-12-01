// extern crate alloc;
use std::collections::BTreeMap;
// Outlining aspects of the Casper test support crate to include.
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_ACCOUNT_PUBLIC_KEY, DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;

use crate::utility::constants::{ACCEPTED_TOKENS_ARG, FEE_WALLET_ARG};
// Custom Casper types that will be used within this test.
use casper_types::{
    runtime_args, CLType, ContractHash, Key, PublicKey, RuntimeArgs, SecretKey, U512,
};

// Calling the contract deploy.
const MARKETPLACE: &str = "marketplace.wasm";
const CEP78: &str = "cep78.wasm";

// Setting entry points constants
const ENTRY_POINT_SET_ACCEPTED_TOKEN: &str = "set_accepted_token";

// Setting runtine arguments constants
const TOKEN_ARG: &str = "token_arg";
const FEE_ARG: &str = "fee_arg";

//WasmTestBuilder<InMemoryGlobalState>, ContractHash

fn should_deploy() -> (
    WasmTestBuilder<InMemoryGlobalState>,
    ContractHash,
    ContractHash,
) {
    let default_account = *DEFAULT_ACCOUNT_ADDR;
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

    let mut accepted_tokens: BTreeMap<String, u32> = BTreeMap::new();
    let null_contract_hash = ContractHash::new([0u8; 32]);
    let price = 1000u32;
    accepted_tokens.insert(null_contract_hash.to_formatted_string(), price);

    let contract_creation_request = ExecuteRequestBuilder::standard(
        default_account,
        MARKETPLACE,
        runtime_args! {
            FEE_WALLET_ARG => Key::from(default_account),
            ACCEPTED_TOKENS_ARG => accepted_tokens,
        },
    )
    .build();

    // Execute this request.
    builder
        .exec(contract_creation_request)
        // Expects the deploy to succeed or crashes the test.
        .expect_success()
        // Process the execution result of the previous execute request.
        .commit();

    let contract_creation_request = ExecuteRequestBuilder::standard(
        default_account,
        CEP78,
        runtime_args! {
            FEE_WALLET_ARG => Key::from(default_account),
            ACCEPTED_TOKENS_ARG => accepted_tokens,
        },
    )
    .build();

    // Execute this request.
    builder
        .exec(contract_creation_request)
        // Expects the deploy to succeed or crashes the test.
        .expect_success()
        // Process the execution result of the previous execute request.
        .commit();

    // Extracts the contract hash from the named keys of the account in question, the default
    // genesis address.
    let marketplace_contract_hash = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get("marketplace_contract_hash")
        .expect("must have contract hash key as part of contract creation")
        .into_hash()
        .map(|hash| ContractHash::new(hash))
        .expect("must get contract hash");
    (builder, contract_hash)
}

#[test]
fn should_add_accepted_token() {
    let (mut builder, contract_hash) = should_deploy();
    let default_account = *DEFAULT_ACCOUNT_ADDR;
    let session_code_request = ExecuteRequestBuilder::contract_call_by_hash(
        // Again, using the default account hash included with genesis.
        default_account,
        // Telling the execution request builder to load up an instance of a deploy built from
        // donate.wasm.
        contract_hash,
        // Including the necessary runtime arguments.
        ENTRY_POINT_SET_ACCEPTED_TOKEN,
        runtime_args! {
            TOKEN_ARG => ContractHash::new([0u8; 32]).to_formatted_string(),
            FEE_ARG => 10u32,
        },
    )
    .build();

    // Execute this request.
    builder.exec(session_code_request).expect_success().commit();
}

// #[test]
// fn should_
