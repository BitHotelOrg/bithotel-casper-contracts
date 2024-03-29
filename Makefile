PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	cd cep78 && rustup target add wasm32-unknown-unknown && \
		rustup component add clippy --toolchain ${PINNED_TOOLCHAIN} && \
		rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}
	cd marketplace && rustup target add wasm32-unknown-unknown && \
		rustup component add clippy --toolchain ${PINNED_TOOLCHAIN} && \
		rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}

test: build-contract
	cd cep78 && make test
	cd marketplace && make test

prepare:
	cd cep78 && rustup target add wasm32-unknown-unknown
	cd marketplace && rustup target add wasm32-unknown-unknown

check-lint:
	cd cep78 && make check-lint
	cd marketplace && make check-lint

build-contract:
	cd cep78 && make build-contract
	cd marketplace && make build-contract