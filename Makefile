.DEFAULT_GOAL := all

all: build-auto-deposit

include mk/vars.mk
include mk/commands.mk
include mk/build.mk
include mk/utils.mk

include mk/contracts/auto-deposit.mk
include mk/contracts/gas-oracle.mk
include mk/contracts/bridge.mk
include mk/contracts/messenger.mk
include mk/contracts/pool.mk

test: all
	CARGO_INCREMENTAL=0 cargo test

define generate-types
	stellar contract bindings typescript \
		--output-dir ./types/$1 \
		--wasm ./target/wasm32v1-none/release/$2.wasm
	@echo "\n"
endef

generate-all-types:
	rm -rf ./types/
	$(call generate-types,gas-oracle,gas_oracle)
	$(call generate-types,bridge,bridge)
	$(call generate-types,messenger,messenger)
	$(call generate-types,pool,pool)
	$(call generate-types,auto-deposit-factory,auto_deposit_factory)
	stellar contract bindings typescript \
		--network $(NETWORK) \
		--output-dir ./types/token \
		--contract-id $(TOKEN_ADDRESS)

install-cli:
	brew install stellar-cli
