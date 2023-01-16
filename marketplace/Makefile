prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	cd client/execute_listing_session && cargo build --release --target wasm32-unknown-unknown
	wasm-strip ../target/wasm32-unknown-unknown/release/marketplace_contract.wasm 2>/dev/null | true
	wasm-strip client/execute_listing_session/target/wasm32-unknown-unknown/release/execute_listing_call.wasm 2>/dev/null | true

test: build-contract
	mkdir -p tests/wasm
	cp ../target/wasm32-unknown-unknown/release/marketplace_contract.wasm tests/wasm/marketplace.wasm
	cp ../target/wasm32-unknown-unknown/release/execute_listing_call.wasm tests/wasm/execute_listing_call.wasm
	cd tests && cargo test

clippy:
	cd client/execute_listing_session && cargo clippy --target wasm32-unknown-unknown -- -D warnings
	cd contract && cargo clippy --target wasm32-unknown-unknown -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd client && cargo fmt -- --check
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd client/execute_listing_session && cargo fmt --target wasm32-unknown-unknown
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
