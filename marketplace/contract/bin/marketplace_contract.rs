#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, CLValue, ContractHash, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs,
    URef, U256, U512,
};
use contract_utils::{AdminControl, ContractContext, OnChainContractStorage, ReentrancyGuard};
use kunftmarketplace_contract::{
    get_immediate_caller_address, Address, Marketplace, Time, TokenId,
};

#[derive(Default)]
struct MarketplaceContract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for MarketplaceContract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl Marketplace<OnChainContractStorage> for MarketplaceContract {}
impl ReentrancyGuard<OnChainContractStorage> for MarketplaceContract {}
impl AdminControl<OnChainContractStorage> for MarketplaceContract {}

impl MarketplaceContract {
    fn constructor(&mut self, acceptable_tokens: BTreeMap<String, u32>, fee_wallet: Address) {
        Marketplace::init(self, acceptable_tokens, fee_wallet);
        ReentrancyGuard::init(self);
        AdminControl::init(self);
    }
}

#[no_mangle]
pub extern "C" fn constructor() {
    let acceptable_tokens: BTreeMap<String, u32> = runtime::get_named_arg("acceptable_tokens");
    let fee_wallet: Address = runtime::get_named_arg("fee_wallet");
    MarketplaceContract::default().constructor(acceptable_tokens, fee_wallet);
    let default_admin = runtime::get_caller();
    MarketplaceContract::default().add_admin_without_checked(Key::from(default_admin));
}

#[no_mangle]
pub extern "C" fn create_sell_order() {
    let caller = get_immediate_caller_address().unwrap();
    let start_time: Time = runtime::get_named_arg("start_time");
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let tokens: BTreeMap<TokenId, U256> = runtime::get_named_arg("tokens");
    let pay_token: Option<ContractHash> = {
        let pay_token_str: Option<String> = runtime::get_named_arg("pay_token");
        pay_token_str.map(|str| ContractHash::from_formatted_str(&str).unwrap())
    };

    MarketplaceContract::default()
        .create_sell_order(caller, start_time, collection, pay_token, tokens);
}

#[no_mangle]
pub extern "C" fn buy_sell_order_cspr() {
    let caller = get_immediate_caller_address().unwrap();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    let amount: U512 = runtime::get_named_arg("amount");
    let additional_recipient: Option<Address> = runtime::get_named_arg("additional_recipient");
    MarketplaceContract::default().set_reentrancy();
    MarketplaceContract::default().buy_sell_order_cspr(
        caller,
        collection,
        token_id,
        amount,
        additional_recipient,
    );
    MarketplaceContract::default().clear_reentrancy();
}

#[no_mangle]
pub extern "C" fn buy_sell_order() {
    let caller = get_immediate_caller_address().unwrap();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    let amount: U256 = runtime::get_named_arg("amount");
    let additional_recipient: Option<Address> = runtime::get_named_arg("additional_recipient");
    MarketplaceContract::default().set_reentrancy();
    MarketplaceContract::default().buy_sell_order(
        caller,
        collection,
        token_id,
        amount,
        additional_recipient,
    );
    MarketplaceContract::default().clear_reentrancy();
}

#[no_mangle]
pub extern "C" fn cancel_sell_order() {
    let caller = get_immediate_caller_address().unwrap();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
    MarketplaceContract::default().cancel_sell_order(caller, collection, token_ids);
}

#[no_mangle]
pub extern "C" fn create_buy_order_cspr() {
    let caller = get_immediate_caller_address().unwrap();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    let additional_recipient: Option<Address> = runtime::get_named_arg("additional_recipient");
    let amount: U512 = runtime::get_named_arg("amount");
    MarketplaceContract::default().set_reentrancy();
    MarketplaceContract::default().create_buy_order_cspr(
        caller,
        collection,
        token_id,
        additional_recipient,
        amount,
    );
    MarketplaceContract::default().clear_reentrancy();
}

#[no_mangle]
pub extern "C" fn create_buy_order() {
    let caller = get_immediate_caller_address().unwrap();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    let additional_recipient: Option<Address> = runtime::get_named_arg("additional_recipient");
    let pay_token: ContractHash = {
        let pay_token_str: String = runtime::get_named_arg("pay_token");
        ContractHash::from_formatted_str(&pay_token_str).unwrap()
    };
    let amount: U256 = runtime::get_named_arg("amount");

    MarketplaceContract::default().create_buy_order(
        caller,
        collection,
        token_id,
        additional_recipient,
        pay_token,
        amount,
    );
}

#[no_mangle]
pub extern "C" fn cancel_buy_order() {
    let caller = get_immediate_caller_address().unwrap();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    MarketplaceContract::default().set_reentrancy();
    MarketplaceContract::default().cancel_buy_order(caller, collection, token_id);
    MarketplaceContract::default().clear_reentrancy();
}

#[no_mangle]
pub extern "C" fn accept_buy_order() {
    let caller = get_immediate_caller_address().unwrap();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    let bidder: Address = runtime::get_named_arg("bidder");
    MarketplaceContract::default().set_reentrancy();
    MarketplaceContract::default().accept_buy_order(caller, collection, token_id, bidder);
    MarketplaceContract::default().clear_reentrancy();
}

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    let purse = MarketplaceContract::default().purse();
    // https://github.com/Jiuhong-casperlabs/restrict-access-right/blob/main/contract/src/contract.rs#L25
    runtime::ret(CLValue::from_t(purse.into_add()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_acceptable_token() {
    let contract_hash: ContractHash = {
        let contract_hash_str: String = runtime::get_named_arg("contract_hash");
        ContractHash::from_formatted_str(&contract_hash_str).unwrap()
    };
    let fee: u32 = runtime::get_named_arg("fee");
    MarketplaceContract::default().assert_caller_is_admin();
    MarketplaceContract::default().set_acceptable_token(contract_hash, fee);
}

#[no_mangle]
pub extern "C" fn remove_acceptable_token() {
    let contract_hash: ContractHash = {
        let contract_hash_str: String = runtime::get_named_arg("contract_hash");
        ContractHash::from_formatted_str(&contract_hash_str).unwrap()
    };
    MarketplaceContract::default().assert_caller_is_admin();
    MarketplaceContract::default().remove_acceptable_token(contract_hash);
}

#[no_mangle]
pub extern "C" fn set_fee_wallet() {
    let fee_wallet: Address = runtime::get_named_arg("fee_wallet");
    MarketplaceContract::default().assert_caller_is_admin();
    MarketplaceContract::default().set_fee_wallet(fee_wallet);
}

#[no_mangle]
pub extern "C" fn call() {
    let contract_name: String = runtime::get_named_arg("contract_name");
    let acceptable_tokens: BTreeMap<String, u32> = runtime::get_named_arg("acceptable_tokens");
    let fee_wallet: Address = runtime::get_named_arg("fee_wallet");
    let exist_contract_package_hash: Option<ContractPackageHash> = {
        let contract_package_hash_str: Option<String> =
            runtime::get_named_arg("contract_package_hash");
        contract_package_hash_str.map(|str| ContractPackageHash::from_formatted_str(&str).unwrap())
    };
    let (contract_hash, _) = match exist_contract_package_hash {
        Some(contract_package_hash) => {
            let named_keys = NamedKeys::new();

            storage::add_contract_version(contract_package_hash, get_entry_points(), named_keys)
        }
        None => storage::new_contract(
            get_entry_points(),
            None,
            Some(format!("{}_contract_package_hash", contract_name)),
            Some(format!("{}_contract_access_token", contract_name)),
        ),
    };

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(&format!("{}_contract_package_hash", contract_name))
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let constructor_access: URef = match exist_contract_package_hash {
        Some(contract_package_hash) => {
            storage::provision_contract_user_group_uref(contract_package_hash, "constructor")
                .unwrap()
        }
        None => {
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert()
        }
    };
    let constructor_args = runtime_args! {
        "acceptable_tokens" => acceptable_tokens,
        "fee_wallet" => fee_wallet
    };
    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new(
                "acceptable_tokens",
                CLType::Map {
                    key: Box::new(CLType::String),
                    value: Box::new(CLType::U32),
                },
            ),
            Parameter::new("fee_wallet", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_sell_order",
        vec![
            Parameter::new("start_time", CLType::U64),
            Parameter::new("collection", CLType::String),
            Parameter::new(
                "tokens",
                CLType::Map {
                    key: Box::new(CLType::U256),
                    value: Box::new(CLType::U256),
                },
            ),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "buy_sell_order_cspr",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
            Parameter::new("amount", CLType::U512),
            Parameter::new(
                "additional_recipient",
                CLType::Option(Box::new(CLType::Key)),
            ),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "buy_sell_order",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
            Parameter::new("amount", CLType::U512),
            Parameter::new(
                "additional_recipient",
                CLType::Option(Box::new(CLType::Key)),
            ),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "cancel_sell_order",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    //
    entry_points.add_entry_point(EntryPoint::new(
        "create_buy_order_cspr",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
            Parameter::new(
                "additional_recipient",
                CLType::Option(Box::new(CLType::Key)),
            ),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_buy_order",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
            Parameter::new(
                "additional_recipient",
                CLType::Option(Box::new(CLType::Key)),
            ),
            Parameter::new("pay_token", CLType::String),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "cancel_buy_order",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "accept_buy_order",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
            Parameter::new("bidder", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_acceptable_token",
        vec![
            Parameter::new("contract_hash", CLType::String),
            Parameter::new("fee", CLType::U32),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "remove_acceptable_token",
        vec![Parameter::new("contract_hash", CLType::String)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_wallet",
        vec![Parameter::new("fee_wallet", CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_deposit_purse",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
