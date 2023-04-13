#![allow(dead_code)]
use casper_contract::{contract_api::runtime, ext_ffi, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, system::CallStackElement, ContractHash, Key};

use crate::{
    enums::Address,
    marketplace::{ADMIN_DICT, OPTIONS_DICT, PAUSED_OPTION, WHITELIST_DICT},
    structs::dict::Dict,
};
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_types::{api_error, bytesrepr::ToBytes, CLTyped};

fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

pub fn get_current_address() -> Address {
    let call_stack_element = runtime::get_call_stack()
        .into_iter()
        .rev()
        .next()
        .unwrap_or_revert();
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

pub fn element_to_key(element: &CallStackElement) -> Key {
    match element {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => (*contract_package_hash).into(),
    }
}

pub fn key_value_to_storage_key<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut bytes_0 = key.to_bytes().unwrap_or_revert();
    let mut bytes_1 = value.to_bytes().unwrap_or_revert();

    bytes_0.append(&mut bytes_1);

    let bytes_key = runtime::blake2b(bytes_0);
    hex::encode(bytes_key)
}

pub fn is_admin(caller: AccountHash) -> bool {
    let admins = Dict::instance(ADMIN_DICT);
    admins.get(&caller.to_string()).unwrap_or(false)
}

pub fn is_whitelisted(contract: ContractHash) -> bool {
    let whitelist = Dict::instance(WHITELIST_DICT);
    whitelist.get(&contract.to_string()).unwrap_or(false)
}

pub fn is_paused() -> bool {
    let options = Dict::instance(OPTIONS_DICT);
    options.get(PAUSED_OPTION).unwrap_or_revert()
}

pub fn named_uref_exists(name: &str) -> bool {
    let (name_ptr, name_size, _bytes) = to_ptr(name);
    let mut key_bytes = vec![0u8; Key::max_serialized_length()];
    let mut total_bytes: usize = 0;
    let ret = unsafe {
        ext_ffi::casper_get_key(
            name_ptr,
            name_size,
            key_bytes.as_mut_ptr(),
            key_bytes.len(),
            &mut total_bytes as *mut usize,
        )
    };

    api_error::result_from(ret).is_ok()
}

pub fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
    let bytes = t.into_bytes().unwrap_or_revert();
    let ptr = bytes.as_ptr();
    let size = bytes.len();
    (ptr, size, bytes)
}
