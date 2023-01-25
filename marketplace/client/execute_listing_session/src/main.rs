#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U512};

const ARG_MARKETPLACE_CONTRACT_HASH: &str = "marketplace_contract_hash";
const ARG_LISTING_ID: &str = "listing_id";
const ARG_AMOUNT: &str = "amount";
const ARG_PURSE: &str = "purse";
const ENTRY_POINT_EXECUTE_LISTING: &str = "execute_listing";

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

    let marketplace_contract_hash: ContractHash =
        runtime::get_named_arg::<Key>(ARG_MARKETPLACE_CONTRACT_HASH)
            .into_hash()
            .map(ContractHash::new)
            .unwrap();
    let listing_id: u64 = runtime::get_named_arg(ARG_LISTING_ID);

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
