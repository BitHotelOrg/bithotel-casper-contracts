#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use core::ptr::null;

use alloc::string::{String, ToString};

use casper_contract::contract_api::{
    account, runtime, storage,
    system::{self, create_purse, transfer_from_purse_to_account},
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args,
    system::mint::Error,
    CLTyped, ContractHash, Key, RuntimeArgs, URef, U512,
};

use crate::structs::{dict::Dict, order::Listing};

const ARG_MARKETPLACE_CONTRACT_HASH: &str = "marketplace_contract_hash";
const ARG_LISTING_ID: &str = "listing_id";
const ARG_AMOUNT: &str = "amount";

/**
 * TODO: call execute listing from marketplace contract
 */

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let marketplace_contract_hash: ContractHash =
        runtime::get_named_arg::<Key>(ARG_MARKETPLACE_CONTRACT_HASH)
            .into_hash()
            .map(|hash| ContractHash::new(hash))
            .unwrap();
    let listing_id: u64 = runtime::get_named_arg(ARG_LISTING_ID);

    let caller_purse = account::get_main_purse();

    let seller_account_hash = runtime::call_contract::<AccountHash>(
        marketplace_contract_hash,
        "get_listing",
        runtime_args! {
            "listing_id_arg" => listing_id
        },
    );

    let re = system::transfer_from_purse_to_account(
        caller_purse.into_read_write(),
        seller_account_hash,
        U512::from(10),
        None,
    )
    .unwrap();
}
