#![no_std]
#![no_main]
#[macro_use]
extern crate alloc;

mod enums;
mod error;
mod interfaces;
mod marketplace;
mod structs;
mod utils;
use casper_types::ContractHash;
pub type Time = u64;
pub type TokenId = u64;
pub type Token = (ContractHash, TokenId);
