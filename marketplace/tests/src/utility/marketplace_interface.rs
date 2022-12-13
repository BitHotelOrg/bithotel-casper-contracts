use crate::utility::constants::{
    COLLECTION_ARG, ENTRY_POINT_ADD_LISTING, ENTRY_POINT_CANCEL_LISTING,
    ENTRY_POINT_EXECUTE_LISTING, ENTRY_POINT_SET_ACCEPTED_TOKEN, FEE_ARG, LISTING_ID_ARG,
    PAY_TOKEN_ARG, PRICE_ARG, TOKEN_ARG, TOKEN_ID_ARG,
};
use casper_engine_test_support::{ExecuteRequestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
use casper_types::{account::AccountHash, runtime_args, ContractHash, Key, RuntimeArgs, U256};

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
                COLLECTION_ARG => Key::from(collection),
                TOKEN_ID_ARG => token_id,
                PAY_TOKEN_ARG => Key::from(pay_token),
                PRICE_ARG => price,
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
                LISTING_ID_ARG => listing_id,
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
        should_succeed: bool,
    ) {
        let request = ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            self.contract_hash,
            "execute_listing",
            runtime_args! {
                LISTING_ID_ARG => listing_id,
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
                TOKEN_ARG => collection.to_formatted_string(),
                FEE_ARG => fee,
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
}
