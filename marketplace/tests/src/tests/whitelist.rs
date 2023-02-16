use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;

use crate::{
    marketplace::{deploy_with_nft, get_account_0},
    utility::marketplace_interface::MarketplaceInstance,
};

/**
 * whitelist contract
 */
#[test]
fn should_whitelist_nft() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.whitelist(&mut builder, *DEFAULT_ACCOUNT_ADDR, nft_contract_hash, true);
}

/**
 * whitelist contract
 * should fail, no admin
 */
#[test]
fn should_not_whitelist_nft() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.whitelist(&mut builder, get_account_0(), nft_contract_hash, false);
}

/**
 * delist contract
 */
#[test]
fn should_delist_nft() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.delist(&mut builder, *DEFAULT_ACCOUNT_ADDR, nft_contract_hash, true);
}

/**
 * delist contract
 * should fail, no admin
 */
#[test]
fn should_not_delist_nft() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.delist(&mut builder, get_account_0(), nft_contract_hash, false);
}
