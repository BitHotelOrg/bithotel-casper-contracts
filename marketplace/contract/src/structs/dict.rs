use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, URef,
};

pub struct Dict {
    uref: URef,
}

impl Dict {
    pub fn init(name: &str) {
        storage::new_dictionary(name).unwrap_or_default();
    }

    pub fn instance(name: &str) -> Dict {
        let uref = *runtime::get_key(name).unwrap().as_uref().unwrap();
        Dict { uref }
    }

    pub fn set<T: CLTyped + ToBytes>(&self, key: &str, value: T) {
        storage::dictionary_put(self.uref, key, value);
    }

    pub fn get<T: CLTyped + FromBytes>(&self, key: &str) -> Option<T> {
        storage::dictionary_get(self.uref, key).unwrap()
    }
}
