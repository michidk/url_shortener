.PHONY: run build check test clippy fmt lint cic clean

# run and compile
run:
	cargo +nightly run

build:
	cargo +nightly build

build-release:
	cargo +nightly build --release

# test and lint
check:
	cargo +nightly check --all

test:
	cargo +nightly test --all

clippy:
	cargo +nightly clippy -- -D warnings

fmt:
	cargo +nightly fmt --all -- --check

# utility
lint: fmt clippy

## can i commit?
cic: test lint

clean:
	cargo clean
