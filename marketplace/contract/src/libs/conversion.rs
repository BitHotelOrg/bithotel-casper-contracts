#![allow(dead_code)]
use casper_types::{
    bytesrepr::{Bytes, FromBytes, ToBytes},
    ApiError, U256, U512,
};

pub trait BytesConversion: Sized {
    fn convert_to_bytes(&self) -> Result<Bytes, ApiError>;
    fn convert_from_bytes(bytes: Bytes) -> Result<Self, ApiError>;
}

impl<T: ToBytes + FromBytes> BytesConversion for T {
    fn convert_to_bytes(&self) -> Result<Bytes, ApiError> {
        match self.to_bytes() {
            Ok(bytes) => Ok(Bytes::from(bytes)),
            Err(_) => Err(ApiError::Formatting),
        }
    }

    fn convert_from_bytes(bytes: Bytes) -> Result<Self, ApiError> {
        let conversion = T::from_bytes(&bytes);
        match conversion {
            Ok((v, rest)) => {
                if !rest.is_empty() {
                    Err(ApiError::Formatting)
                } else {
                    Ok(v)
                }
            }
            Err(_) => Err(ApiError::Formatting),
        }
    }
}

pub fn u512_to_u256(u512: &U512) -> Result<U256, ApiError> {
    U256::convert_from_bytes(u512.convert_to_bytes()?)
}

pub fn u256_to_512(u256: &U256) -> Result<U512, ApiError> {
    U512::convert_from_bytes(u256.convert_to_bytes()?)
}
