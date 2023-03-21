use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, system::mint, RuntimeArgs, U256};

use crate::{
    marketplace::{approve_nft, deploy_with_nft, get_account_0},
    utility::{
        constants::CEP78, helpers::transfer_nft, marketplace_interface::MarketplaceInstance,
    },
};

/**
 * should pause
 */
#[test]
fn should_pause() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.pause(&mut builder, *DEFAULT_ACCOUNT_ADDR, true);
    marketplace.add_listing(
        &mut builder,
        get_account_0(),
        nft_contract_hash,
        0u64,
        U256::from(5u64),
        false,
    );
}

/**
 * should not pause
 * no admin
 */
#[test]
fn should_not_pause() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
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
    marketplace.pause(&mut builder, get_account_0(), false);
}

/**
 * should not pause
 * no admin
 */
#[test]
fn should_pause_and_unpause() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
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

    transfer_nft(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        get_account_0().into(),
        0u64,
    );
    approve_nft(
        &mut builder,
        get_account_0(),
        nft_contract_hash,
        marketplace_contract_hash.into(),
        0u64,
    );

    marketplace.pause(&mut builder, *DEFAULT_ACCOUNT_ADDR, true);
    marketplace.add_listing(
        &mut builder,
        get_account_0(),
        nft_contract_hash,
        0u64,
        U256::from(5u64),
        false,
    );
    marketplace.un_pause(&mut builder, *DEFAULT_ACCOUNT_ADDR, true);
    marketplace.add_listing(
        &mut builder,
        get_account_0(),
        nft_contract_hash,
        0u64,
        U256::from(5u64),
        true,
    );
}
