// extern crate alloc;
use std::collections::BTreeMap;
// Outlining aspects of the Casper test support crate to include.
use crate::utility::{
    constants::{
        ACCEPTED_TOKENS_ARG, CEP78, ENTRY_POINT_CANCEL_LISTING, ERC20, FEE_WALLET_ARG,
        LISTING_ID_ARG, MARKETPLACE, USER_ACCOUNT_0,
    },
    helpers::{get_contract_hash, nft_get_balance, query_stored_value},
    marketplace_interface::MarketplaceInstance,
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::{
    core::{engine_state::balance, validate_balance_proof},
    storage::global_state::in_memory::InMemoryGlobalState,
};
// Custom Casper types that will be used within this test.
use self::meta::metadata_0;
use casper_types::{
    account::{Account, AccountHash},
    bytesrepr::FromBytes,
    runtime_args,
    system::{self, handle_payment::MintProvider, mint},
    CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, SecretKey, U256,
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

fn get_account_0() -> AccountHash {
    let secret_key = SecretKey::ed25519_from_bytes(USER_ACCOUNT_0).unwrap();
    PublicKey::from(&secret_key).to_account_hash()
}

fn deploy() -> (
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

fn mint_nft(
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

fn approve_nft(
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

fn deploy_with_nft(
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
        deploy();

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.add_accepted_token(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        _erc20_contract_hash,
        20u32,
        true,
    );
}

#[test]
fn should_add_and_cancel_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let mut balance_of_account = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance_of_account, 1u64);

    let marketplace = MarketplaceInstance {
        contract_hash: marketplace_contract_hash,
    };
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(100u64),
        true,
    );

    marketplace.cancel_listing(&mut builder, *DEFAULT_ACCOUNT_ADDR, 1u64, true);

    balance_of_account = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance_of_account, 1u64);
}

#[test]
fn should_not_cancel_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);

    marketplace.cancel_listing(&mut builder, *DEFAULT_ACCOUNT_ADDR, 1u64, false);
}

#[test]
fn should_not_execute_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);

    marketplace.execute_listing(&mut builder, *DEFAULT_ACCOUNT_ADDR, 1u64, false);
}

#[test]
fn should_not_add_and_execute_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance {
        contract_hash: marketplace_contract_hash,
    };

    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(100u64),
        true,
    );

    let mut balance = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );

    assert_eq!(balance, 0);

    marketplace.execute_listing(&mut builder, *DEFAULT_ACCOUNT_ADDR, 1u64, false);

    balance = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );

    assert_eq!(balance, 0);
}

#[test]
fn should_execute_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance {
        contract_hash: marketplace_contract_hash,
    };

    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(100i32),
        true,
    );

    let fund_account_request = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            mint::ARG_AMOUNT => 100_000_000_000_000u64,
            mint::ARG_TARGET => get_account_0(),
            mint::ARG_ID => Option::<u64>::None,
        },
    )
    .build();

    builder.exec(fund_account_request).expect_success().commit();

    marketplace.execute_listing(&mut builder, get_account_0(), 1u64, true);

    let balance_buyer = nft_get_balance(
        &mut builder,
        get_account_0(),
        nft_contract_hash,
        get_account_0().into(),
    );

    let balance_seller = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );

    assert_eq!(balance_buyer, 1);
    assert_eq!(balance_seller, 0);
}

#[test]
fn get_account() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);
    let marketplace = MarketplaceInstance {
        contract_hash: marketplace_contract_hash,
    };
    let price = 10u64;
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(price),
        true,
    );
    let fund_account_request = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            mint::ARG_AMOUNT => 100_000_000_000_000u64,
            mint::ARG_TARGET => get_account_0(),
            mint::ARG_ID => Option::<u64>::None,
        },
    )
    .build();

    builder.exec(fund_account_request).expect_success().commit();
    let default_account = builder.get_account(get_account_0()).unwrap();

    let balance_before = builder.get_purse_balance(default_account.main_purse());
    let fee = marketplace.buy_listing(&mut builder, get_account_0());
    let balance_after = builder.get_purse_balance(default_account.main_purse());
    assert_eq!(balance_before, balance_after + fee + price);
}

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
