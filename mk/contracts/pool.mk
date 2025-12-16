pool-deploy:
	$(DEPLOY) $(POOL_WASM_PATH) > $(POOL_ADDRESS_PATH) && echo $(POOL_ADDRESS)

pool-deploy-by-hash:
	stellar contract deploy \
		--wasm-hash <hash> \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		> $(POOL_ADDRESS_PATH) && echo $(POOL_ADDRESS)

pool-initialize:
	$(INVOKE) \
		--id $(POOL_ADDRESS) \
		-- \
		initialize \
		--admin $(ADMIN) \
			--bridge $(BRIDGE_ADDRESS) \
			--a 20 \
			--token $(TOKEN_ADDRESS) \
			--fee_share_bp 15 \
			--balance_ratio_min_bp 1000 \
			--admin_fee_share_bp 2000

pool-set-bridge:
	$(INVOKE) \ 
		--id $(POOL_ADDRESS) \
		-- \
		set_bridge \
		--bridge $(BRIDGE_ADDRESS)

pool-deposit:
	$(INVOKE) \
		--id $(POOL_ADDRESS) \
		-- \
		deposit \
		--sender $(ADMIN) \
		--amount 1000000000000

pool-get-pool-info:
	$(INVOKE_VIEW) \
		--id $(POOL_ADDRESS) \
		-- \
		get_pool

pool-get-admin:
	$(INVOKE) \
		--id $(POOL_ADDRESS) \
		-- \
		get_admin

pool-get-pending-reward:
	$(INVOKE_VIEW) \
		--id $(POOL_ADDRESS) \
		-- \
		pending_reward \
		--user $(ADMIN)

pool-get-user-deposit:
	$(INVOKE_VIEW) \
		--id $(POOL_ADDRESS) \
		-- \
		get_user_deposit \
		--user $(ADMIN)

pool-get-claimable-balance:
	$(INVOKE_VIEW) \
		--id $(POOL_ADDRESS) \
		-- \
		get_claimable_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

pool-claim-balance:
	$(INVOKE) \
		--id $(POOL_ADDRESS) \
		-- \
		claim_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

pool-install:
	stellar contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(POOL_WASM_PATH)

pool-update-contract:
	$(INVOKE) \
		--id $(POOL_ADDRESS) \
			-- \
			upgrade \
			--new_wasm_hash 8f1bf6f8b9e82b29415e93202525058012898769cbcd3b5c81affdfb0bf645f4

pool-restore-contract:
	stellar contract restore \
	--id $(POOL_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679

pool-set-admin:
	$(INVOKE) \
		--id $(POOL_ADDRESS) \
		-- \
		set_admin \
		--new_admin GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF

