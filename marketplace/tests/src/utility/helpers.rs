use casper_engine_test_support::{ExecuteRequestBuilder, WasmTestBuilder};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, Key,
    RuntimeArgs,
};

pub(crate) fn get_contract_hash(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    account: AccountHash,
    contract_name: &str,
) -> ContractHash {
    builder
        .get_expected_account(account)
        .named_keys()
        .get(contract_name)
        .expect("must have contract hash key as part of contract creation")
        .into_hash()
        .map(ContractHash::new)
        .expect("must get contract hash")
}

pub(crate) fn query_stored_value<T: CLTyped + FromBytes>(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
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

pub(crate) fn nft_get_balance(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    sender: AccountHash,
    nft_contract_hash: ContractHash,
    token_owner: Key,
) -> u64 {
    let key_name = "balance_of";
    let session_code_request = ExecuteRequestBuilder::standard(
        sender,
        "balance_of.wasm",
        runtime_args! {
            "nft_contract_hash" => Key::from(nft_contract_hash),
            "token_owner" => token_owner,
            "key_name" => key_name.to_string(),
        },
    )
    .build();

    builder.exec(session_code_request).expect_success().commit();
    query_stored_value::<u64>(builder, sender.into(), [key_name.to_string()].into())
}

pub(crate) fn transfer_nft(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    sender: AccountHash,
    nft_contract_hash: ContractHash,
    recipient: Key,
    token_id: u64,
) {
    let session_code_request = ExecuteRequestBuilder::contract_call_by_hash(
        sender,
        nft_contract_hash,
        "transfer",
        runtime_args! {
            "token_id" => token_id,
            "source_key" => Key::from(sender),
            "target_key" => recipient,
        },
    )
    .build();
    builder.exec(session_code_request).expect_success().commit();
}
