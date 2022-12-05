// extern crate alloc;
use std::{collections::BTreeMap, default};
// Outlining aspects of the Casper test support crate to include.
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;

use crate::utility::{
    constants::{ACCEPTED_TOKENS_ARG, FEE_WALLET_ARG},
    helpers::{get_contract_hash, query_stored_value},
};
// Custom Casper types that will be used within this test.
use self::meta::red_dragon;
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U256};
use serde::{Deserialize, Serialize};

// Calling the contract deploy.
const MARKETPLACE: &str = "marketplace.wasm";
const CEP78: &str = "cep78.wasm";
const ERC20: &str = "erc20.wasm";

// Setting entry points constants
const ENTRY_POINT_SET_ACCEPTED_TOKEN: &str = "set_accepted_token";
const ENTRY_POINT_ADD_LISTING: &str = "add_listing";

// Setting runtine arguments constants
const TOKEN_ARG: &str = "token_arg";
const FEE_ARG: &str = "fee_arg";
const COLLECTION_ARG: &str = "collection_arg";
const TOKEN_ID_ARG: &str = "token_id_arg";
const PAY_TOKEN_ARG: &str = "pay_token_arg";
const PRICE_ARG: &str = "price_arg";

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataStruct {
    name: String,
    symbol: String,
    token_uri: String,
}
mod meta {
    use super::MetadataStruct;

    pub fn red_dragon() -> String {
        let metadata = MetadataStruct {
            name: String::from("Bit Hotel"),
            symbol: String::from("BHOTEL"),
            token_uri: String::from("https://google.com"),
        };
        let metadata_res = serde_json::to_string(&metadata);
        metadata_res.unwrap()
    }
}

fn should_deploy() -> (
    WasmTestBuilder<InMemoryGlobalState>,
    ContractHash,
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
    builder
        .exec(contract_creation_request)
        .expect_success()
        .commit();

    let contract_creation_request = ExecuteRequestBuilder::standard(
        default_account,
        CEP78,
        runtime_args! {
            "collection_name" => "Bit Hotel",
            "collection_symbol" => "BHOTEL",
            "total_token_supply" => 10u64,
            "ownership_mode" => 2u8,
            "nft_kind" => 0u8,
            "json_schema" => "nft-schema",
            "allow_minting" => true,
            "nft_metadata_kind" => 1u8,
            "identifier_mode" => 0u8,
            "metadata_mutability" => 1u8,
            // "nft_holder_mode" => 2u8, // FIXME: check if this works
        },
    )
    .build();
    builder
        .exec(contract_creation_request)
        .expect_success()
        .commit();

    let contract_creation_request = ExecuteRequestBuilder::standard(
        default_account,
        ERC20,
        runtime_args! {
            "name" => "BTH",
            "symbol" => "BTH",
            "decimals" => 9u8,
            "total_supply" => U256::from(10_000u64).checked_mul(U256::exp10(9)).unwrap(),
        },
    )
    .build();
    builder
        .exec(contract_creation_request)
        .expect_success()
        .commit();

    // Extracts the contract hash from the named keys of the account in question, the default
    // genesis address.
    let marketplace_contract_hash =
        get_contract_hash(&builder, default_account, "marketplace_contract_hash");

    let nft_contract_hash = get_contract_hash(&builder, default_account, "nft_contract");
    let erc20_contract_hash = get_contract_hash(&builder, default_account, "erc20_token_contract");
    (
        builder,
        marketplace_contract_hash,
        nft_contract_hash,
        erc20_contract_hash,
    )
}

#[test]
fn should_add_accepted_token() {
    let (mut builder, marketplace_contract_hash, _nft_contract_hash, _erc20_contract_hash) =
        should_deploy();
    let default_account = *DEFAULT_ACCOUNT_ADDR;
    let session_code_request = ExecuteRequestBuilder::contract_call_by_hash(
        // Again, using the default account hash included with genesis.
        default_account,
        // Telling the execution request builder to load up an instance of a deploy built from
        // donate.wasm.
        marketplace_contract_hash,
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

#[test]
fn should_add_sell_order() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        should_deploy();
    let default_account = *DEFAULT_ACCOUNT_ADDR;

    let nft_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        default_account,
        nft_contract_hash,
        "mint",
        runtime_args! {
            "nft_contract_hash" => nft_contract_hash,
            "token_owner" => Key::from(default_account),
            "token_meta_data" => red_dragon(),
        },
    )
    .build();

    builder.exec(nft_mint_request).expect_success().commit();

    // let session_code_request = ExecuteRequestBuilder::standard(
    //     default_account,
    //     "balance_of.wasm",
    //     runtime_args! {
    //         "token_owner" => default_account,
    //         "nft_contract_hash" => Key::from(nft_contract_hash),
    //         "key_name" => "balance_of".to_string(),
    //     },
    // )
    // .build();
    // builder.exec(session_code_request).expect_success().commit();
    // let balance_of_marketplace = query_stored_value::<u64>(
    //     &mut builder,
    //     Key::from(default_account),
    //     ["balance_of".to_string()].into(),
    // );

    // assert_eq!(balance_of_marketplace, 1u64);

    // let nft_approve_request = ExecuteRequestBuilder::contract_call_by_hash(
    //     default_account,
    //     nft_contract_hash,
    //     "approve",
    //     runtime_args! {
    //         "nft_contract_hash" => nft_contract_hash,
    //         "operator" => Key::from(marketplace_contract_hash),
    //         "token_id" => 0u64,
    //     },
    // )
    // .build();

    // builder.exec(nft_approve_request).expect_success().commit();

    // let session_code_request = ExecuteRequestBuilder::contract_call_by_hash(
    //     // Again, using the default account hash included with genesis.
    //     default_account,
    //     // Telling the execution request builder to load up an instance of a deploy built from
    //     // donate.wasm.
    //     marketplace_contract_hash,
    //     // Including the necessary runtime arguments.
    //     ENTRY_POINT_ADD_LISTING,
    //     runtime_args! {
    //         COLLECTION_ARG => Key::Hash(nft_contract_hash.value()),
    //         TOKEN_ID_ARG => 0u64,
    //         PAY_TOKEN_ARG => Key::Hash(erc20_contract_hash.value()),
    //         PRICE_ARG => U256::from(0u64),
    //     },
    // )
    // .build();

    // // Execute this request.
    // builder.exec(session_code_request).expect_success().commit();
}
