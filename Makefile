SHELL := /usr/bin/env bash

.PHONY: test check package-check release-local deb rpm

test:
	cargo test

check:
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo test

package-check:
	cargo package --allow-dirty
	cargo publish --dry-run

release-local:
	cargo build --release --target x86_64-unknown-linux-gnu
	cross build --release --target x86_64-unknown-linux-musl
	cross build --release --target aarch64-unknown-linux-gnu
	cross build --release --target aarch64-unknown-linux-musl
	cross build --release --target x86_64-pc-windows-gnu

deb:
	cargo deb

rpm:
	cargo generate-rpm
