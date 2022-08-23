#![no_std]
extern crate alloc;

mod data;
mod enums;
mod error;
mod event;
mod interfaces;
mod libs;
mod marketplace;
mod structs;
use alloc::collections::BTreeMap;
use casper_types::{ContractHash, U256};
pub use error::Error;
pub type Time = u64;
pub type TokenId = U256;
pub type Token = (ContractHash, TokenId);
pub type Bids = BTreeMap<Address, BuyOrder>;
pub use enums::Address;
pub use libs::address_utils::get_immediate_caller_address;
pub use marketplace::Marketplace;
use structs::order::BuyOrder;
pub use structs::order::SellOrder;
