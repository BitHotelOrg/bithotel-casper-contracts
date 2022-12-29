use crate::TokenId;
use alloc::vec::Vec;
use core::convert::TryFrom;

use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, ContractHash, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};

#[derive(Clone, Copy, Debug, CLTyped, ToBytes, FromBytes)]
pub struct Listing {
    pub owner: AccountHash,
    pub collection: ContractHash,
    pub token_id: TokenId,
    pub pay_token: ContractHash,
    pub price: U256,
    pub status: Status,
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Added = 0,
    Cancelled = 1,
    Executed = 2,
}

impl FromBytes for Status {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (result, bytes) = u8::from_bytes(bytes).unwrap();
        Ok((Status::try_from(result).unwrap(), bytes))
    }
}

impl ToBytes for Status {
    fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend((*self as u8).to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        (*self as u8).serialized_length()
    }

    fn into_bytes(self) -> Result<alloc::vec::Vec<u8>, bytesrepr::Error>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
}

impl TryFrom<u8> for Status {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Added),
            1 => Ok(Status::Cancelled),
            2 => Ok(Status::Executed),
            _ => Err(()),
        }
    }
}

impl CLTyped for Status {
    fn cl_type() -> casper_types::CLType {
        CLType::U8
    }
}
