.DEFAULT_GOAL := all

all: build-bridge

NATIVE_ADDRESS = CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT
MESSENGER_ADDRESS_PATH = soroban-deploy/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

GAS_ORACLE_ADDRESS_PATH = soroban-deploy/gas_orace
GAS_ORACLE_WASM_PATH = target/wasm32-unknown-unknown/release/gas_oracle.wasm
GAS_ORACLE_ADDRESS = $$(cat $(GAS_ORACLE_ADDRESS_PATH))

POOL_WASM_PATH = target/wasm32-unknown-unknown/release/pool.wasm
POOL_YARO_ADDRESS_PATH = soroban-deploy/pool_yaro
POOL_YARO_ADDRESS = $$(cat $(POOL_YARO_ADDRESS_PATH))

MESSENGER_WASM_PATH = target/wasm32-unknown-unknown/release/messenger.wasm
MESSENGER_ADDRESS_PATH = soroban-deploy/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

BRIDGE_WASM_PATH = target/wasm32-unknown-unknown/release/bridge.wasm
BRIDGE_ADDRESS_PATH = soroban-deploy/bridge
BRIDGE_ADDRESS = $$(cat $(BRIDGE_ADDRESS_PATH))

POOL_ADDRESS_PATH=$(POOL_YARO_ADDRESS_PATH)
POOL_ADDRESS=$(POOL_YARO_ADDRESS)

ALICE = $$(soroban config identity address alice)
ADMIN_ALIAS = alice
ADMIN = $$(soroban config identity address $(ADMIN_ALIAS))

YARO_ADDRESS=CDFVZVTV4K5S66GQXER7YVK6RB23BMPMD3HQUA3TGEZUGDL3NM3R5GDW

TOKEN_ADDRESS=$(YARO_ADDRESS)

NETWORK=futurenet

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



deploy-gas-oracle:
	soroban contract deploy \
      --wasm $(GAS_ORACLE_WASM_PATH) \
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
        --price 1000000000000000000 \
        --gas_price 50

gas-oracle-get-price-data:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_gas_price \
		--chain_id 1

gas-oracle-get-price:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_price \
		--chain_id 1

gas-oracle-get_admin:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_admin

gas-oracle-get-gas-cost-in-native-token:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_gas_cost_in_native_token \
		--other_chain_id 1 \
		--gas_amount 250000

gas-oracle-get-transaction-gas-cost-in-usd:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_transaction_gas_cost_in_usd \
		--other_chain_id 1 \
		--gas_amount 1

gas-oracle-crossrate:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		crossrate \
		--other_chain_id 1

#----------------POOL----------------------------

pool-deploy:
	soroban contract deploy \
          --wasm $(POOL_WASM_PATH) \
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
        --bridge GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF \
        --a 20 \
        --token $(TOKEN_ADDRESS) \
        --fee_share_bp 10 \
        --balance_ratio_min_bp 0 \
        --admin_fee_share_bp 10

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
		--amount 10000000000

pool-get-pool-info:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_pool

pool-get-admin:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_admin

#---------------MESSENGER---------------------------
messenger-deploy:
	soroban contract deploy \
		--wasm $(MESSENGER_WASM_PATH) \
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
        --other_chain_ids 0001010101010100000000000000000000000000000000000000000000000000 \
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
		--chain_id 1\
        --gas_usage 100000


send-message:
	soroban contract invoke \
		  --id $(MESSENGER_ADDRESS) \
		  --source alice \
		  --network futurenet \
		  -- \
		  send_message \
		  --message 0701efefefefefefefefefefefefefefefefefefefefefefefefefefefefefef \
		  --sender $(ALICE)


#---------------BRIDGE---------------------------
bridge-deploy:
	soroban contract deploy \
		--wasm $(BRIDGE_WASM_PATH) \
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
		--chain_id 1 \
		--gas_usage 250000

bridge-register-bridge:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		register_bridge \
		--chain_id 1 \
		--bridge_address 000000000000000000000000a32196e86caa4e5d8bb44a7e7f162804421e38b7

bridge-add-bridge-token:
	soroban contract invoke \
    		--id $(BRIDGE_ADDRESS) \
    		--source $(ADMIN_ALIAS) \
    		--network $(NETWORK) 	\
    		-- \
    		add_bridge_token \
    		--chain_id 1 \
    		--token_address 000000000000000000000000ddac3cb57dea3fbeff4997d78215535eb5787117

bridge-add-pool:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		add_pool \
		--pool $(POOL_ADDRESS) \
		--token cb5cd675e2bb2f78d0b923fc555e8875b0b1ec1ecf0a03733133430d7b6b371e

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
        --token cb5cd675e2bb2f78d0b923fc555e8875b0b1ec1ecf0a03733133430d7b6b371e \
        --amount 10000000 \
        --recipient 000000000000000000000000be959eed208225aab424505569d41bf3212142c0 \
        --destination_chain_id 1 \
        --receive_token 000000000000000000000000ddac3cb57dea3fbeff4997d78215535eb5787117 \
        --nonce 0000000000000000000000000000000000000000000000000000000000000002 \
        --gas_amount 300000 \
        --fee_token_amount 0

bridge-get-config:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_config

bridge-get-gas-usage:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_gas_usage \
		--chain_id 1


bridge-has-received-message:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		has_received_message \
		--message 0107007b940a7518925c1ec6a1fc95d2b0516c2a0af0dd4baf3635f86aca3d22



#----------TOKEN--------------------------
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
		--amount 1000000000

token-get-balance:
	soroban contract invoke \
		--id $(TOKEN_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		balance \
		--id $(POOL_ADDRESS)

