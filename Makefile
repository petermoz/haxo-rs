TARGET ?= aarch64-unknown-linux-gnu
VERGEN_GIT_DESCRIBE := $(shell git describe --always --dirty --tags)

.PHONY: all
all: build deb

.PHONY: build
build:
	VERGEN_GIT_DESCRIBE=$(VERGEN_GIT_DESCRIBE) cross build --release --features midi --target $(TARGET)

.PHONY: deb
deb:
	cargo deb --no-build --no-strip --target $(TARGET)
