deploy-gas-oracle:
	$(DEPLOY) $(GAS_ORACLE_WASM_PATH) > $(GAS_ORACLE_ADDRESS_PATH) && echo $(GAS_ORACLE_ADDRESS)

gas-oracle-init:
	$(INVOKE) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		initialize \
		--admin $(ADMIN)

gas-oracle-set-price:
	$(INVOKE) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		set_price \
		--chain_id 7 \
        --price 136000000000000000 \
        --gas_price 50

gas-oracle-set-price-1:
	$(INVOKE) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		set_price \
		--chain_id 1 \
        --price 0 \
        --gas_price 0

gas-oracle-get-price-data:
	$(INVOKE_VIEW) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		get_gas_price \
		--chain_id 2

gas-oracle-get-price:
	$(INVOKE_VIEW) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		get_price \
			--chain_id 7

gas-oracle-get-admin:
	$(INVOKE_VIEW) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		get_admin

gas-oracle-get-gas-cost-in-native-token:
	$(INVOKE_VIEW) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		get_gas_cost_in_native_token \
		--other_chain_id 2 \
		--gas_amount 250000

gas-oracle-get-transaction-gas-cost-in-usd:
	$(INVOKE_VIEW) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		get_transaction_gas_cost_in_usd \
		--other_chain_id 1 \
		--gas_amount 1

gas-oracle-crossrate:
	$(INVOKE_VIEW) \
		--id $(GAS_ORACLE_ADDRESS) \
		-- \
		crossrate \
		--other_chain_id 1

gas-oracle-install:
	$(INSTALL) $(GAS_ORACLE_WASM_PATH)

gas-oracle-update-contract:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
        -- \
        upgrade \
        --new_wasm_hash a16f4b45aff547518e6c1bb2ffe2e26b232562edb2405fd9be413e31d495098d

gas-oracle-restore-contract:
	stellar contract restore \
	--id $(GAS_ORACLE_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) \
	--durability persistent \
	--ledgers-to-extend 535679

