#![allow(dead_code)]
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, Key};

use crate::enums::Address;
use alloc::string::String;
use casper_types::{bytesrepr::ToBytes, CLTyped};

fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

pub fn get_current_address(call_stack_element: CallStackElement) -> Address {
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
