.PHONY: build install

BUILD_DIR   = target/release
INSTALL_DIR = /usr/local/bin
CONFIG_DIR  = $(HOME)/.config/pind

build:
	cargo build --release

install: build
	sudo cp $(BUILD_DIR)/pindd $(INSTALL_DIR)/
	sudo cp ./pindc $(INSTALL_DIR)/
	sudo chmod 755 $(INSTALL_DIR)/pindc
	sudo chmod 755 $(INSTALL_DIR)/pindd
	mkdir -p $(CONFIG_DIR)
	cp -n pindrc $(CONFIG_DIR)/ 2>/dev/null || true

uninstall:
	sudo rm /usr/local/bin/pindd
	sudo rm /usr/local/bin/pindc
	sudo rm -r $(CONFIG_DIR) 
