use crate::{enums::Address, Time, TokenId};
use alloc::vec::Vec;
use casper_types::{ContractHash, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};

#[derive(Clone, Copy, Debug, CLTyped, ToBytes, FromBytes)]
pub struct SellOrder {
    pub creator: Address,
    pub collection: ContractHash,
    pub token_id: TokenId,
    pub pay_token: ContractHash,
    pub price: U256,
    pub status: u8,
}

#[derive(Clone, Copy, Debug, CLTyped, ToBytes, FromBytes)]
pub struct BuyOrder {
    pub pay_token: Option<ContractHash>,
    pub price: U256,
    pub start_time: Time,
    pub additional_recipient: Option<Address>,
}
