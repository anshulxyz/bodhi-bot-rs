.PHONY: all
all: format clippy build test run

.PHONY: format
format:
	cargo fmt

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: run
run:
	cargo run
