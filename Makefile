format:
	cargo fmt

clippy:
	cargo clippy


build:
	cargo build


test:
	cargo test

run:
	cargo run

all:
	format clippy build test run
