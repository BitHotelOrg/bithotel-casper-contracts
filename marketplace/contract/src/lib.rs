#![no_std]
#![no_main]
extern crate alloc;

mod enums;
mod error;
mod interfaces;
mod marketplace;
mod structs;
mod utils;
use casper_types::ContractHash;
use structs::dict::Dict;
pub type Time = u64;
pub type TokenId = u64;
pub type Token = (ContractHash, TokenId);
