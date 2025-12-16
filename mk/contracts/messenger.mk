messenger-deploy:
	$(DEPLOY) $(MESSENGER_WASM_PATH) > $(MESSENGER_ADDRESS_PATH) && echo $(MESSENGER_ADDRESS)

messenger-initialize:
	$(INVOKE) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		initialize \
			--admin $(ADMIN)\
			--chain_id 7 \
			--native_token_address $(NATIVE_ADDRESS)\
			--other_chain_ids 0001010101010100010101010101010101000000000000000000000000000000 \
			--gas_oracle_address $(GAS_ORACLE_ADDRESS)\
			--primary_validator_key 04734fc43dde79306ddf1bd5b4840d2cc9195bb48dbef92d081e71805694f5828d9cce76008f1f1a2a8a6ccd564b84937f83630d3d3af9541a5a3f3c1c1ea62c98 \
			--secondary_validator_keys '{ "04734fc43dde79306ddf1bd5b4840d2cc9195bb48dbef92d081e71805694f5828d9cce76008f1f1a2a8a6ccd564b84937f83630d3d3af9541a5a3f3c1c1ea62c98": true }'

messenger-set-gas-usage:
	$(INVOKE) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		set_gas_usage \
		--chain_id 16\
        --gas_usage 100000

define messenger-set-gas-usage-param
	stellar contract invoke \
    		--id $(MESSENGER_ADDRESS) \
    		--source $(ADMIN_ALIAS) \
    		--network $(NETWORK) 	\
    		-- \
    		set_gas_usage \
    		--chain_id $1\
            --gas_usage $2
endef

messenger-set-all-gas-usage:
	$(call messenger-set-gas-usage-param,2,100000)
	$(call messenger-set-gas-usage-param,3,100000)
	$(call messenger-set-gas-usage-param,4,1300)
	$(call messenger-set-gas-usage-param,5,100000)
	$(call messenger-set-gas-usage-param,6,100000)
	$(call messenger-set-gas-usage-param,10,100000)
	$(call messenger-set-gas-usage-param,13,4000)
	$(call messenger-set-gas-usage-param,15,8000)
	$(call messenger-set-gas-usage-param,60,100000)

messenger-send-message:
	$(INVOKE) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		send_message \
		--message 0701efefefefefefefefefefefefefefefefefefefefefefefefefefefefefef \
		--sender $(ALICE)

messenger-receive_message:
	$(INVOKE) \
		--id $(MESSENGER_ADDRESS) \
		--cost \
		-- \
		receive_message \
		--message 02071be378bf6338f627121187e82e54fcf64577ff705e2ee3fc12a930a43bd3 \
				--primary_signature 807131e734509d76ad48da6a075a93257897d51a49e2b6dc53e945a313dbb1f10e1fbb57050a156a29976be1a3dd3f87d6cca948eb4bd1d34328e3c58a7d0032 \
				--primary_recovery_id 0 \
				--secondary_signature 807131e734509d76ad48da6a075a93257897d51a49e2b6dc53e945a313dbb1f10e1fbb57050a156a29976be1a3dd3f87d6cca948eb4bd1d34328e3c58a7d0032 \
				--secondary_recovery_id 0

messenger-get-gas-usage:
	$(INVOKE_VIEW) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		get_gas_usage \
		--chain_id 2

messenger-get-transaction-cost:
	$(INVOKE_VIEW) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		get_transaction_cost \
		--chain_id 2

messenger-has-received-message:
	$(INVOKE_VIEW) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		has_received_message \
		--message 0207a3508a81ab1b0043a51568079044f4e34648226124dccd21f5d89c51f3fb

messenger-has-sent-message:
	$(INVOKE_VIEW) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		has_sent_message \
		--message 020777b64e53254cc42d1d695036cf5f438312735b915adec350b68ff713c997

messenger-install:
	stellar contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(MESSENGER_WASM_PATH)

messenger-update-contract:
	$(INVOKE) \
		--id $(MESSENGER_ADDRESS) \
		-- \
		upgrade \
			--new_wasm_hash 5accf5d0f95f58fa341bc6ac968908bd2ebc864cb9bf6eeda6c799022cde1d45

messenger-restore-contract:
	stellar contract restore \
	--id $(MESSENGER_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679

