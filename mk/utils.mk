token-transfer:
	$(INVOKE) \
		--id $(TOKEN_ADDRESS) \
		-- \
		transfer \
		--from GDL27JZFDPBXX7B4DTWPSEWRFHGTAQM6HK365M3J6LVAOBY6VCEUGRCU \
		--to GCDIRA4GRYWVUWB33F6UQUQYAN4L5FXIWJRFPJQMG5YS5EQTWIKAMONK \
		--amount 1000

token-native-transfer:
	$(INVOKE) \
		--id $(NATIVE_ADDRESS) \
		-- \
		transfer \
		--from $(ADMIN) \
		--to $(BRIDGE_ADDRESS) \
		--amount 10000000000

token-get-balance:
	$(INVOKE_VIEW) \
		--id $(TOKEN_ADDRESS) \
		-- \
		balance \
		--id GDL27JZFDPBXX7B4DTWPSEWRFHGTAQM6HK365M3J6LVAOBY6VCEUGRCU

token-get-name:
	$(INVOKE_VIEW) \
		--id CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75 \
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
