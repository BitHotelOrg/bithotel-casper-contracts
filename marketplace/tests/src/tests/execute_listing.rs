use crate::{
    marketplace::{deploy_with_nft, get_account_0},
    utility::{helpers::nft_get_balance, marketplace_interface::MarketplaceInstance},
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, system::mint, Key, RuntimeArgs, U256, U512};

/**
 * add listing
 * transfer funds to buyer's purse
 * execute listing
 * assert nft balances of buyer and seller
 * assert purse balance buyer
 */
#[test]
fn should_execute_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);
    let marketplace = MarketplaceInstance {
        contract_hash: marketplace_contract_hash,
    };
    let price = 10_000_000_000u64; // 10
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(price),
        true,
    );
    let fund_account_request = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            mint::ARG_AMOUNT => 10_000_000_000_000u64, // 10.000
            mint::ARG_TARGET => get_account_0(),
            mint::ARG_ID => Option::<u64>::None,
        },
    )
    .build();
    builder.exec(fund_account_request).expect_success().commit();

    let buyer_account = builder.get_account(get_account_0()).unwrap();
    let buyer_purse_balance_before = builder.get_purse_balance(buyer_account.main_purse());
    let fee =
        marketplace.execute_listing(&mut builder, get_account_0(), 1u64, U512::from(price), true);
    let buyer_purse_balance_after = builder.get_purse_balance(buyer_account.main_purse());

    let nft_balance_seller = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(nft_balance_seller, 0);
    let nft_balance_buyer = nft_get_balance(
        &mut builder,
        get_account_0(),
        nft_contract_hash,
        get_account_0().into(),
    );
    assert_eq!(nft_balance_buyer, 1);
    assert_eq!(
        buyer_purse_balance_before,
        buyer_purse_balance_after + price + fee
    );
}

/**
 * execute listing
 * should fail because there is no listing
 */
#[test]
fn should_not_execute_listing_0() {
    let (mut builder, marketplace_contract_hash, _nft_contract_hash, _erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);

    marketplace.execute_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        1u64,
        U512::from(10u64),
        false,
    );
}

/**
 * add listing
 * execute listing
 * should fail because listing owner cannot buy own listing
 */
#[test]
fn should_not_execute_listing_1() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance {
        contract_hash: marketplace_contract_hash,
    };

    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(100u64),
        true,
    );

    let mut balance = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );

    assert_eq!(balance, 0);

    marketplace.execute_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        1u64,
        U512::from(10u64),
        false,
    );

    balance = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );

    assert_eq!(balance, 0);
}

/**
 * add listing
 * transfer too little funds to buyer's purse
 * execute listing, should fail due to insufficient balance
 * asset seller balance, should be unaffected
 */
#[test]
fn should_not_execute_listing_2() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);
    let marketplace = MarketplaceInstance {
        contract_hash: marketplace_contract_hash,
    };
    let price = 10u64;
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(price),
        true,
    );
    let fund_account_request = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            mint::ARG_AMOUNT => 5_000_000_000u64, // 50
            mint::ARG_TARGET => get_account_0(),
            mint::ARG_ID => Option::<u64>::None,
        },
    )
    .build();
    builder.exec(fund_account_request).expect_success().commit();

    let default_account = builder.get_account(*DEFAULT_ACCOUNT_ADDR).unwrap();
    let seller_purse_balance_before = builder.get_purse_balance(default_account.main_purse());
    marketplace.execute_listing(
        &mut builder,
        get_account_0(),
        1u64,
        U512::from(price),
        false,
    );
    let seller_purse_balance_after = builder.get_purse_balance(default_account.main_purse());
    let fund_account_request = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            mint::ARG_AMOUNT => 10_000_000_000_000u64, // 10.000
            mint::ARG_TARGET => get_account_0(),
            mint::ARG_ID => Option::<u64>::None,
        },
    )
    .build();
    builder.exec(fund_account_request).expect_success().commit();
    let nft_balance_buyer = nft_get_balance(
        &mut builder,
        get_account_0(),
        nft_contract_hash,
        get_account_0().into(),
    );
    assert_eq!(nft_balance_buyer, 0);
    assert_eq!(seller_purse_balance_before, seller_purse_balance_after);
}
