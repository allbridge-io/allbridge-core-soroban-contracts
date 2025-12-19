BUILD:=stellar contract build --optimize --profile release --package
INSTALL:=stellar contract install --source $(ADMIN_ALIAS) --network $(NETWORK) --wasm 
INVOKE:= stellar contract invoke --source $(ADMIN_ALIAS) --network $(NETWORK)
INVOKE_VIEW:=$(INVOKE) --send=no
DEPLOY:=stellar contract deploy --source $(ADMIN_ALIAS) --network $(NETWORK) --wasm
