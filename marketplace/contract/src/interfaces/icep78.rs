#![allow(dead_code)]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U256};

use crate::{enums::Address, TokenId};

pub struct ICEP78 {
    pub contract_hash: ContractHash,
}

impl ICEP78 {
    pub fn new(contract_hash: ContractHash) -> Self {
        ICEP78 { contract_hash }
    }

    pub fn balance_of(&self, owner: Address) -> u64 {
        runtime::call_contract::<u64>(
            self.contract_hash,
            "balance_of",
            runtime_args! {
              "owner" => owner,
            },
        )
    }

    pub fn approve(&self, spender: Address, token_ids: Vec<U256>) -> () {
        runtime::call_contract::<()>(
            self.contract_hash,
            "approve",
            runtime_args! {
              "spender" => spender,
              "token_ids" => token_ids
            },
        )
    }
    pub fn get_approved(&self, owner: Address, token_id: TokenId) -> Option<Key> {
        runtime::call_contract::<Option<Key>>(
            self.contract_hash,
            "get_approved",
            runtime_args! {
              "owner" => owner,
              "token_id" => token_id
            },
        )
    }

    pub fn owner_of(&self, token_id: TokenId) -> Key {
        runtime::call_contract::<Key>(
            self.contract_hash,
            "owner_of",
            runtime_args! {
              "token_id" => token_id
            },
        )
    }

    pub fn transfer(&self, sender: Key, recipient: Key, token_id: TokenId) -> (String, Key) {
        runtime::call_contract::<(String, Key)>(
            self.contract_hash,
            "transfer",
            runtime_args! {
                "token_id" => token_id,
                "source_key" => sender,
                "target_key" => recipient,
            },
        )
    }
}
