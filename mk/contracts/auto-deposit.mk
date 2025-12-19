auto-deposit-wallet-upload: build-auto-deposit-wallet
	stellar contract upload \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) \
		--wasm $(AUTO_DEPOSIT_WALLET_WASM_PATH) \
		> $(AUTO_DEPOSIT_WALLET_WASM_HASH_PATH) && echo $(AUTO_DEPOSIT_WALLET_WASM_HASH)

auto-deposit-factory-upload: build-auto-deposit-factory
	stellar contract upload \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) \
		--wasm $(AUTO_DEPOSIT_FACTORY_WASM_PATH)

deploy-auto-deposit-factory: build-auto-deposit-factory auto-deposit-wallet-upload
	$(DEPLOY) $(AUTO_DEPOSIT_FACTORY_WASM_PATH) \
		-- \
		--admin $(ADMIN) \
		--native_token_address $(NATIVE_ADDRESS) \
		--gas_oracle_address $(GAS_ORACLE_ADDRESS) \
		--bridge $(BRIDGE_ADDRESS) \
		--send_tx_cost 10000000 \
		--wallet_wasm_hash $(AUTO_DEPOSIT_WALLET_WASM_HASH) \
		> $(AUTO_DEPOSIT_FACTORY_ADDRESS_PATH) && echo $(AUTO_DEPOSIT_FACTORY_ADDRESS)

auto-deposit-factory-create-deposit-wallet:
	$(INVOKE) \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		-- \
		create_deposit_wallet \
			--sender GD4A45PEPZBWYBDEQZYHDSEDML76J4JJTUTO2FHYFCKLD5O3YXOY6QIK \
			--recipient GD4A45PEPZBWYBDEQZYHDSEDML76J4JJTUTO2FHYFCKLD5O3YXOY6QIK \
			--recipient_token $(USDY_ADDRESS) \
			--min_deposit_amount 1 \
			--fee_token_amount 140000000 \
			--chain-ids [2]

auto-deposit-factory-deploy-deposit-wallet:
	$(INVOKE) \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		-- \
		deploy_deposit_wallet \
			--recipient_chain_id 2 \
			--recipient 0000000000000000000000009ACA1C932640A5743B777162D6D3C6196053596E \
			--recipient_token 0000000000000000000000001C7D4B196CB0C7B01D743FBC6116A902379C7238 \
			--min_deposit_amount 1

auto-deposit-factory-set-wallet-wasm-hash:
	$(INVOKE) \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		-- \
		set_wallet_wasm_hash \
			--wallet_wasm_hash $(AUTO_DEPOSIT_WALLET_WASM_HASH)

auto-deposit-factory-swap-and-bridge:
	$(INVOKE) \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		-- \
		swap_and_bridge \
			--wallet_address CBPZUXF4AKG2HZGFI4MT3GDJGTZQPGPQKCLUZE6KGLOYQMOFSMOOK73W \
			--token_address $(USDY_ADDRESS) \
			--nonce 241928 

auto-deposit-factory-update-contract:
	$(INVOKE) \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
			-- \
			upgrade \
			--new_wasm_hash db345359017e405e529afebf51764e46bff3b5ed7ea9fbfdbea49eac5b232c01

define auto-deposit-factory-register-token
	stellar contract invoke \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) \
		-- \
		register_token \
		--token_address $1
endef

auto-deposit-factory-register-all-tokens:
	$(call auto-deposit-factory-register-token,$(USDY_ADDRESS))
	$(call auto-deposit-factory-register-token,$(USDC_ADDRESS))
	$(call auto-deposit-factory-register-token,$(YARO_ADDRESS))

define auto-deposit-factory-set-gas-usage
	stellar contract invoke \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) \
		-- \
		set_gas_usage \
		--chain_id $1 \
		--gas-usage $2
endef

auto-deposit-factory-set-all-gas-usages:
	$(call auto-deposit-factory-set-gas-usage,2,250000)
	$(call auto-deposit-factory-set-gas-usage,3,150000)
	$(call auto-deposit-factory-set-gas-usage,4,3500)
	$(call auto-deposit-factory-set-gas-usage,5,250000)
	$(call auto-deposit-factory-set-gas-usage,6,250000)

