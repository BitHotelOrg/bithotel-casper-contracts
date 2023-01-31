PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}

build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	cd client/mint_session && cargo build --release --target wasm32-unknown-unknown
	cd client/balance_of_session && cargo build --release --target wasm32-unknown-unknown
	cd client/owner_of_session && cargo build --release --target wasm32-unknown-unknown
	cd client/get_approved_session && cargo build --release --target wasm32-unknown-unknown
	cd client/transfer_session && cargo build --release --target wasm32-unknown-unknown
	cd client/updated_receipts && cargo build --release --target wasm32-unknown-unknown
	cd test-contracts/minting_contract && cargo build --release --target wasm32-unknown-unknown
	cd test-contracts/mangle_named_keys && cargo build --release --target wasm32-unknown-unknown
	wasm-strip ../target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
	wasm-strip ../target/wasm32-unknown-unknown/release/mint_call.wasm 2>/dev/null | true
	wasm-strip ../target/wasm32-unknown-unknown/release/balance_of_call.wasm 2>/dev/null | true
	wasm-strip ../target/wasm32-unknown-unknown/release/owner_of_call.wasm 2>/dev/null | true
	wasm-strip ../target/wasm32-unknown-unknown/release/get_approved_call.wasm 2>/dev/null | true
	wasm-strip ../target/wasm32-unknown-unknown/release/transfer_call.wasm 2>/dev/null | true
	wasm-strip ../target/wasm32-unknown-unknown/release/updated_receipts.wasm 2>/dev/null | true
	wasm-strip ../target/wasm32-unknown-unknown/release/minting_contract.wasm 2>/dev/null | true

setup-test: build-contract
	mkdir -p tests/wasm
	mkdir -p tests/wasm/1_0_0; curl -L https://github.com/casper-ecosystem/cep-78-enhanced-nft/releases/download/v1.0.0/cep-78-wasm.tar.gz | tar zxv -C tests/wasm/1_0_0/
	cp ../target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/mint_call.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/balance_of_call.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/owner_of_call.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/get_approved_call.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/transfer_call.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/updated_receipts.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/minting_contract.wasm tests/wasm
	cp ../target/wasm32-unknown-unknown/release/mangle_named_keys.wasm tests/wasm

test: setup-test
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
