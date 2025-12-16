bridge-deploy:
	$(DEPLOY) $(BRIDGE_WASM_PATH) > $(BRIDGE_ADDRESS_PATH) && echo $(BRIDGE_ADDRESS)

bridge-initialize:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		initialize \
		--admin $(ADMIN) \
        --messenger $(MESSENGER_ADDRESS) \
        --gas_oracle $(GAS_ORACLE_ADDRESS) \
        --native_token $(NATIVE_ADDRESS) \

bridge-set-messenger:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		set_messenger \
			--messenger $(MESSENGER_ADDRESS)

bridge-set-gas-usage:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		set_gas_usage \
			--chain_id 16 \
			--gas_usage 150000

define bridge-set-gas-usage-param
	stellar contract invoke \
			--id $(BRIDGE_ADDRESS) \
			--source $(ADMIN_ALIAS) \
			--network $(NETWORK) \
			-- \
			set_gas_usage \
    		--chain_id $1\
            --gas_usage $2
endef

bridge-set-all-gas-usage:
	$(call bridge-set-gas-usage-param,2,250000)
	$(call bridge-set-gas-usage-param,3,150000)
	$(call bridge-set-gas-usage-param,4,3500)
	$(call bridge-set-gas-usage-param,5,250000)
	$(call bridge-set-gas-usage-param,6,250000)
	$(call bridge-set-gas-usage-param,10,250000)
	$(call bridge-set-gas-usage-param,13,7000)
	$(call bridge-set-gas-usage-param,15,7000)
	$(call bridge-set-gas-usage-param,16,150000)

bridge-register-bridge:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		register_bridge \
		--chain_id 16 \
		--bridge_address 02361abd90805a1dfb58fa709d5eff79ce99a47b9a8358cd75c7b29021737b22

define bridge-register-bridge-param
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		register_bridge \
		--chain_id $1 \
		--bridge_address $2
endef

bridge-register-bridge-all:
	$(call bridge-register-bridge-param,16,02361abd90805a1dfb58fa709d5eff79ce99a47b9a8358cd75c7b29021737b22)
	$(call bridge-register-bridge-param,15,0e544b1db5cb742ad501a5f77f166b34c67e3be2d38d7e462af9643eea33f794)
	$(call bridge-register-bridge-param,13,fedfeb84cb16ec880259934a9c802cf775c01f2ecb20d91adf178ecce0112354)
	$(call bridge-register-bridge-param,10,000000000000000000000000760d5d74bead2ccef05ccbfde32a08ebe7e4cfce)
	$(call bridge-register-bridge-param,6,000000000000000000000000835a712cdfb05bb05adf5d44a48f98ee1447f61a)
	$(call bridge-register-bridge-param,5,0000000000000000000000007620b91fC4DaA8f047e624cFB123E75379198498)
	$(call bridge-register-bridge-param,4,270a35d028b2940decaca3c3634f0bf4030c49a7a9a1c70c35bfa5dde5dd6208)
	$(call bridge-register-bridge-param,3,0000000000000000000000000e1de5c7267dc1c1bc498cc9bc3dbcaab305e8da)
	$(call bridge-register-bridge-param,2,000000000000000000000000aa8d065e35929942f10fa8cb58a9af24ee03655d)

bridge-add-bridge-token:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		add_bridge_token \
			--chain_id 16 \
			--token_address 93176772a423589cee546e6121968792fc9d4adf7f04d713075856614192e65a

define bridge-add-bridge-token-param
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		add_bridge_token \
		--chain_id $1 \
		--token_address $2
endef

bridge-add-bridge-token-all:
	$(call bridge-add-bridge-token-param,16,93176772a423589cee546e6121968792fc9d4adf7f04d713075856614192e65a)
	$(call bridge-add-bridge-token-param,16,7d53c865deee2dc29dc38bd87f0296cd28f13a21bc7cc69f7a88a91c4dc6aca1)
	$(call bridge-add-bridge-token-param,15,000000000000000000000000000000000000000000000000000000002c615a73)
	$(call bridge-add-bridge-token-param,15,000000000000000000000000000000000000000000000000000000002c7158e3)
	$(call bridge-add-bridge-token-param,13,b53b0620f575a4ab2bdf1322641ac7392716f8f6d52701c6f91c5283fc9c590a)
	$(call bridge-add-bridge-token-param,13,d4fb9b021179ee578547ddfcbcd24b0725c703428b3fe05467616f87e40b6e18)
	$(call bridge-add-bridge-token-param,10,000000000000000000000000ac7d9d0cc7da68f704a229a7258dc2ba654ffcbc)
	$(call bridge-add-bridge-token-param,10,00000000000000000000000097034742df00c506bd8b9f90e51330bf91ea59b4)
	$(call bridge-add-bridge-token-param,6,00000000000000000000000075faf114eafb1BDbe2F0316DF893fd58CE46AA4d)
	$(call bridge-add-bridge-token-param,5,00000000000000000000000041e94eb019c0762f9bfcf9fb1e58725bfb0e7582)
	$(call bridge-add-bridge-token-param,4,3b442cb3912157f13a933d0134282d032b5ffecd01a2dbf1b7790608df002ea7)
	$(call bridge-add-bridge-token-param,4,dc1f342783eef1ba0c9940714c5b5fe1a76d1f0f2ddab4a4faab53277e07dce3)
	$(call bridge-add-bridge-token-param,4,09c0917b1690e4929808fbc5378d9619a1ff49b3aaff441b2fa4bd58ab035a33)
	$(call bridge-add-bridge-token-param,3,0000000000000000000000003693bdbc20d9d8d0999b1d8effa686e88617e129)
	$(call bridge-add-bridge-token-param,3,0000000000000000000000003224f74a9e32e3f57c1b78a6aee79c257065110b)
	$(call bridge-add-bridge-token-param,2,0000000000000000000000000209da4a278956ca15438af8b108bd85642f096c)
	$(call bridge-add-bridge-token-param,2,00000000000000000000000049be77224dc061bd53699b25431b9aa7029a2cb8)
	$(call bridge-add-bridge-token-param,2,0000000000000000000000001c7D4B196Cb0C7B01d743Fbc6116a902379C7238)

bridge-add-pool:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		add_pool \
		--pool $(POOL_ADDRESS) \
		--token $(TOKEN_ADDRESS)

bridge-set-rebalancer:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		set_rebalancer \
		--rebalancer GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF

bridge-swap-and-bridge:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		swap_and_bridge \
		--sender $(ADMIN) \
        --token $(TOKEN_ADDRESS) \
        --amount 10000000 \
        --recipient 000000000000000000000000be959eed208225aab424505569d41bf3212142c0 \
        --destination_chain_id 2 \
        --receive_token 0000000000000000000000000209da4a278956ca15438af8b108bd85642f096c \
        --nonce 0000000000000000000000000000000000000000000000000000000000000020 \
        --gas_amount 200000000 \
        --fee_token_amount 0

bridge-swap-and-bridge-2:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		swap_and_bridge \
			--sender $(ADMIN) \
			--token $(TOKEN_ADDRESS) \
			--amount 10000000 \
			--recipient 000000000000000000000000be959eed208225aab424505569d41bf3212142c0 \
			--destination_chain_id 2 \
			--receive_token 00000000000000000000000049be77224dc061bd53699b25431b9aa7029a2cb8 \
			--nonce 0000000000000000000000000000000000000000000000000000000000000021 \
			--gas_amount 50000000 \
			--fee_token_amount 0

bridge-swap:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		swap \
			--sender $(ADMIN) \
			--token cb5cd675e2bb2f78d0b923fc555e8875b0b1ec1ecf0a03733133430d7b6b371e \
			--amount 10000000 \
			--recipient $(ADMIN) \
			--receive_token fea8431af9bb6bc27a45309a9db03f9ba478c4675a3d0579d18e303c3aaed561 \
			--receive_amount_min 0

bridge-receive:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		receive_tokens \
			--sender GCDIRA4GRYWVUWB33F6UQUQYAN4L5FXIWJRFPJQMG5YS5EQTWIKAMONK \
			--amount 10048 \
			--recipient GDL27JZFDPBXX7B4DTWPSEWRFHGTAQM6HK365M3J6LVAOBY6VCEUGRCU \
			--source_chain_id 2 \
			--receive_token 1cfbfc7546de559c9ff6ff89169e9acdecebf9ebb6cef3e0683178381816315f \
			--nonce 70705596976575299460036823042420755475664960130040039088966393669628405923302 \
			--receive_amount_min 99009791 \
			--claimable false

bridge-get-transaction-cost:
	$(INVOKE_VIEW) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		get_transaction_cost \
		--chain_id 2

bridge-get-pool-address:
	$(INVOKE_VIEW) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		get_pool_address \
			--token_address 04e57ce1f8ff28bd87daf1875bff9f87c1e8bf9c7f425558d4eb2a0e511b3c3c

bridge-get-config:
	$(INVOKE_VIEW) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		get_config

bridge-get-gas-usage:
	$(INVOKE_VIEW) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		get_gas_usage \
		--chain_id 2

bridge-get-another-bridge:
	$(INVOKE_VIEW) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		get_another_bridge \
		--chain_id 2

bridge-has-received-message:
	$(INVOKE_VIEW) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		has_received_message \
		--message 0107155a5bc1db9cb9d8fc56150518f01011f56ca2e3f0bdeb8dee115344d75b

bridge-install:
	stellar contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(BRIDGE_WASM_PATH)

bridge-update-contract:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		upgrade \
		--new_wasm_hash 4fa4fc1edb540c7c21cd73155838f11be5144e5f2a7060bc89a6b6bee5c24c09

bridge-set-admin:
	$(INVOKE) \
		--id $(BRIDGE_ADDRESS) \
		-- \
		set_admin \
		--new_admin GDQO6XBJ4AFNMFVQMEBCONE36SVZKZ6HUC5ZBU3Z2UYLDEKJVCS6MREW


bridge-restore-contract:
	stellar contract restore \
	--id $(BRIDGE_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679
