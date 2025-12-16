build-auto-deposit: build-auto-deposit-factory

build-auto-deposit-factory: build-bridge build-auto-deposit-wallet
	$(BUILD) auto-deposit-factory

build-auto-deposit-wallet:
	$(BUILD) auto-deposit-wallet

build-gas-oracle:
	$(BUILD) gas-oracle

build-messenger: build-gas-oracle
	$(BUILD) messenger

build-pool:
	$(BUILD) pool

build-bridge: build-messenger build-pool
	$(BUILD) bridge

