use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, URef, U512,
};
use contract_utils::{get_key, key_and_value_to_str, key_to_str, set_key, Dict};

use crate::{event::MarketplaceEvent, structs::order::SellOrder, Address, Bids, Error, TokenId};

fn contract_hash_and_value_to_str<T: ToBytes + CLTyped>(
    contract_hash: ContractHash,
    value: T,
) -> String {
    key_and_value_to_str(&Key::from(contract_hash), &value)
}

const SELL_ORDERS_DICT: &str = "sell_orders";

pub struct SellOrders {
    dict: Dict,
}

impl SellOrders {
    pub fn instance() -> SellOrders {
        SellOrders {
            dict: Dict::instance(SELL_ORDERS_DICT),
        }
    }

    pub fn init() {
        Dict::init(SELL_ORDERS_DICT);
    }

    fn contract_hash_and_value_to_str(
        &self,
        contract_hash: ContractHash,
        created_time: TokenId,
    ) -> String {
        key_and_value_to_str(&Key::from(contract_hash), &created_time)
    }

    pub fn get(&self, contract_hash: ContractHash, token_id: TokenId) -> SellOrder {
        self.dict
            .get(&self.contract_hash_and_value_to_str(contract_hash, token_id))
            .unwrap_or_revert_with(Error::NotExistOrder)
    }

    pub fn set(&self, contract_hash: ContractHash, token_id: TokenId, order: SellOrder) {
        self.dict.set(
            &self.contract_hash_and_value_to_str(contract_hash, token_id),
            order,
        );
    }

    pub fn remove(&self, contract_hash: ContractHash, token_id: TokenId) {
        self.dict
            .remove::<SellOrder>(&self.contract_hash_and_value_to_str(contract_hash, token_id));
    }
}

const BUY_ORDERS_DICT: &str = "buy_orders";

pub struct BuyOrders {
    dict: Dict,
}

impl BuyOrders {
    pub fn instance() -> BuyOrders {
        BuyOrders {
            dict: Dict::instance(BUY_ORDERS_DICT),
        }
    }

    pub fn init() {
        Dict::init(BUY_ORDERS_DICT);
    }

    pub fn get(&self, contract_hash: ContractHash, token_id: TokenId) -> Bids {
        self.dict
            .get(&contract_hash_and_value_to_str(contract_hash, token_id))
            .unwrap_or_default()
    }

    pub fn set(&self, contract_hash: ContractHash, token_id: TokenId, bids: Bids) {
        self.dict.set(
            &contract_hash_and_value_to_str(contract_hash, token_id),
            bids,
        );
    }

    pub fn _remove(&self, contract_hash: ContractHash, token_id: TokenId) {
        self.dict
            .remove::<Bids>(&contract_hash_and_value_to_str(contract_hash, token_id));
    }
}

const PURSE_KEY_NAME: &str = "deposit_purse";
const PURSE_BALANCE_KEY_NAME: &str = "purse_balance";

#[derive(Default)]
pub struct DepositPurse {}

impl DepositPurse {
    pub fn init() {
        if runtime::get_key(PURSE_KEY_NAME).is_none() {
            let purse = system::create_purse();
            runtime::put_key(PURSE_KEY_NAME, Key::from(purse));
            set_key(PURSE_BALANCE_KEY_NAME, U512::zero());
        }
    }

    pub fn purse() -> URef {
        *runtime::get_key(PURSE_KEY_NAME).unwrap().as_uref().unwrap()
    }

    pub fn purse_balance() -> U512 {
        get_key(PURSE_BALANCE_KEY_NAME).unwrap_or_revert()
    }

    pub fn update_purse_balance(balance: U512) {
        set_key(PURSE_BALANCE_KEY_NAME, balance);
    }
}

const ACCEPTABLE_TOKENS_DICT: &str = "acceptable_tokens";

pub struct AcceptableTokens {
    dict: Dict,
}

impl AcceptableTokens {
    pub fn instance() -> AcceptableTokens {
        AcceptableTokens {
            dict: Dict::instance(ACCEPTABLE_TOKENS_DICT),
        }
    }

    pub fn init() {
        Dict::init(ACCEPTABLE_TOKENS_DICT)
    }

    pub fn get(&self, contract_hash: ContractHash) -> u32 {
        self.dict
            .get(&key_to_str(&Key::from(contract_hash)))
            .unwrap_or_revert_with(Error::NotAcceptableToken)
    }

    pub fn set(&self, contract_hash: ContractHash, fee: u32) {
        self.dict.set(&key_to_str(&Key::from(contract_hash)), fee)
    }

    pub fn remove(&self, contract_hash: ContractHash) {
        self.dict
            .remove::<u32>(&key_to_str(&Key::from(contract_hash)))
    }
}

const FEE_WALLET_KEY: &str = "fee_wallet";

pub fn set_fee_wallet(wallet: Address) {
    set_key(FEE_WALLET_KEY, wallet);
}

pub fn get_fee_wallet() -> Address {
    get_key(FEE_WALLET_KEY).unwrap_or_revert()
}

pub fn emit(event: &MarketplaceEvent, contract_package_hash: ContractPackageHash) {
    let mut events = Vec::new();
    match event {
        MarketplaceEvent::SellOrderCreated {
            creator,
            collection,
            token_id,
            pay_token,
            price,
            start_time,
        } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("event_type", "SellOrderCreated".to_string());
            param.insert("creator", format!("{:?}", creator));
            param.insert("collection", collection.to_string());
            param.insert("token_id", format!("{}", token_id));
            param.insert("pay_token", format!("{:?}", pay_token));
            param.insert("price", format!("{}", price));
            param.insert("start_time", format!("{}", start_time));
            events.push(param);
        }
        MarketplaceEvent::SellOrderCanceled {
            creator,
            collection,
            token_id,
            start_time,
        } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("event_type", "SellOrderCanceled".to_string());
            param.insert("creator", format!("{:?}", creator));
            param.insert("collection", collection.to_string());
            param.insert("token_id", format!("{}", token_id));
            param.insert("start_time", format!("{}", start_time));
            events.push(param);
        }
        MarketplaceEvent::SellOrderBought {
            creator,
            collection,
            token_id,
            buyer,
            additional_recipient,
            start_time,
        } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("event_type", "SellOrderBought".to_string());
            param.insert("creator", format!("{:?}", creator));
            param.insert("collection", collection.to_string());
            param.insert("token_id", format!("{}", token_id));
            param.insert("buyer", format!("{:?}", buyer));
            param.insert(
                "additional_recipient",
                format!("{:?}", additional_recipient),
            );
            param.insert("start_time", format!("{}", start_time));
            events.push(param);
        }
        MarketplaceEvent::BuyOrderCreated {
            creator,
            collection,
            token_id,
            pay_token,
            price,
            additional_recipient,
            start_time,
        } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("event_type", "BuyOrderCreated".to_string());
            param.insert("creator", format!("{:?}", creator));
            param.insert("collection", collection.to_string());
            param.insert("token_id", format!("{}", token_id));
            param.insert("pay_token", format!("{:?}", pay_token));
            param.insert("price", format!("{}", price));
            param.insert(
                "additional_recipient",
                format!("{:?}", additional_recipient),
            );
            param.insert("start_time", format!("{}", start_time));
            events.push(param);
        }
        MarketplaceEvent::BuyOrderCanceled {
            creator,
            collection,
            token_id,
            start_time,
        } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("event_type", "BuyOrderCanceled".to_string());
            param.insert("creator", format!("{:?}", creator));
            param.insert("collection", collection.to_string());
            param.insert("token_id", format!("{}", token_id));
            param.insert("start_time", format!("{}", start_time));

            events.push(param);
        }
        MarketplaceEvent::BuyOrderAccepted {
            creator,
            collection,
            token_id,
            start_time,
            owner,
        } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("event_type", "BuyOrderAccepted".to_string());
            param.insert("creator", format!("{:?}", creator));
            param.insert("collection", collection.to_string());
            param.insert("token_id", format!("{}", token_id));
            param.insert("start_time", format!("{}", start_time));
            param.insert("owner", format!("{:?}", owner));

            events.push(param);
        }
        MarketplaceEvent::AcceptableTokenAdded { contract_hash, fee } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("contract_hash", contract_hash.to_string());
            param.insert("fee", format!("{}", fee));
            events.push(param);
        }
        MarketplaceEvent::AcceptableTokenRemoved { contract_hash } => {
            let mut param = BTreeMap::new();
            param.insert("contract_package_hash", contract_package_hash.to_string());
            param.insert("contract_hash", contract_hash.to_string());

            events.push(param);
        }
    }
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
