use crate::utility::constants::{
    ARG_COLLECTION, ARG_FEE, ARG_LISTING_ID, ARG_MARKETPLACE_HASH, ARG_PAY_TOKEN, ARG_PRICE,
    ARG_TOKEN, ARG_TOKEN_ID, ENTRY_POINT_ADD_LISTING, ENTRY_POINT_CANCEL_LISTING,
    ENTRY_POINT_EXECUTE_LISTING, ENTRY_POINT_SET_ACCEPTED_TOKEN,
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, WasmTestBuilder, ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR,
};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
use casper_types::{
    account::AccountHash, runtime_args, ContractHash, Key, RuntimeArgs, U256, U512,
};

use super::helpers;

#[derive(Clone, Copy)]
pub struct MarketplaceInstance {
    pub contract_hash: ContractHash,
}

impl<'a> MarketplaceInstance {
    pub fn new(contract_hash: ContractHash) -> Self {
        MarketplaceInstance { contract_hash }
    }

    pub fn add_listing(
        self,
        builder: &mut WasmTestBuilder<InMemoryGlobalState>,
        sender: AccountHash,
        collection: ContractHash,
        token_id: u64,
        pay_token: ContractHash,
        price: U256,
        should_succeed: bool,
    ) {
        let request = ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            self.contract_hash,
            ENTRY_POINT_ADD_LISTING,
            runtime_args! {
                ARG_COLLECTION => Key::from(collection),
                ARG_TOKEN_ID => token_id,
                ARG_PAY_TOKEN => Key::from(pay_token),
                ARG_PRICE => price,
            },
        )
        .build();

        builder.exec(request);
        if should_succeed {
            builder.expect_success();
        } else {
            builder.expect_failure();
        }
        builder.commit();
    }

    pub fn cancel_listing(
        self,
        builder: &mut WasmTestBuilder<InMemoryGlobalState>,
        sender: AccountHash,
        listing_id: u64,
        should_succeed: bool,
    ) {
        let request = ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            self.contract_hash,
            ENTRY_POINT_CANCEL_LISTING,
            runtime_args! {
                ARG_LISTING_ID => listing_id,
            },
        )
        .build();

        builder.exec(request);
        if should_succeed {
            builder.expect_success();
        } else {
            builder.expect_failure();
        }
        builder.commit();
    }

    pub fn add_accepted_token(
        self,
        builder: &mut WasmTestBuilder<InMemoryGlobalState>,
        sender: AccountHash,
        collection: ContractHash,
        fee: u32,
        should_succeed: bool,
    ) {
        let request = ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            self.contract_hash,
            ENTRY_POINT_SET_ACCEPTED_TOKEN,
            runtime_args! {
                ARG_TOKEN => collection.to_formatted_string(),
                ARG_FEE => fee,
            },
        )
        .build();

        builder.exec(request);
        if should_succeed {
            builder.expect_success();
        } else {
            builder.expect_failure();
        }
        builder.commit();
    }

    pub fn execute_listing(
        self,
        builder: &mut WasmTestBuilder<InMemoryGlobalState>,
        sender: AccountHash,
        listing_id: u64,
        price: U512,
        should_succeed: bool,
    ) -> U512 {
        let request = ExecuteRequestBuilder::standard(
            sender,
            "execute_listing_call.wasm",
            runtime_args! {
                ARG_MARKETPLACE_HASH => Key::from(self.contract_hash),
                ARG_LISTING_ID => listing_id,
                "amount" => price,
            },
        )
        .build();

        let proposer_reward_starting_balance = builder.get_proposer_purse_balance();

        builder.exec(request);
        if should_succeed {
            builder.expect_success();
        } else {
            builder.expect_failure();
        }
        builder.commit();

        // U512::from(0)
        builder.get_proposer_purse_balance() - proposer_reward_starting_balance
    }
}
