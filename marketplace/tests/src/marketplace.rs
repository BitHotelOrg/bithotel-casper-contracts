// extern crate alloc;
use std::collections::BTreeMap;
// Outlining aspects of the Casper test support crate to include.
use crate::utility::{
    constants::{ARG_FEE_WALLET, CEP78, ERC20, MARKETPLACE, USER_ACCOUNT_0},
    helpers::{get_contract_hash, nft_get_balance, query_stored_value},
    marketplace_interface::MarketplaceInstance,
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
// Custom Casper types that will be used within this test.
use self::meta::metadata_0;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, Key,
    PublicKey, RuntimeArgs, SecretKey, U256,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataStruct {
    name: String,
    symbol: String,
    token_uri: String,
}
mod meta {
    use super::MetadataStruct;

    pub fn metadata_0() -> String {
        let metadata = MetadataStruct {
            name: String::from("Bit Hotel"),
            symbol: String::from("BHOTEL"),
            token_uri: String::from("https://bithotel.io"),
        };
        let metadata_res = serde_json::to_string(&metadata);
        metadata_res.unwrap()
    }
}

pub fn get_account_0() -> AccountHash {
    let secret_key = SecretKey::ed25519_from_bytes(USER_ACCOUNT_0).unwrap();
    PublicKey::from(&secret_key).to_account_hash()
}

pub fn deploy() -> (
    WasmTestBuilder<InMemoryGlobalState>,
    ContractHash,
    ContractHash,
    ContractHash,
) {
    let default_account = *DEFAULT_ACCOUNT_ADDR;

    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

    let contract_creation_request = ExecuteRequestBuilder::standard(
        default_account,
        MARKETPLACE,
        runtime_args! {
            ARG_FEE_WALLET => Key::from(default_account),
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
            "nft_holder_mode" => 2u8, // FIXME: check if this works
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

pub fn mint_nft(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    nft_contract_hash: ContractHash,
) -> () {
    let default_account = *DEFAULT_ACCOUNT_ADDR;
    let nft_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        default_account,
        nft_contract_hash,
        "mint",
        runtime_args! {
            "token_owner" => Key::Account(default_account),
            "token_meta_data" => metadata_0(),
        },
    )
    .build();

    builder.exec(nft_mint_request).expect_success().commit();

    let balance_of_request = ExecuteRequestBuilder::standard(
        default_account,
        "balance_of.wasm",
        runtime_args! {
            "nft_contract_hash" => Key::from(nft_contract_hash),
            "token_owner" => Key::Account(default_account),
            "key_name" => "balance_of".to_string(),
        },
    )
    .build();

    builder.exec(balance_of_request).expect_success().commit();
    let balance_of = query_stored_value::<u64>(
        builder,
        default_account.into(),
        ["balance_of".to_string()].into(),
    );

    assert_eq!(balance_of, 1u64);
}

pub fn approve_nft(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    nft_contract_hash: ContractHash,
    operator: Key,
    token_id: u64,
) {
    let nft_approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        "approve",
        runtime_args! {
            "nft_contract_hash" => nft_contract_hash,
            "operator" => operator,
            "token_id" => token_id,
        },
    )
    .build();

    builder.exec(nft_approve_request).expect_success().commit();
}

#[test]
fn should_deploy() {
    (_) = deploy();
}

#[test]
fn should_deploy_with_nft() {
    (_) = deploy_with_nft(false);
}

#[test]
fn should_deploy_with_nft_and_approve() {
    (_) = deploy_with_nft(true);
}

pub fn deploy_with_nft(
    approve_marketplace: bool,
) -> (
    WasmTestBuilder<InMemoryGlobalState>,
    ContractHash,
    ContractHash,
    ContractHash,
) {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) = deploy();
    mint_nft(&mut builder, nft_contract_hash);
    if approve_marketplace {
        approve_nft(
            &mut builder,
            nft_contract_hash,
            Key::from(marketplace_contract_hash),
            0u64,
        );
    }

    let balance_of_account = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance_of_account, 1u64);
    (
        builder,
        marketplace_contract_hash,
        nft_contract_hash,
        erc20_contract_hash,
    )
}

#[warn(dead_code)]
fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    nft_contract_key: &Key,
    dictionary_name: &str,
    dictionary_key: &str,
) -> T {
    let seed_uref = *builder
        .query(None, *nft_contract_key, &[])
        .expect("must have nft contract")
        .as_contract()
        .expect("must convert contract")
        .named_keys()
        .get(dictionary_name)
        .expect("must have key")
        .as_uref()
        .expect("must convert to seed uref");

    builder
        .query_dictionary_item(None, seed_uref, dictionary_key)
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap()
}
