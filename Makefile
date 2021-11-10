RELEASE =

CARGO_TARGET := $(if $(RELEASE),release,debug)

CARGO_OPTS += $(if $(RELEASE),--release,)

BIN_SRCS = $(wildcard bin/target/src/*.rs)

all: lib/src/lib.rs lib/target/$(CARGO_TARGET)/libgithub_openapi.rlib

bin: Makefile $(BIN_SRCS)
	cd bin && cargo build $(CARGO_OPTS)

lib/src/lib.rs: api.github.com.json bin Makefile $(BIN_SRCS)
	bin/target/$(CARGO_TARGET)/github-openapi-builder $< $@

lib/target/$(CARGO_TARGET)/libgithub_openapi.rlib: lib/src/lib.rs
	cd lib && cargo build $(CARGO_OPTS)

.PHONY: all bin
