use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;

use crate::{
    marketplace::{deploy_with_nft, get_account_0},
    utility::marketplace_interface::MarketplaceInstance,
};

/**
 * add admin
 */
#[test]
fn should_add_admin() {
    let (mut builder, marketplace_contract_hash, _nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.add_admin(&mut builder, *DEFAULT_ACCOUNT_ADDR, get_account_0(), true);
}

/**
 * add admin
 * should fail, no admin
 */
#[test]
fn should_not_add_admin() {
    let (mut builder, marketplace_contract_hash, _nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.add_admin(&mut builder, get_account_0(), get_account_0(), false);
}

/**
 * remove admin
 */
#[test]
fn should_remove_admin() {
    let (mut builder, marketplace_contract_hash, _nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.remove_admin(&mut builder, *DEFAULT_ACCOUNT_ADDR, get_account_0(), true);
}

/**
 * remove admin
 * should fail, no admin
 */
#[test]
fn should_not_remove_admin() {
    let (mut builder, marketplace_contract_hash, _nft_contract_hash) = deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.remove_admin(&mut builder, get_account_0(), get_account_0(), false);
}
