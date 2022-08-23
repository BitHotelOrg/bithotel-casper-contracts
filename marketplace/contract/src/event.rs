use casper_types::{ContractHash, U256};

use crate::{Address, Time, TokenId};

pub enum MarketplaceEvent {
    SellOrderCreated {
        creator: Address,
        collection: ContractHash,
        token_id: TokenId,
        pay_token: Option<ContractHash>,
        price: U256,
        start_time: Time,
    },
    SellOrderCanceled {
        creator: Address,
        start_time: Time,
        collection: ContractHash,
        token_id: TokenId,
    },
    SellOrderBought {
        creator: Address,
        start_time: Time,
        collection: ContractHash,
        token_id: TokenId,
        buyer: Address,
        additional_recipient: Option<Address>,
    },
    BuyOrderCreated {
        creator: Address,
        collection: ContractHash,
        token_id: TokenId,
        pay_token: Option<ContractHash>,
        price: U256,
        additional_recipient: Option<Address>,
        start_time: Time,
    },
    BuyOrderCanceled {
        creator: Address,
        collection: ContractHash,
        token_id: TokenId,
        start_time: Time,
    },
    BuyOrderAccepted {
        creator: Address,
        collection: ContractHash,
        token_id: TokenId,
        start_time: Time,
        owner: Address,
    },
    AcceptableTokenAdded {
        contract_hash: ContractHash,
        fee: u32,
    },
    AcceptableTokenRemoved {
        contract_hash: ContractHash,
    },
}
