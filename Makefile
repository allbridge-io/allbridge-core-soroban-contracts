.DEFAULT_GOAL := all

all: build-bridge

optimize-all: optimize-gas-oracle optimize-messenger optimize-pool optimize-bridge

ADDRESS_PATH = soroban-deploy

#NATIVE_ADDRESS = CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT #Futurenet
NATIVE_ADDRESS = CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC #Testnet
#NATIVE_ADDRESS = CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA #Mainnet
MESSENGER_ADDRESS_PATH = $(ADDRESS_PATH)/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

GAS_ORACLE_ADDRESS_PATH = $(ADDRESS_PATH)/gas_orace
GAS_ORACLE_WASM_PATH = target/wasm32-unknown-unknown/release/gas_oracle.wasm
GAS_ORACLE_WASM_PATH_OP = target/wasm32-unknown-unknown/release/gas_oracle.optimized.wasm
GAS_ORACLE_ADDRESS = $$(cat $(GAS_ORACLE_ADDRESS_PATH))

POOL_WASM_PATH = target/wasm32-unknown-unknown/release/pool.wasm
POOL_WASM_PATH_OP = target/wasm32-unknown-unknown/release/pool.optimized.wasm
POOL_YARO_ADDRESS_PATH = $(ADDRESS_PATH)/pool_yaro
POOL_YARO_ADDRESS = $$(cat $(POOL_YARO_ADDRESS_PATH))

POOL_USDY_ADDRESS_PATH = $(ADDRESS_PATH)/pool_usdy
POOL_USDY_ADDRESS = $$(cat $(POOL_USDY_ADDRESS_PATH))

POOL_USDC_ADDRESS_PATH = $(ADDRESS_PATH)/pool
POOL_USDC_ADDRESS = $$(cat $(POOL_USDC_ADDRESS_PATH))

MESSENGER_WASM_PATH = target/wasm32-unknown-unknown/release/messenger.wasm
MESSENGER_WASM_PATH_OP = target/wasm32-unknown-unknown/release/messenger.optimized.wasm
MESSENGER_ADDRESS_PATH = $(ADDRESS_PATH)/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

BRIDGE_WASM_PATH = target/wasm32-unknown-unknown/release/bridge.wasm
BRIDGE_WASM_PATH_OP = target/wasm32-unknown-unknown/release/bridge.optimized.wasm
BRIDGE_ADDRESS_PATH = $(ADDRESS_PATH)/bridge
BRIDGE_ADDRESS = $$(cat $(BRIDGE_ADDRESS_PATH))

POOL_ADDRESS_PATH=$(POOL_YARO_ADDRESS_PATH)
POOL_ADDRESS=$(POOL_YARO_ADDRESS)

ALICE = $$(soroban config identity address alice)
ADMIN_ALIAS = alice
ADMIN = $$(soroban config identity address $(ADMIN_ALIAS))

#YARO_ADDRESS=CDFVZVTV4K5S66GQXER7YVK6RB23BMPMD3HQUA3TGEZUGDL3NM3R5GDW #Futurenet
#USDY_ADDRESS=CD7KQQY27G5WXQT2IUYJVHNQH6N2I6GEM5ND2BLZ2GHDAPB2V3KWCW7M #Futurenet

YARO_ADDRESS=CACOK7HB7D7SRPMH3LYYOW77T6D4D2F7TR7UEVKY2TVSUDSRDM6DZVLK #Testnet
USDY_ADDRESS=CAOPX7DVI3PFLHE7637YSFU6TLG6Z27Z5O3M547ANAYXQOAYCYYV6NO6 #Testnet

USDC_ADDRESS=CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75

#TOKEN_ADDRESS=$(YARO_ADDRESS)
#POOL_ADDRESS_PATH=$(POOL_YARO_ADDRESS_PATH)
#POOL_ADDRESS=$(POOL_YARO_ADDRESS)

TOKEN_ADDRESS=$(USDY_ADDRESS)
POOL_ADDRESS_PATH=$(POOL_USDY_ADDRESS_PATH)
POOL_ADDRESS=$(POOL_USDY_ADDRESS)

#TOKEN_ADDRESS=$(USDC_ADDRESS)
#POOL_ADDRESS_PATH=$(POOL_USDC_ADDRESS_PATH)
#POOL_ADDRESS=$(POOL_USDC_ADDRESS)

NETWORK=testnet

test: all
	cargo test

build-gas-oracle:
	 cargo build --target wasm32-unknown-unknown --release --package gas-oracle

build-messenger: build-gas-oracle
	 cargo build --target wasm32-unknown-unknown --release --package messenger

build-pool: 
	cargo build --target wasm32-unknown-unknown --release --package pool

build-bridge: build-messenger build-pool
	cargo build --target wasm32-unknown-unknown --release --package bridge

optimize-gas-oracle:
	soroban contract optimize --wasm $(GAS_ORACLE_WASM_PATH)

optimize-messenger:
	soroban contract optimize --wasm $(MESSENGER_WASM_PATH)

optimize-pool:
	soroban contract optimize --wasm $(POOL_WASM_PATH)

optimize-bridge:
	soroban contract optimize --wasm $(BRIDGE_WASM_PATH)

deploy-gas-oracle:
	soroban contract deploy \
      --wasm $(GAS_ORACLE_WASM_PATH_OP) \
      --source $(ADMIN_ALIAS) \
      --network $(NETWORK) 	\
      > $(GAS_ORACLE_ADDRESS_PATH) && echo $(GAS_ORACLE_ADDRESS)

gas-oracle-init:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		initialize \
		--admin $(ADMIN)

gas-oracle-set-price:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_price \
		--chain_id 7 \
        --price 136000000000000000 \
        --gas_price 50

gas-oracle-set-price-1:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_price \
		--chain_id 1 \
        --price 0 \
        --gas_price 0

gas-oracle-get-price-data:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--is-view \
		-- \
		get_gas_price \
		--chain_id 7

gas-oracle-get-price:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--is-view \
		-- \
		get_price \
		--chain_id 2

gas-oracle-get-admin:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_admin

gas-oracle-get-gas-cost-in-native-token:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_gas_cost_in_native_token \
		--other_chain_id 2 \
		--gas_amount 250000

gas-oracle-get-transaction-gas-cost-in-usd:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_transaction_gas_cost_in_usd \
		--other_chain_id 1 \
		--gas_amount 1

gas-oracle-crossrate:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		crossrate \
		--other_chain_id 1

gas-oracle-install:
	soroban contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(GAS_ORACLE_WASM_PATH_OP)

gas-oracle-update-contract:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
        -- \
        upgrade \
        --new_wasm_hash a16f4b45aff547518e6c1bb2ffe2e26b232562edb2405fd9be413e31d495098d


gas-oracle-restore-contract:
	soroban contract restore \
	--id $(GAS_ORACLE_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679

#----------------POOL----------------------------

pool-deploy:
	soroban contract deploy \
          --wasm $(POOL_WASM_PATH_OP) \
          --source $(ADMIN_ALIAS) \
          --network $(NETWORK) 	\
          > $(POOL_ADDRESS_PATH) && echo $(POOL_ADDRESS)

pool-deploy-by-hash:
	soroban contract deploy \
          --wasm-hash <hash> \
          --source $(ADMIN_ALIAS) \
          --network $(NETWORK) 	\
          > $(POOL_ADDRESS_PATH) && echo $(POOL_ADDRESS)

pool-initialize:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
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
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_bridge \
		--bridge $(BRIDGE_ADDRESS)

pool-deposit:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		deposit \
		--sender $(ADMIN) \
		--amount 100000000

pool-get-pool-info:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_pool

pool-get-admin:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--is-view \
		-- \
		get_admin

pool-get-pending-reward:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		pending_reward \
		--user $(ADMIN)

pool-get-user-deposit:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_user_deposit \
		--user $(ADMIN)

pool-get-claimable-balance:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_claimable_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

pool-claim-balance:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		claim_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

pool-install:
	soroban contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(POOL_WASM_PATH_OP)

pool-update-contract:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
        -- \
        upgrade \
        --new_wasm_hash 8f1bf6f8b9e82b29415e93202525058012898769cbcd3b5c81affdfb0bf645f4

pool-restore-contract:
	soroban contract restore \
	--id $(POOL_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679

#---------------MESSENGER---------------------------
messenger-deploy:
	soroban contract deploy \
		--wasm $(MESSENGER_WASM_PATH_OP) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		> $(MESSENGER_ADDRESS_PATH) && echo $(MESSENGER_ADDRESS)

messenger-initialize:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
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
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_gas_usage \
		--chain_id 4\
        --gas_usage 1300


messenger-send-message:
	soroban contract invoke \
		  --id $(MESSENGER_ADDRESS) \
		  --source alice \
		  --network $(NETWORK) \
		  -- \
		  send_message \
		  --message 0701efefefefefefefefefefefefefefefefefefefefefefefefefefefefefef \
		  --sender $(ALICE)

messenger-receive_message:
	soroban contract invoke \
		  --id $(MESSENGER_ADDRESS) \
		  --source alice \
		  --network $(NETWORK) \
		  --cost \
		  -- \
		  receive_message \
		  --message 02071be378bf6338f627121187e82e54fcf64577ff705e2ee3fc12a930a43bd3 \
          --primary_signature 807131e734509d76ad48da6a075a93257897d51a49e2b6dc53e945a313dbb1f10e1fbb57050a156a29976be1a3dd3f87d6cca948eb4bd1d34328e3c58a7d0032 \
          --primary_recovery_id 0 \
          --secondary_signature 807131e734509d76ad48da6a075a93257897d51a49e2b6dc53e945a313dbb1f10e1fbb57050a156a29976be1a3dd3f87d6cca948eb4bd1d34328e3c58a7d0032 \
          --secondary_recovery_id 0

messenger-get-gas-usage:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_gas_usage \
		--chain_id 1

messenger-get-transaction-cost:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_transaction_cost \
		--chain_id 1

messenger-has-received-message:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		has_received_message \
		--message 020777b64e53254cc42d1d695036cf5f438312735b915adec350b68ff713c997

messenger-has-sent-message:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		has_sent_message \
		--message 020777b64e53254cc42d1d695036cf5f438312735b915adec350b68ff713c997

messenger-install:
	soroban contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(MESSENGER_WASM_PATH_OP)

messenger-update-contract:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
        -- \
        upgrade \
        --new_wasm_hash 5accf5d0f95f58fa341bc6ac968908bd2ebc864cb9bf6eeda6c799022cde1d45

messenger-restore-contract:
	soroban contract restore \
	--id $(MESSENGER_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679

#---------------BRIDGE---------------------------
bridge-deploy:
	soroban contract deploy \
		--wasm $(BRIDGE_WASM_PATH_OP) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		> $(BRIDGE_ADDRESS_PATH) && echo $(BRIDGE_ADDRESS)

bridge-initialize:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		initialize \
		--admin $(ADMIN) \
        --messenger $(MESSENGER_ADDRESS) \
        --gas_oracle $(GAS_ORACLE_ADDRESS) \
        --native_token $(NATIVE_ADDRESS) \

bridge-set-messenger:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_messenger \
		--messenger $(MESSENGER_ADDRESS)

bridge-set-gas-usage:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_gas_usage \
		--chain_id 25 \
		--gas_usage 25

bridge-register-bridge:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		register_bridge \
#		--chain_id 10 \
#		--bridge_address 000000000000000000000000760d5d74bead2ccef05ccbfde32a08ebe7e4cfce
#		--chain_id 6 \
#		--bridge_address 000000000000000000000000c63c0261c2f1d21b3efe7828032e646c797ee21e
#		--chain_id 5 \
#		--bridge_address 000000000000000000000000763e75ca7bc589396f0e5c1b8049ac5ed7c8387f
#		--chain_id 4 \
#		--bridge_address 270a35d028b2940decaca3c3634f0bf4030c49a7a9a1c70c35bfa5dde5dd6208
#		--chain_id 3 \
#		--bridge_address 0000000000000000000000000e1de5c7267dc1c1bc498cc9bc3dbcaab305e8da
#		--chain_id 2 \
#		--bridge_address 000000000000000000000000aa8d065e35929942f10fa8cb58a9af24ee03655d
#		--chain_id 1 \
#		--bridge_address 000000000000000000000000a32196e86caa4e5d8bb44a7e7f162804421e38b7

bridge-add-bridge-token:
	soroban contract invoke \
    		--id $(BRIDGE_ADDRESS) \
    		--source $(ADMIN_ALIAS) \
    		--network $(NETWORK) 	\
    		-- \
    		add_bridge_token \
#    		--chain_id 10 \
#    		--token_address 000000000000000000000000ac7d9d0cc7da68f704a229a7258dc2ba654ffcbc
#    		--chain_id 10 \
#    		--token_address 00000000000000000000000097034742df00c506bd8b9f90e51330bf91ea59b4
#    		--chain_id 6 \
#    		--token_address 000000000000000000000000fd064A18f3BF249cf1f87FC203E90D8f650f2d63
#    		--chain_id 5 \
#    		--token_address 000000000000000000000000d18967827f4cc29193a7dbe2aa5ad440f6b27597
#    		--chain_id 5 \
#    		--token_address 0000000000000000000000003dbe838b635c54502c318f752187a8d8e7c73743
#    		--chain_id 4 \
#    		--token_address 3b442cb3912157f13a933d0134282d032b5ffecd01a2dbf1b7790608df002ea7
#    		--chain_id 4 \
#    		--token_address dc1f342783eef1ba0c9940714c5b5fe1a76d1f0f2ddab4a4faab53277e07dce3
#    		--chain_id 4 \
#    		--token_address 09c0917b1690e4929808fbc5378d9619a1ff49b3aaff441b2fa4bd58ab035a33
#    		--chain_id 3 \
#    		--token_address 0000000000000000000000003693bdbc20d9d8d0999b1d8effa686e88617e129
#    		--chain_id 3 \
#    		--token_address 0000000000000000000000003224f74a9e32e3f57c1b78a6aee79c257065110b
#    		--chain_id 2 \
#    		--token_address 0000000000000000000000000209da4a278956ca15438af8b108bd85642f096c
#    		--chain_id 2 \
#    		--token_address 00000000000000000000000049be77224dc061bd53699b25431b9aa7029a2cb8
#    		--chain_id 1 \
#    		--token_address 00000000000000000000000007865c6e87b9f70255377e024ace6630c1eaa37f
#    		--chain_id 1 \
#    		--token_address 000000000000000000000000c7dbc4a896b34b7a10dda2ef72052145a9122f43
#    		--chain_id 1 \
#    		--token_address 000000000000000000000000ddac3cb57dea3fbeff4997d78215535eb5787117

bridge-add-pool:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		add_pool \
		--pool $(POOL_ADDRESS) \
		--token $(TOKEN_ADDRESS)

bridge-set-rebalancer:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_rebalancer \
		--rebalancer GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF

bridge-swap-and-bridge:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		swap_and_bridge \
		--sender $(ADMIN) \
        --token $(TOKEN_ADDRESS) \
        --amount 10000000 \
        --recipient 000000000000000000000000be959eed208225aab424505569d41bf3212142c0 \
        --destination_chain_id 30 \
        --receive_token 0000000000000000000000003c499c542cef5e3811e1192ce70d8cc03d5c3359 \
        --nonce 0000000000000000000000000000000000000000000000000000000000000019 \
        --gas_amount 20000000 \
        --fee_token_amount 0

bridge-swap-and-bridge-2:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		swap_and_bridge \
		--sender $(ADMIN) \
        --token $(TOKEN_ADDRESS) \
        --amount 10000000 \
        --recipient 000000000000000000000000be959eed208225aab424505569d41bf3212142c0 \
        --destination_chain_id 2 \
        --receive_token 0000000000000000000000000209da4a278956ca15438af8b108bd85642f096c \
        --nonce 0000000000000000000000000000000000000000000000000000000000000019 \
        --gas_amount 50000000 \
        --fee_token_amount 0

bridge-swap:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		swap \
		--sender $(ADMIN) \
        --token cb5cd675e2bb2f78d0b923fc555e8875b0b1ec1ecf0a03733133430d7b6b371e \
        --amount 10000000 \
        --recipient $(ADMIN) \
        --receive_token fea8431af9bb6bc27a45309a9db03f9ba478c4675a3d0579d18e303c3aaed561 \
        --receive_amount_min 0 \
        --claimable false


bridge-receive:
	soroban contract invoke \
    		--id $(BRIDGE_ADDRESS) \
    		--source $(ADMIN_ALIAS) \
    		--network $(NETWORK) 	\
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
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_transaction_cost \
		--chain_id 1

bridge-get-pool-address:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_pool_address \
		--token_address 04e57ce1f8ff28bd87daf1875bff9f87c1e8bf9c7f425558d4eb2a0e511b3c3c

bridge-get-config:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--is-view \
		-- \
		get_config

bridge-get-gas-usage:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
        --is-view \
		-- \
		get_gas_usage \
		--chain_id 7

bridge-get-another-bridge:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
        --is-view \
		-- \
		get_another_bridge \
		--chain_id 6

bridge-has-received-message:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
        --is-view \
		-- \
		has_received_message \
		--message 0107155a5bc1db9cb9d8fc56150518f01011f56ca2e3f0bdeb8dee115344d75b

bridge-install:
	soroban contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(BRIDGE_WASM_PATH_OP)

bridge-update-contract:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
        -- \
        upgrade \
        --new_wasm_hash 349aaa0f85aaf2ab74bb251dfeb4b150939f0dbaa9f82bf922648604d21ec6fa


bridge-restore-contract:
	soroban contract restore \
	--id $(BRIDGE_ADDRESS) \
	--source $(ADMIN_ALIAS) \
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679

#----------UTILS--------------------------
token-transfer:
	soroban contract invoke \
		--id $(TOKEN_ADDRESS) \
		--source SBTECKZAIBLA6ZGPCG5IKON2IG4SJ37AVZEIY5OHCCKJ7KYCAJQKF5EB \
		--network $(NETWORK) 	\
		-- \
		transfer \
		--from GA2LLFIX5V3JT6IW67HH2JESPYALDGCV2AGCSEQOOEMKMF5K3WL2K7OS \
		--to $(ADMIN) \
		--amount 10000000000000

token-native-transfer:
	soroban contract invoke \
		--id $(NATIVE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		transfer \
		--from $(ADMIN) \
		--to $(BRIDGE_ADDRESS) \
		--amount 100000000

token-get-balance:
	soroban contract invoke \
		--id $(TOKEN_ADDRESS) \
		--network $(NETWORK) 	\
		--is-view \
		-- \
		balance \
		--id $(POOL_ADDRESS)


token-get-name:
	soroban contract invoke \
		--id CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75 \
		--source $(ADMIN_ALIAS) \
		--is-view \
		--network $(NETWORK) 	\
		-- \
		name

wrap-token:
	soroban lab token wrap \
		--network testnet 	\
		--source alice \
		--asset BOGD:GAYODJWF27E5OQO2C6LA6Z6QXQ2EYUONMXFNL2MNMGRJP6RED2CPQKTW

native-token-address:
	soroban lab token id \
 		--network $(NETWORK) \
 		--source $(ADMIN_ALIAS) \
 		--asset native

generate-types:
	soroban contract bindings typescript \
	--network $(NETWORK) \
	--output-dir ./types/gas-oracle \
	--wasm $(GAS_ORACLE_WASM_PATH_OP) \
	--contract-id $(GAS_ORACLE_ADDRESS)

install-cli:
	cargo install --locked --version 20.3.1 soroban-cli