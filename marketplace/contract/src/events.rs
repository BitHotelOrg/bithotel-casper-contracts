use crate::{alloc::string::ToString, utils::get_current_address, TokenId};
use alloc::{collections::BTreeMap, vec::Vec};
use casper_contract::contract_api::storage;
use casper_types::{account::AccountHash, ContractHash, URef, U256};
pub enum MarketplaceEvent {
    AddListing {
        listing_id: u64,
        seller: AccountHash,
        collection: ContractHash,
        token_id: TokenId,
        price: U256,
    },
    CancelListing {
        listing_id: u64,
    },
    ExecuteListing {
        listing_id: u64,
        buyer: AccountHash,
    },
}

pub fn emit(event: &MarketplaceEvent) {
    let mut events = Vec::new();
    let mut param = BTreeMap::new();
    param.insert(
        "contract_package_hash",
        get_current_address()
            .as_contract_package_hash()
            .unwrap()
            .to_string(),
    );
    match event {
        MarketplaceEvent::AddListing {
            listing_id,
            seller,
            collection,
            token_id,
            price,
        } => {
            param.insert("event_type", "add_listing".to_string());
            param.insert("listing_id", listing_id.to_string());
            param.insert("seller", seller.to_string());
            param.insert("collection", collection.to_string());
            param.insert("token_id", token_id.to_string());
            param.insert("price", price.to_string());
        }
        MarketplaceEvent::CancelListing { listing_id } => {
            param.insert("listing_id", listing_id.to_string());
        }
        MarketplaceEvent::ExecuteListing { listing_id, buyer } => {
            param.insert("listing_id", listing_id.to_string());
            param.insert("buyer", buyer.to_string());
        }
    }
    events.push(param);
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
