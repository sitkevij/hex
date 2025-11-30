# http://www.gnu.org/software/make/manual/make.html#Special-Variables
.DEFAULT_GOAL := release

# http://www.gnu.org/software/make/manual/make.html#Phony-Targets
.PHONY: all fmt fmt-check debug release test doc example cargo-install-tools python-install-tools publish-dry-run geiger tarpaulin grcov install install-force docker-build docker-run docker-test docker-test-lib docker-test-all docker-test-ubuntu deb manpage lint lint-clippy lint-format lint-markdown lint-spell clean

TARGET_DIR = target
DEBUG_DIR = $(TARGET_DIR)/debug
RELEASE_DIR = $(TARGET_DIR)/release
RLS_DIR = $(TARGET_DIR)/rls
INSTALL_DIR = /usr/local/bin
BINARY = hx

all: fmt test clean

fmt:
	cargo fmt --all --verbose

fmt-check:
	cargo fmt --all -- --check

debug:
	RUSTFLAGS="" cargo build

release: test
	cargo build --release

test:
	cargo test --verbose --all -- --nocapture

doc: ## Generate Rust documentation
	cargo doc --no-deps --open

example:
	cargo run --example simple

cargo-install-tools:
	cargo install cargo-bloat
	cargo install cargo-deb
	cargo install cargo-geiger
	cargo install cargo-trend
	cargo install cargo-show
	cargo install cargo-outdated
	cargo install cargo-edit
	cargo install --list

python-install-tools:
	pip install codespell

publish-dry-run:
	cargo publish --dry-run
	cargo package --list

geiger:
	cargo geiger

tarpaulin:
	# use docker as tarpaulin only supports x86_64 processors running linux
	docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin
	open tarpaulin-report.html

grcov:
	# grcov requires rust nightly for now
	rm -rf target/debug/
	# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
	export CARGO_INCREMENTAL=0 && \
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" && \
	export RUSTDOCFLAGS="-Cpanic=abort" && \
	cargo +nightly build
	cargo +nightly test --verbose
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
	open target/debug/coverage/index.html

install: release debug
	cargo install --path . 
	## cp $(RELEASE_DIR)/$(BINARY) $(INSTALL_DIR)/$(BINARY)

install-force: clean release debug
	cargo install --path . --force

docker-build:
	docker build -t sitkevij/hx:latest .

docker-run:
	cat README.md | docker run -i sitkevij/hx:latest

docker-test: ## Run tests in Docker using Rust image (fastest)
	docker run --rm -v "${PWD}:/workspace" -w /workspace rust:latest bash -c "cargo build && cargo test"

docker-test-lib: ## Run library tests only in Docker using Rust image (no binary needed)
	docker run --rm -v "${PWD}:/workspace" -w /workspace rust:latest cargo test --lib

docker-test-all: ## Run all tests (including integration tests) in Docker
	docker run --rm -v "${PWD}:/workspace" -w /workspace rust:latest bash -c "cargo build && cargo test --verbose --all"

docker-test-ubuntu: ## Run tests in Docker using Ubuntu image (more comprehensive)
	docker run --rm -v "${PWD}:/workspace" -w /workspace ubuntu:latest bash -c 'apt-get update -qq && apt-get install -y -qq curl build-essential >/dev/null 2>&1 && curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y >/dev/null 2>&1 && export PATH="$$HOME/.cargo/bin:$$PATH" && cargo build && cargo test'

deb:
	cargo deb

manpage: debug
	target/debug/hx --help >target/debug/hx.1.txt
	pandoc MANPAGE.md -s -t man
	HELP=$$(cat target/debug/hx.1.txt); \
	echo "$$HELP"
	MANPAGE=$$(cat MANPAGE.md); \
	# echo $$MANPAGE | sed 's/$$/\\n/g' | tr -d'\n'
	pandoc --standalone --to man MANPAGE.md -o hx.1
	cp hx.1 /usr/local/share/man/man1
	man hx

lint: lint-clippy lint-format lint-markdown lint-spell

lint-clippy:
	cargo clippy --workspace --all-targets --verbose
	cargo clippy --workspace --all-targets --verbose --no-default-features
	cargo clippy --workspace --all-targets --verbose --all-features

lint-format:
	cargo fmt --all -- --check

lint-markdown:
	markdownlint-cli2 --config .markdownlint.json *.md

lint-spell:
	codespell -L crate -w src/*.rs *.md tests/*.rs *.toml

clean: ## Remove all artifacts
	rm -rf $(DEBUG_DIR)
	rm -rf $(RELEASE_DIR)
	rm -rf $(RLS_DIR)
