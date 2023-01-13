use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::U256;

use crate::{marketplace::deploy_with_nft, utility::marketplace_interface::MarketplaceInstance};

/**
 * add listing
 * should fail due to non existent token id
 */
#[test]
fn should_add_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(10u64),
        true,
    );
}

/**
 * add listing
 * should fail due to non existent token id
 */
#[test]
fn should_not_add_listing_0() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        5u64,
        erc20_contract_hash,
        U256::from(10u64),
        false,
    );
}

/**
 * add listing
 * should fail, price is 0
 */
#[test]
fn should_not_add_listing_1() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(0u64),
        false,
    );
}
