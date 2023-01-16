test:
	cd cep78 && make test
	cd marketplace && make test

prepare:
	rustup target add wasm32-unknown-unknown

check-lint:
	cd cep78 && make check-lint
	cd marketplace && make check-lint

build-contract:
	cd cep78 && make build-contract
	cd marketplace && make build-contract