.DEFAULT_GOAL := all

all: build-auto-deposit

ADDRESS_PATH = stellar-deploy-testnet

#NATIVE_ADDRESS = CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT #Futurenet
NATIVE_ADDRESS = CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC #Testnet
#NATIVE_ADDRESS = CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA #Mainnet
MESSENGER_ADDRESS_PATH = $(ADDRESS_PATH)/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

GAS_ORACLE_ADDRESS_PATH = $(ADDRESS_PATH)/gas_orace
GAS_ORACLE_WASM_PATH = target/wasm32v1-none/release/gas_oracle.wasm
GAS_ORACLE_WASM_PATH_OP = target/wasm32v1-none/release/gas_oracle.wasm
GAS_ORACLE_ADDRESS = $$(cat $(GAS_ORACLE_ADDRESS_PATH))

POOL_WASM_PATH = target/wasm32v1-none/release/pool.wasm
POOL_WASM_PATH_OP = target/wasm32v1-none/release/pool.wasm
POOL_YARO_ADDRESS_PATH = $(ADDRESS_PATH)/pool_yaro
POOL_YARO_ADDRESS = $$(cat $(POOL_YARO_ADDRESS_PATH))

POOL_USDY_ADDRESS_PATH = $(ADDRESS_PATH)/pool_usdy
POOL_USDY_ADDRESS = $$(cat $(POOL_USDY_ADDRESS_PATH))

POOL_USDC_ADDRESS_PATH = $(ADDRESS_PATH)/pool
POOL_USDC_ADDRESS = $$(cat $(POOL_USDC_ADDRESS_PATH))

MESSENGER_WASM_PATH = target/wasm32v1-none/release/messenger.wasm
MESSENGER_WASM_PATH_OP = target/wasm32v1-none/release/messenger.wasm
MESSENGER_ADDRESS_PATH = $(ADDRESS_PATH)/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

BRIDGE_WASM_PATH = target/wasm32v1-none/release/bridge.wasm
BRIDGE_WASM_PATH_OP = target/wasm32v1-none/release/bridge.wasm
BRIDGE_ADDRESS_PATH = $(ADDRESS_PATH)/bridge
BRIDGE_ADDRESS = $$(cat $(BRIDGE_ADDRESS_PATH))

AUTO_DEPOSIT_WALLET_WASM_PATH = target/wasm32v1-none/release/auto_deposit_wallet.wasm
AUTO_DEPOSIT_WALLET_WASM_PATH_OP = target/wasm32v1-none/release/auto_deposit_wallet.wasm

AUTO_DEPOSIT_WALLET_WASM_HASH=eddf45c0751a66e124b76604d96a757adeada9978f461eb662b36994439af1aa

AUTO_DEPOSIT_FACTORY_ADDRESS_PATH = $(ADDRESS_PATH)/auto_deposit_factory
AUTO_DEPOSIT_FACTORY_ADDRESS = $$(cat $(AUTO_DEPOSIT_FACTORY_ADDRESS_PATH))
AUTO_DEPOSIT_FACTORY_WASM_PATH = target/wasm32v1-none/release/auto_deposit_factory.wasm
AUTO_DEPOSIT_FACTORY_WASM_PATH_OP = target/wasm32v1-none/release/auto_deposit_factory.wasm

ALICE = $$(stellar keys address alice)
ADMIN_ALIAS = alice
ADMIN = $$(stellar keys address $(ADMIN_ALIAS))

#YARO_ADDRESS=CDFVZVTV4K5S66GQXER7YVK6RB23BMPMD3HQUA3TGEZUGDL3NM3R5GDW #Futurenet
#USDY_ADDRESS=CD7KQQY27G5WXQT2IUYJVHNQH6N2I6GEM5ND2BLZ2GHDAPB2V3KWCW7M #Futurenet

YARO_ADDRESS=CACOK7HB7D7SRPMH3LYYOW77T6D4D2F7TR7UEVKY2TVSUDSRDM6DZVLK #Testnet
USDY_ADDRESS=CAOPX7DVI3PFLHE7637YSFU6TLG6Z27Z5O3M547ANAYXQOAYCYYV6NO6 #Testnet

USDC_ADDRESS=CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA

#TOKEN_ADDRESS=$(YARO_ADDRESS)
#POOL_ADDRESS_PATH=$(POOL_YARO_ADDRESS_PATH)
#POOL_ADDRESS=$(POOL_YARO_ADDRESS)

#TOKEN_ADDRESS=$(USDY_ADDRESS)
#POOL_ADDRESS_PATH=$(POOL_USDY_ADDRESS_PATH)
#POOL_ADDRESS=$(POOL_USDY_ADDRESS)

TOKEN_ADDRESS=$(USDC_ADDRESS)
POOL_ADDRESS_PATH=$(POOL_USDC_ADDRESS_PATH)
POOL_ADDRESS=$(POOL_USDC_ADDRESS)

NETWORK=testnet

test: all
	CARGO_INCREMENTAL=0 cargo test

build-auto-deposit-factory: build-bridge build-auto-deposit-wallet
	 stellar contract build --optimize --profile release --package auto-deposit-factory

build-auto-deposit-wallet:
	 stellar contract build --optimize --profile release --package auto-deposit-wallet

build-auto-deposit: build-auto-deposit-factory

build-gas-oracle:
	stellar contract build --optimize --profile release --package gas-oracle

build-messenger: build-gas-oracle
	stellar contract build --optimize --profile release --package messenger

build-pool:
	stellar contract build --optimize --profile release --package pool

build-bridge: build-messenger build-pool
	stellar contract build --optimize --profile release --package bridge

deploy-gas-oracle:
	stellar contract deploy \
      --wasm $(GAS_ORACLE_WASM_PATH_OP) \
      --source $(ADMIN_ALIAS) \
      --network $(NETWORK) 	\
      > $(GAS_ORACLE_ADDRESS_PATH) && echo $(GAS_ORACLE_ADDRESS)

gas-oracle-init:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		initialize \
		--admin $(ADMIN)

gas-oracle-set-price:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_price \
		--chain_id 7 \
        --price 136000000000000000 \
        --gas_price 50

gas-oracle-set-price-1:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_price \
		--chain_id 1 \
        --price 0 \
        --gas_price 0

gas-oracle-get-price-data:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--send=no \
		-- \
		get_gas_price \
		--chain_id 2

gas-oracle-get-price:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--send=no \
		-- \
		get_price \
		--chain_id 7

gas-oracle-get-admin:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_admin

gas-oracle-get-gas-cost-in-native-token:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_gas_cost_in_native_token \
		--other_chain_id 2 \
		--gas_amount 250000

gas-oracle-get-transaction-gas-cost-in-usd:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_transaction_gas_cost_in_usd \
		--other_chain_id 1 \
		--gas_amount 1

gas-oracle-crossrate:
	stellar contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		crossrate \
		--other_chain_id 1

gas-oracle-install:
	stellar contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(GAS_ORACLE_WASM_PATH_OP)

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
	--network $(NETWORK) 	\
	--durability persistent \
	--ledgers-to-extend 535679

#----------------POOL----------------------------

pool-deploy:
	stellar contract deploy \
          --wasm $(POOL_WASM_PATH_OP) \
          --source $(ADMIN_ALIAS) \
          --network $(NETWORK) 	\
          > $(POOL_ADDRESS_PATH) && echo $(POOL_ADDRESS)

pool-deploy-by-hash:
	stellar contract deploy \
          --wasm-hash <hash> \
          --source $(ADMIN_ALIAS) \
          --network $(NETWORK) 	\
          > $(POOL_ADDRESS_PATH) && echo $(POOL_ADDRESS)

pool-initialize:
	stellar contract invoke \
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
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_bridge \
		--bridge $(BRIDGE_ADDRESS)

pool-deposit:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		deposit \
		--sender $(ADMIN) \
		--amount 1000000000000

pool-get-pool-info:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_pool

pool-get-admin:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--send=no \
		-- \
		get_admin

pool-get-pending-reward:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		pending_reward \
		--user $(ADMIN)

pool-get-user-deposit:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_user_deposit \
		--user $(ADMIN)

pool-get-claimable-balance:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_claimable_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

pool-claim-balance:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		claim_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

pool-install:
	stellar contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(POOL_WASM_PATH_OP)

pool-update-contract:
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
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
	stellar contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_admin \
		--new_admin GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF

#---------------MESSENGER---------------------------
messenger-deploy:
	stellar contract deploy \
		--wasm $(MESSENGER_WASM_PATH_OP) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		> $(MESSENGER_ADDRESS_PATH) && echo $(MESSENGER_ADDRESS)

messenger-initialize:
	stellar contract invoke \
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
	stellar contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
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
	stellar contract invoke \
		  --id $(MESSENGER_ADDRESS) \
		  --source alice \
		  --network $(NETWORK) \
		  -- \
		  send_message \
		  --message 0701efefefefefefefefefefefefefefefefefefefefefefefefefefefefefef \
		  --sender $(ALICE)

messenger-receive_message:
	stellar contract invoke \
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
	stellar contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_gas_usage \
		--chain_id 2

messenger-get-transaction-cost:
	stellar contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_transaction_cost \
		--chain_id 2

messenger-has-received-message:
	stellar contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		has_received_message \
		--message 0207a3508a81ab1b0043a51568079044f4e34648226124dccd21f5d89c51f3fb

messenger-has-sent-message:
	stellar contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		has_sent_message \
		--message 020777b64e53254cc42d1d695036cf5f438312735b915adec350b68ff713c997

messenger-install:
	stellar contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(MESSENGER_WASM_PATH_OP)

messenger-update-contract:
	stellar contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
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

#---------------BRIDGE---------------------------
bridge-deploy:
	stellar contract deploy \
		--wasm $(BRIDGE_WASM_PATH_OP) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		> $(BRIDGE_ADDRESS_PATH) && echo $(BRIDGE_ADDRESS)

bridge-initialize:
	stellar contract invoke \
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
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_messenger \
		--messenger $(MESSENGER_ADDRESS)


bridge-set-gas-usage:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_gas_usage \
		--chain_id 16 \
		--gas_usage 150000

define bridge-set-gas-usage-param
	stellar contract invoke \
			--id $(BRIDGE_ADDRESS) \
			--source $(ADMIN_ALIAS) \
			--network $(NETWORK) 	\
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
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
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
	stellar contract invoke \
    		--id $(BRIDGE_ADDRESS) \
    		--source $(ADMIN_ALIAS) \
    		--network $(NETWORK) 	\
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
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		add_pool \
		--pool $(POOL_ADDRESS) \
		--token $(TOKEN_ADDRESS)

bridge-set-rebalancer:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_rebalancer \
		--rebalancer GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF

bridge-swap-and-bridge:
	stellar contract invoke \
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
        --nonce 0000000000000000000000000000000000000000000000000000000000000020 \
        --gas_amount 200000000 \
        --fee_token_amount 0

bridge-swap-and-bridge-2:
	stellar contract invoke \
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
        --receive_token 00000000000000000000000049be77224dc061bd53699b25431b9aa7029a2cb8 \
        --nonce 0000000000000000000000000000000000000000000000000000000000000021 \
        --gas_amount 50000000 \
        --fee_token_amount 0

bridge-swap:
	stellar contract invoke \
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
        --receive_amount_min 0


bridge-receive:
	stellar contract invoke \
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
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_transaction_cost \
		--chain_id 2

bridge-get-pool-address:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_pool_address \
		--token_address 04e57ce1f8ff28bd87daf1875bff9f87c1e8bf9c7f425558d4eb2a0e511b3c3c

bridge-get-config:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
		--send=no \
		-- \
		get_config

bridge-get-gas-usage:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
        --send=no \
		-- \
		get_gas_usage \
		--chain_id 2

bridge-get-another-bridge:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
        --send=no \
		-- \
		get_another_bridge \
		--chain_id 2

bridge-has-received-message:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		--source $(ADMIN_ALIAS) \
        --send=no \
		-- \
		has_received_message \
		--message 0107155a5bc1db9cb9d8fc56150518f01011f56ca2e3f0bdeb8dee115344d75b

bridge-install:
	stellar contract install \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--wasm $(BRIDGE_WASM_PATH_OP)

bridge-update-contract:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
        -- \
        upgrade \
        --new_wasm_hash 4fa4fc1edb540c7c21cd73155838f11be5144e5f2a7060bc89a6b6bee5c24c09

bridge-set-admin:
	stellar contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
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

#----------UTILS--------------------------
token-transfer:
	stellar contract invoke \
		--id $(TOKEN_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		transfer \
		--from GDL27JZFDPBXX7B4DTWPSEWRFHGTAQM6HK365M3J6LVAOBY6VCEUGRCU \
		--to GCDIRA4GRYWVUWB33F6UQUQYAN4L5FXIWJRFPJQMG5YS5EQTWIKAMONK \
		--amount 1000

token-native-transfer:
	stellar contract invoke \
		--id $(NATIVE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		transfer \
		--from $(ADMIN) \
		--to $(BRIDGE_ADDRESS) \
		--amount 10000000000

token-get-balance:
	stellar contract invoke \
		--id $(TOKEN_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		--send=no \
		-- \
		balance \
		--id GDL27JZFDPBXX7B4DTWPSEWRFHGTAQM6HK365M3J6LVAOBY6VCEUGRCU


token-get-name:
	stellar contract invoke \
		--id CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75 \
		--source $(ADMIN_ALIAS) \
		--send=no \
		--network $(NETWORK) 	\
		-- \
		name

wrap-token:
	stellar contract asset deploy \
		--network $(NETWORK) 	\
		--source  $(ADMIN_ALIAS) \
		--asset USDY:GAYODJWF27E5OQO2C6LA6Z6QXQ2EYUONMXFNL2MNMGRJP6RED2CPQKTW

native-token-address:
	stellar contract asset id \
 		--network $(NETWORK) \
 		--asset native

generate-types: generate-types-gas-oracle generate-types-pool generate-types-bridge generate-types-messenger generate-types-token

generate-types-gas-oracle:
	stellar contract bindings typescript \
	--network $(NETWORK) \
	--output-dir ./types/gas-oracle \
	--contract-id $(GAS_ORACLE_ADDRESS)

generate-types-pool:
	stellar contract bindings typescript \
	--network $(NETWORK) \
	--output-dir ./types/pool \
	--contract-id $(POOL_ADDRESS)

generate-types-bridge:
	stellar contract bindings typescript \
	--network $(NETWORK) \
	--output-dir ./types/bridge \
	--contract-id $(BRIDGE_ADDRESS)

generate-types-messenger:
	stellar contract bindings typescript \
	--network $(NETWORK) \
	--output-dir ./types/messenger \
	--contract-id $(MESSENGER_ADDRESS)

generate-types-token:
	stellar contract bindings typescript \
	--network $(NETWORK) \
	--output-dir ./types/token \
	--contract-id $(TOKEN_ADDRESS)

generate-types-auto-deposit-factory:
	stellar contract bindings typescript \
		--network $(NETWORK) \
		--output-dir ./types/auto-deposit-factory \
		--contract-id $(AUTO_DEPOSIT_FACTORY_ADDRESS)

# ----------------------------- Auto deposit ---------------------------------------

auto-deposit-wallet-upload:
	stellar contract upload \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) \
		--wasm $(AUTO_DEPOSIT_WALLET_WASM_PATH_OP)

deploy-auto-deposit-factory:
	stellar contract deploy \
      --wasm $(AUTO_DEPOSIT_FACTORY_WASM_PATH_OP) \
      --source $(ADMIN_ALIAS) \
      --network $(NETWORK) \
			-- \
			--admin $(ADMIN) \
			--native_token_address $(NATIVE_ADDRESS) \
			--gas_oracle_address $(GAS_ORACLE_ADDRESS) \
			--bridge $(BRIDGE_ADDRESS) \
			--send_tx_cost 1000000000000000000 \
			--wallet_wasm_hash $(AUTO_DEPOSIT_WALLET_WASM_HASH) \
      > $(AUTO_DEPOSIT_FACTORY_ADDRESS_PATH) && echo $(AUTO_DEPOSIT_FACTORY_ADDRESS)

auto-deposit-factory-create-deposit-wallet:
	stellar contract invoke \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		--network $(NETWORK) \
		--source $(ADMIN_ALIAS) \
		-- \
		create_deposit_wallet \
			--sender GD4A45PEPZBWYBDEQZYHDSEDML76J4JJTUTO2FHYFCKLD5O3YXOY6QIK \
			--recipient GD4A45PEPZBWYBDEQZYHDSEDML76J4JJTUTO2FHYFCKLD5O3YXOY6QIK \
			--recipient_token CAOPX7DVI3PFLHE7637YSFU6TLG6Z27Z5O3M547ANAYXQOAYCYYV6NO6 \
			--min_deposit_amount 1 \
			--fee_token_amount 140000000 \
			--chain-ids [4] 

auto-deposit-factory-update-contract:
	stellar contract invoke \
		--id $(AUTO_DEPOSIT_FACTORY_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) \
        -- \
        upgrade \
        --new_wasm_hash db345359017e405e529afebf51764e46bff3b5ed7ea9fbfdbea49eac5b232c01

install-cli:
	brew install stellar-cli
