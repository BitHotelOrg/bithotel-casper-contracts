test:
	cd cep78 && make test

prepare:
	rustup target add wasm32-unknown-unknown

check-lint:
	cd cep78 && make check-lint