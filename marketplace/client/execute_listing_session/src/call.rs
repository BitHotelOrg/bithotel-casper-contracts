#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use core::ptr::null;

use alloc::string::{String, ToString};

use casper_contract::{
    contract_api::{
        account, runtime, storage,
        system::{self, create_purse, transfer_from_purse_to_account},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    api_error::ApiError,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args,
    system::mint::Error,
    CLTyped, ContractHash, Key, RuntimeArgs, URef, U512,
};

use crate::structs::{dict::Dict, order::Listing};

const ARG_MARKETPLACE_CONTRACT_HASH: &str = "marketplace_contract_hash";
const ARG_LISTING_ID: &str = "listing_id";
const ARG_AMOUNT: &str = "amount";
const ARG_PURSE: &str = "purse";
const ENTRY_POINT_EXECUTE_LISTING: &str = "execute_listing";
const ENTRY_POINT_GET_LISTING: &str = "get_listing";
/**
 * TODO: call execute listing from marketplace contract
 */

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg("amount");

    let marketplace_contract_hash: ContractHash =
        runtime::get_named_arg::<Key>(ARG_MARKETPLACE_CONTRACT_HASH)
            .into_hash()
            .map(|hash| ContractHash::new(hash))
            .unwrap();
    let listing_id: u64 = runtime::get_named_arg(ARG_LISTING_ID);

    let caller_purse = account::get_main_purse();
    let new_purse = system::create_purse();

    // Transfer from the caller's main purse to the new purse that was just created.
    // Note that transfer is done safely by the host logic.
    system::transfer_from_purse_to_purse(account::get_main_purse(), new_purse, amount, None)
        .unwrap_or_revert();

    runtime::call_contract(
        marketplace_contract_hash,
        ENTRY_POINT_EXECUTE_LISTING,
        runtime_args! {
            ARG_LISTING_ID => listing_id,
            ARG_PURSE => new_purse,
        },
    )
}
