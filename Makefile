.PHONY: all
all: deb

.PHONY: libs
libs:
	$(MAKE) -C libs/

.PHONY: binary
binary: libs
	cargo zigbuild \
		--target arm-unknown-linux-gnueabihf \
		--release \
		--features midi

.PHONY: deb 
deb: binary
	cargo deb \
		--target arm-unknown-linux-gnueabihf \
		--no-build
