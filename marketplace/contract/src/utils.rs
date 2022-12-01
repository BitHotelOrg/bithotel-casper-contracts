#![allow(dead_code)]
use casper_contract::contract_api::runtime;
use casper_types::system::CallStackElement;

use crate::{enums::Address, error::MarketplaceError};

fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

pub fn get_immediate_caller_address() -> Result<Address, MarketplaceError> {
    get_immediate_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(MarketplaceError::InvalidContext)
}

fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
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
