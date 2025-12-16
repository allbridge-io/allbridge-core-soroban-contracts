.DEFAULT_GOAL := all

all: build-auto-deposit

include mk/commands.mk
include mk/vars.mk
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
		--network $(NETWORK) \
		--output-dir ./types/$1 \
		--contract-id $2
endef

generate-all-types:
	rm -rf ./types/
	$(call generate-types,gas-oracle,$(GAS_ORACLE_ADDRESS))
	$(call generate-types,bridge,$(BRIDGE_ADDRESS))
	$(call generate-types,messenger,$(MESSENGER_ADDRESS))
	$(call generate-types,pool,$(POOL_ADDRESS))
	$(call generate-types,token,$(TOKEN_ADDRESS))
	$(call generate-types,auto-deposit-factory,$(AUTO_DEPOSIT_FACTORY_ADDRESS))

install-cli:
	brew install stellar-cli
