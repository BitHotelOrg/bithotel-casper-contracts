#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use core::ptr::null;

use alloc::string::{String, ToString};

use casper_contract::contract_api::{runtime, storage, account, system};
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U512};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, URef,
};
use casper_types::system::mint::Error;

use crate::structs::{dict::Dict, order::Listing};

const ARG_MARKETPLACE_CONTRACT_HASH: &str = "marketplace_contract_hash";
const ARG_LISTING_ID: &str = "listing_id";
const ARG_AMOUNT: &str = "amount";

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let marketplace_contract_hash: ContractHash = runtime::get_named_arg::<Key>(ARG_MARKETPLACE_CONTRACT_HASH)
        .into_hash()
        .map(|hash| ContractHash::new(hash))
        .unwrap();
    let listing_id: u64 = runtime::get_named_arg(ARG_LISTING_ID);

    let caller_purse = account::get_main_purse();
    let marketplace_purse = system::create_purse();

    // TODO: transfer to listing owner
    system::transfer_from_purse_to_purse(
        caller_purse.into_read_write(), 
        marketplace_purse.into_add(),
        U512::from(10),
        None
    ).unwrap_or_else(|_| runtime::revert(Error::UnapprovedSpendingAmount));
}
