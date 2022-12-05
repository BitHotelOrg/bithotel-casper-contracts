use casper_engine_test_support::{InMemoryWasmTestBuilder, WasmTestBuilder};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
use casper_types::{account::AccountHash, bytesrepr::FromBytes, CLTyped, ContractHash, Key};

pub(crate) fn get_contract_hash(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    account: AccountHash,
    contract_name: &str,
) -> ContractHash {
    let contract_hash = builder
        .get_expected_account(account)
        .named_keys()
        .get(contract_name)
        .expect("must have contract hash key as part of contract creation")
        .into_hash()
        .map(|hash| ContractHash::new(hash))
        .expect("must get contract hash");
    contract_hash
}

pub(crate) fn query_stored_value<T: CLTyped + FromBytes>(
    builder: &mut InMemoryWasmTestBuilder,
    base_key: Key,
    path: Vec<String>,
) -> T {
    builder
        .query(None, base_key, &path)
        .expect("must have stored value")
        .as_cl_value()
        .cloned()
        .expect("must have cl value")
        .into_t::<T>()
        .expect("must get value")
}
