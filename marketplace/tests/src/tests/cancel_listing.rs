use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{Key, U256};

use crate::{
    marketplace::deploy_with_nft,
    utility::{helpers::nft_get_balance, marketplace_interface::MarketplaceInstance},
};

/**
 * add listing --> sends nft to marketplace
 * cancel listing --> receives nft from marketplace
 * assert balance is 1 nft again
 */
#[test]
fn should_add_and_cancel_listing() {
    let (mut builder, marketplace_contract_hash, nft_contract_hash, erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);
    marketplace.add_listing(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        0u64,
        erc20_contract_hash,
        U256::from(100u64),
        true,
    );

    marketplace.cancel_listing(&mut builder, *DEFAULT_ACCOUNT_ADDR, 1u64, true);

    let balance_of_account = nft_get_balance(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        Key::from(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance_of_account, 1u64);
}

/**
 * cancel listing
 * should fail because the listing does not exist
 */
#[test]
fn should_not_cancel_listing() {
    let (mut builder, marketplace_contract_hash, _nft_contract_hash, _erc20_contract_hash) =
        deploy_with_nft(true);

    let marketplace = MarketplaceInstance::new(marketplace_contract_hash);

    marketplace.cancel_listing(&mut builder, *DEFAULT_ACCOUNT_ADDR, 1u64, false);
}
