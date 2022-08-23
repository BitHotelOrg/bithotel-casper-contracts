use casper_types::{
    account::AccountHash, runtime_args, ContractHash, Key, RuntimeArgs, U256, U512,
};
use kunftmarketplace_contract::Address;
use std::{collections::BTreeMap, path::PathBuf, vec};
use test_env::{utils::DeploySource, TestEnv};

use crate::{
    cep47_instance::{CEP47Instance, Meta, TokenId},
    erc20_instance::ERC20Instance,
    marketplace_instance::MarketplaceInstance,
};

const PER_BUY_SELL_ORDER_CSPR_WASM: &str = "pre_buy_sell_order_cspr.wasm";

struct TestContext {
    marketplace: MarketplaceInstance,
    nft: CEP47Instance,
    erc20: ERC20Instance,
}

mod meta {
    use super::{BTreeMap, Meta};
    pub fn contract_meta() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("origin".to_string(), "fire".to_string());
        meta
    }

    pub fn red_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "red".to_string());
        meta
    }

    pub fn _blue_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "blue".to_string());
        meta
    }

    pub fn _black_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "black".to_string());
        meta
    }

    pub fn _gold_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "gold".to_string());
        meta
    }
}

fn deploy() -> (TestEnv, TestContext, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let erc20 = ERC20Instance::new(
        &env,
        "USDT",
        owner,
        "USDT",
        9,
        U256::from(10_000u64).checked_mul(U256::exp10(9)).unwrap(),
    );

    let mut acceptable_tokens: BTreeMap<String, u32> = BTreeMap::new();
    let null_contract_hash = ContractHash::new([0u8; 32]);
    acceptable_tokens.insert(null_contract_hash.to_formatted_string(), 1000);
    acceptable_tokens.insert(erc20.contract_hash().to_formatted_string(), 500);

    let marketplace = MarketplaceInstance::new(
        &env,
        "kunft_marketplace",
        owner,
        acceptable_tokens,
        Address::from(owner),
    );

    let nft = CEP47Instance::new(&env, "kunft", owner, "KUNFT", "KNFT", meta::contract_meta());

    let test_context = TestContext {
        marketplace,
        nft,
        erc20,
    };

    (env, test_context, owner)
}

#[test]
fn should_deploy() {
    let _ = deploy();
}

#[test]
fn should_create_sell_order_and_buy_cspr() {
    let (env, test_context, owner) = deploy();
    let user = env.next_user();
    let token_0 = TokenId::zero();
    let token_1 = TokenId::one();
    let token_meta = meta::red_dragon();
    let nft = test_context.nft;
    let marketplace = test_context.marketplace;

    nft.mint_copies(owner, user, vec![token_0, token_1], token_meta, 2);

    nft.approve(
        user,
        Address::from(marketplace.contract_package_hash()),
        vec![token_0, token_1],
    );

    let pay_token: Option<String> = None;
    let price_0 = U256::from(50u8).checked_mul(U256::exp10(9)).unwrap();
    let price_1 = U256::from(100u8).checked_mul(U256::exp10(9)).unwrap();
    let mut tokens: BTreeMap<TokenId, U256> = BTreeMap::new();
    tokens.insert(token_0, price_0);
    tokens.insert(token_1, price_1);
    marketplace.create_sell_order(
        user,
        0u64,
        nft.contract_hash().to_formatted_string(),
        tokens,
        pay_token,
    );

    // buy token 0
    let buyer_0 = env.next_user();
    let session_code = PathBuf::from(PER_BUY_SELL_ORDER_CSPR_WASM);
    let price_u512_0 = U512::from(50u8).checked_mul(U512::exp10(9)).unwrap();
    let additional_recipient: Option<Address> = None;
    env.run(
        buyer_0,
        DeploySource::Code(session_code),
        runtime_args! {
            "marketplace_contract" => marketplace.contract_hash().to_formatted_string(),
            "collection" => nft.contract_hash().to_formatted_string(),
            "token_id" => token_0,
            "amount" => price_u512_0,
            "additional_recipient" => additional_recipient
        },
    );

    let nft_owner = nft.owner_of(token_0).unwrap();
    assert_eq!(nft_owner, Key::from(buyer_0));

    // buy token 1
    let buyer_1 = env.next_user();
    let session_code = PathBuf::from(PER_BUY_SELL_ORDER_CSPR_WASM);
    let price_u512_1 = U512::from(100u8).checked_mul(U512::exp10(9)).unwrap();
    let additional_recipient: Option<Key> = Some(Key::from(env.next_user()));
    env.run(
        buyer_1,
        DeploySource::Code(session_code),
        runtime_args! {
            "marketplace_contract" => marketplace.contract_hash().to_formatted_string(),
            "collection" => nft.contract_hash().to_formatted_string(),
            "token_id" => token_1,
            "amount" => price_u512_1,
            "additional_recipient" => additional_recipient
        },
    );

    let nft_owner = nft.owner_of(token_1).unwrap();
    assert_eq!(nft_owner, additional_recipient.unwrap());
}

#[test]
fn should_create_sell_order_and_buy() {
    let (env, test_context, owner) = deploy();

    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    let nft = test_context.nft;
    let marketplace = test_context.marketplace;
    nft.mint_one(owner, user, token_id, token_meta);

    nft.approve(
        user,
        Key::from(marketplace.contract_package_hash()),
        vec![token_id],
    );

    // Mint ERC20

    let pay_token: Option<String> = None;
    let price = U256::one();
    let mut tokens: BTreeMap<TokenId, U256> = BTreeMap::new();
    tokens.insert(token_id, price);
    marketplace.create_sell_order(
        user,
        0u64,
        nft.contract_hash().to_formatted_string(),
        tokens,
        pay_token,
    );
}

#[test]
fn should_create_sell_order_and_cancel() {
    let (env, test_context, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    let nft = test_context.nft;
    let marketplace = test_context.marketplace;
    nft.mint_one(owner, user, token_id, token_meta);

    nft.approve(
        user,
        Key::from(marketplace.contract_package_hash()),
        vec![token_id],
    );

    let pay_token: Option<String> = None;
    let price = U256::one();
    let mut tokens: BTreeMap<TokenId, U256> = BTreeMap::new();
    tokens.insert(token_id, price);
    marketplace.create_sell_order(
        user,
        0u64,
        nft.contract_hash().to_formatted_string(),
        tokens,
        pay_token,
    );

    marketplace.cancel_sell_order(
        user,
        nft.contract_hash().to_formatted_string(),
        vec![token_id],
    );
}

#[test]
fn should_create_buy_order_and_cancel() {
    let (env, test_context, owner) = deploy();
    let marketplace = test_context.marketplace;
    let nft = test_context.nft;
    let usdt = test_context.erc20;

    let ali = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft.mint_one(owner, ali, token_id, token_meta);

    let bob = env.next_user();
    let offer_amount = U256::from(90u8).checked_mul(U256::exp10(9)).unwrap();
    usdt.transfer(owner, Key::from(bob), offer_amount);
    usdt.approve(
        bob,
        Key::from(marketplace.contract_package_hash()),
        offer_amount,
    );
    let mut bob_usdt_balance = usdt.balance_of(Key::from(bob)).unwrap();
    assert_eq!(bob_usdt_balance, offer_amount);

    marketplace.create_buy_order(
        bob,
        nft.contract_hash().to_formatted_string(),
        token_id,
        None,
        usdt.contract_hash().to_formatted_string(),
        offer_amount,
    );
    bob_usdt_balance = usdt.balance_of(Key::from(bob)).unwrap();
    assert_eq!(bob_usdt_balance, U256::zero());

    marketplace.cancel_buy_order(bob, nft.contract_hash().to_formatted_string(), token_id);
}

#[test]
fn should_create_buy_order_and_accept() {
    let (env, test_context, owner) = deploy();
    let marketplace = test_context.marketplace;
    let nft = test_context.nft;
    let usdt = test_context.erc20;

    let ali = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft.mint_one(owner, ali, token_id, token_meta);

    let bob = env.next_user();
    let offer_amount = U256::from(90u8).checked_mul(U256::exp10(9)).unwrap();
    usdt.transfer(owner, Key::from(bob), offer_amount);
    usdt.approve(
        bob,
        Key::from(marketplace.contract_package_hash()),
        offer_amount,
    );
    let mut bob_usdt_balance = usdt.balance_of(Key::from(bob)).unwrap();
    assert_eq!(bob_usdt_balance, offer_amount);

    marketplace.create_buy_order(
        bob,
        nft.contract_hash().to_formatted_string(),
        token_id,
        None,
        usdt.contract_hash().to_formatted_string(),
        offer_amount,
    );
    bob_usdt_balance = usdt.balance_of(Key::from(bob)).unwrap();
    assert_eq!(bob_usdt_balance, U256::zero());
    nft.approve(
        ali,
        Key::from(marketplace.contract_package_hash()),
        vec![token_id],
    );
    marketplace.accept_buy_order(
        ali,
        nft.contract_hash().to_formatted_string(),
        token_id,
        Address::from(bob),
    );
    let token_owner = nft.owner_of(token_id).unwrap();
    assert_eq!(token_owner, Key::from(bob));
}

#[test]
fn should_set_fee_wallet() {
    let (env, test_context, owner) = deploy();
    let marketplace = test_context.marketplace;
    let user = env.next_user();

    marketplace.set_fee_wallet(owner, Key::from(user));
    let fee_wallet = marketplace.fee_wallet();
    println!("{:?}", user);
    assert_eq!(fee_wallet, Address::from(user))
}
