RELEASE =

CARGO_TARGET := $(if $(RELEASE),release,debug)

CARGO_OPTS += $(if $(RELEASE),--release,)

BIN_SRCS = $(wildcard bin/target/src/*.rs)

all: lib/src/lib.rs

bin/target/$(CARGO_TARGET)/github-openapi-builder: Makefile $(BIN_SRCS)
	cd bin && cargo build $(CARGO_OPTS)

lib/src/lib.rs: api.github.com.yaml Makefile $(BIN_SRCS)
	bin/target/$(CARGO_TARGET)/github-openapi-builder $< $@

