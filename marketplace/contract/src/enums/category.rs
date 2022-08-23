use alloc::vec::Vec;
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Category {
    None,
    Character,
    Room,
    Office,
    Furniture,
    Wearable,
    Badge,
    Consumable, 
    Background
}

impl CLTyped for Category {
    fn cl_type() -> casper_types::CLType {
        CLType::Key
    }
}

impl FromBytes for Category {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (value, bytes) = u8::from_bytes(bytes)?;

        match value {
            0 => Ok((Category::None, bytes)),
            1 => Ok((Category::Character, bytes)),
            2 => Ok((Category::Room, bytes)),
            3 => Ok((Category::Office, bytes)),
            4 => Ok((Category::Furniture, bytes)),
            5 => Ok((Category::Wearable, bytes)),
            6 => Ok((Category::Badge, bytes)),
            7 => Ok((Category::Consumable, bytes)),
            8 => Ok((Category::Background, bytes)),
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

impl ToBytes for Category {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        (*self as u8).to_bytes()
    }

    fn serialized_length(&self) -> usize {
        1
    }
}
