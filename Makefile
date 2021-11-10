RELEASE =

CARGO_TARGET := $(if $(RELEASE),release,debug)

CARGO_OPTS += $(if $(RELEASE),--release,)

BIN_SRCS = $(wildcard bin/src/*.rs)

all: lib/src/lib.rs target/$(CARGO_TARGET)/libgithub_openapi.rlib

publish: all
	cd lib && cargo publish

bin: Makefile $(BIN_SRCS)
	cargo build --workspace $(CARGO_OPTS) --bin github-openapi-builder

lib/src/lib.rs: api.github.com.json bin Makefile $(BIN_SRCS)
	target/$(CARGO_TARGET)/github-openapi-builder $< $@

target/$(CARGO_TARGET)/libgithub_openapi.rlib: lib/src/lib.rs
	cargo build --workspace $(CARGO_OPTS) --lib

.PHONY: all bin
