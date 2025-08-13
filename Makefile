.PHONY: build install

BUILD_DIR   = target/release
INSTALL_DIR = /usr/local/bin
CONFIG_DIR  = /etc/pind

build:
	cargo build --release

install: build
	sudo cp $(BUILD_DIR)/pindd $(INSTALL_DIR)/
	sudo cp ./pindc $(INSTALL_DIR)/
	sudo chmod 755 $(INSTALL_DIR)/pindc
	sudo chmod 755 $(INSTALL_DIR)/pindd
	sudo mkdir -p $(CONFIG_DIR)
	sudo cp pindrc $(CONFIG_DIR)

uninstall:
	sudo rm /usr/local/bin/pindd
	sudo rm /usr/local/bin/pindc
	sudo rm -r $(CONFIG_DIR) 
