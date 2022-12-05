use crate::{
    enums::Address, error::MarketplaceError, interfaces::icep78::ICEP78, structs::order::SellOrder,
    utils::get_immediate_caller_address, Time, TokenId,
};

use alloc::{boxed::Box, collections::BTreeMap};
// Importing Rust types.
use alloc::{
    string::{String, ToString},
    vec,
};
// Importing aspects of the Casper platform.
// use casper_contract::contract_api::storage::dictionary_get;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
// Importing specific Casper types.
// use casper_types::account::AccountHash;
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, U256,
};

// Creating constants for the various contract entry points.
const ENTRY_POINT_INIT: &str = "init";
const ENTRY_POINT_SET_ACCEPTED_TOKEN: &str = "set_accepted_token";
const ENTRY_POINT_ADD_LISTING: &str = "add_listing";
// const ENTRY_POINT_ADD_LISTING: &str = "add_listing";

// Creating constants for the entry point arguments.
const CONTRACT_NAME_ARG: &str = "contract_name_arg";
const FEE_WALLET_ARG: &str = "fee_wallet_arg";
const ACCEPTED_TOKENS_ARG: &str = "accepted_tokens_arg";
const TOKEN_ARG: &str = "token_arg";
const FEE_ARG: &str = "fee_arg";
const COLLECTION_ARG: &str = "collection_arg";
const TOKEN_ID_ARG: &str = "token_id_arg";
const PAY_TOKEN_ARG: &str = "pay_token_arg";
const PRICE_ARG: &str = "price_arg";

// Creating constants for values within the contract.
const FEE_WALLET: &str = "fee_wallet";
const ACCEPTED_TOKENS_DICT: &str = "accepted_tokens_dict";
const SELL_ORDERS_DICT: &str = "sell_orders_dict";

// Creating constants for the Urefs
const ACCEPTED_TOKENS_UREF: &str = "accepted_tokens_uref";
const SELL_ORDERS_UREF: &str = "sell_orders_uref";

// This entry point initializes the marketplace, setting up the fee wallet
// and creating a dictionary to track the accepted tokens.
#[no_mangle]
pub extern "C" fn init() {
    let fee_wallet_hash = runtime::get_named_arg::<Key>(FEE_WALLET_ARG);
    let accepted_tokens = runtime::get_named_arg::<BTreeMap<String, u32>>(ACCEPTED_TOKENS_ARG);
    runtime::put_key(FEE_WALLET, fee_wallet_hash.into());
    storage::new_dictionary(SELL_ORDERS_DICT).unwrap_or_revert();
    storage::new_dictionary(ACCEPTED_TOKENS_DICT).unwrap_or_revert();
    let sell_orders_dict = *runtime::get_key(SELL_ORDERS_DICT)
        .unwrap()
        .as_uref()
        .unwrap();
    let accepted_tokens_dict = *runtime::get_key(ACCEPTED_TOKENS_DICT)
        .unwrap()
        .as_uref()
        .unwrap();
    runtime::put_key(SELL_ORDERS_UREF, Key::from(sell_orders_dict));
    runtime::put_key(ACCEPTED_TOKENS_UREF, Key::from(accepted_tokens_dict));
    // Create a dictionary to track the mapping of account hashes to number of donations made.

    accepted_tokens.iter().for_each(|token| {
        let contract_hash = ContractHash::from_formatted_str(token.0).unwrap();
        storage::dictionary_put(accepted_tokens_dict, &contract_hash.to_string(), *token.1)
    });
}

#[no_mangle]
pub extern "C" fn set_accepted_token() {
    let contract_hash =
        ContractHash::from_formatted_str(&runtime::get_named_arg::<String>(TOKEN_ARG)).unwrap();
    let token_fee = runtime::get_named_arg::<u32>(FEE_ARG);
    let accepted_tokens_uref = *runtime::get_key(ACCEPTED_TOKENS_UREF)
        .unwrap()
        .as_uref()
        .unwrap();
    storage::dictionary_put(accepted_tokens_uref, &contract_hash.to_string(), token_fee);
}

#[no_mangle]
pub extern "C" fn add_listing() {
    let caller = get_immediate_caller_address().ok().unwrap();
    let collection = runtime::get_named_arg::<Key>(COLLECTION_ARG)
        .into_hash()
        .map(|hash| ContractHash::new(hash))
        .unwrap();
    let token_id: TokenId = runtime::get_named_arg(TOKEN_ID_ARG);
    let price: U256 = runtime::get_named_arg(PRICE_ARG);
    let pay_token = runtime::get_named_arg::<Key>(PAY_TOKEN_ARG)
        .into_hash()
        .map(|hash| ContractHash::new(hash))
        .unwrap();

    let sell_order: SellOrder = SellOrder {
        creator: caller,
        collection,
        token_id,
        pay_token,
        price,
        status: 0u8,
    };

    let nft = ICEP78::new(collection);

    // let approved = nft
    //     .get_approved(caller, token_id)
    //     .unwrap_or_revert_with(MarketplaceError::NFTRequireApprove);

    // TODO:
    // if !approved.eq(&Address::from(self.contract_package_hash())) {
    //     self.revert(Error::RequireApprove);
    // }
    // "marketplace_package_hash"

    let marketplace_contract_hash = *runtime::get_call_stack()
        .last()
        .unwrap()
        .contract_hash()
        .unwrap();

    nft.transfer(caller, caller, token_id);
    // let sell_orders_uref = *runtime::get_key(SELL_ORDERS_UREF)
    //     .unwrap()
    //     .as_uref()
    //     .unwrap();
    // let mut key = collection.to_string();
    // key.push_str(&token_id.to_string());
    // storage::dictionary_put(sell_orders_uref, &key, sell_order)
}

//This is the full `call` function as defined within the donation contract.
#[no_mangle]
pub extern "C" fn call() {
    // let contract_name: String = runtime::get_named_arg(CONTRACT_NAME_ARG);
    let accepted_tokens: BTreeMap<String, u32> = runtime::get_named_arg(ACCEPTED_TOKENS_ARG);
    let fee_wallet: Key = runtime::get_named_arg(FEE_WALLET_ARG);
    // This establishes the `init` entry point for initializing the contract's infrastructure.
    let init_entry_point = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![
            Parameter::new(FEE_WALLET_ARG, CLType::Key),
            Parameter::new(
                ACCEPTED_TOKENS_ARG,
                CLType::Map {
                    key: Box::new(CLType::String),
                    value: Box::new(CLType::U32),
                },
            ),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes the `donate` entry point for callers looking to donate.
    let set_accepted_token_entry_point = EntryPoint::new(
        ENTRY_POINT_SET_ACCEPTED_TOKEN,
        vec![
            Parameter::new(TOKEN_ARG, CLType::String),
            Parameter::new(FEE_ARG, CLType::U32),
        ],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let add_listing_entry_point = EntryPoint::new(
        ENTRY_POINT_ADD_LISTING,
        vec![
            Parameter::new(COLLECTION_ARG, CLType::String),
            Parameter::new(TOKEN_ID_ARG, CLType::U256),
            Parameter::new(PAY_TOKEN_ARG, CLType::String),
            Parameter::new(PRICE_ARG, CLType::U256),
        ],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init_entry_point);
    entry_points.add_entry_point(set_accepted_token_entry_point);
    entry_points.add_entry_point(add_listing_entry_point);

    let named_keys = NamedKeys::new();

    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("marketplace_package_hash".to_string()),
        Some("marketplace_access_uref".to_string()),
    );

    runtime::put_key("marketplace_contract_hash", contract_hash.into());
    // Call the init entry point to setup and create the fundraising purse
    // and the ledger to track donations made.
    runtime::call_contract::<()>(
        contract_hash,
        ENTRY_POINT_INIT,
        runtime_args! {
            FEE_WALLET_ARG => fee_wallet,
            ACCEPTED_TOKENS_ARG => accepted_tokens,
        },
    )
}
