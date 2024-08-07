.PHONY: all
all: binary

.PHONY: libs
libs:
	$(MAKE) -C libs/

.PHONY: binary
binary: libs
	cargo zigbuild \
		--target arm-unknown-linux-gnueabihf \
		--release \
		--features midi
