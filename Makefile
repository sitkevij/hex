# http://www.gnu.org/software/make/manual/make.html#Special-Variables
.DEFAULT_GOAL := release

# http://www.gnu.org/software/make/manual/make.html#Phony-Targets
.PHONY: clean docker

TARGET_DIR = target
DEBUG_DIR = $(TARGET_DIR)/debug
RELEASE_DIR = $(TARGET_DIR)/release
RLS_DIR = $(TARGET_DIR)/rls
INSTALL_DIR = /usr/local/bin
BINARY = hx

all: fmt test clean

fmt:
	cargo fmt --verbose

debug:
	cargo build

release: test
	cargo build --release

test:
	cargo test --verbose --all -- --nocapture

install: release debug test
	cargo install --path . 
	## cp $(RELEASE_DIR)/$(BINARY) $(INSTALL_DIR)/$(BINARY)

install-force: clean release debug test
	cargo install --path . --force

clippy:
	cargo clippy

docker:
	docker build -t sitkevij/stretch-slim:$(BINARY)-0.2.0 .

clean: ## Remove all artifacts
	rm -rf $(DEBUG_DIR)
	rm -rf $(RELEASE_DIR)
	rm -rf $(RLS_DIR)
