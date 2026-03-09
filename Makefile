ROOT := /home/tzachb/Projects/zcodex
CODEX_RS := $(ROOT)/codex-rs
ZCODEX_BIN := $(CODEX_RS)/target/release/zcodex
DEBUG_ZCODEX_BIN := $(CODEX_RS)/target/debug/zcodex
CARGO_GIT_ENV := CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=url.https://github.com/.insteadOf GIT_CONFIG_VALUE_0=ssh://git@github.com/

.PHONY: help build build-fast run version test fmt build-zcodex build-zcodex-debug run-zcodex version-zcodex test-image-preview fmt-zcodex

help:
	@printf "%s\n" "Targets"
	@printf "%s\n" "  make build       Build release zcodex binary"
	@printf "%s\n" "  make build-fast  Build debug zcodex binary"
	@printf "%s\n" "  make run         Run debug zcodex"
	@printf "%s\n" "  make version     Print release zcodex version"
	@printf "%s\n" "  make test        Run focused image preview tests"
	@printf "%s\n" "  make fmt         Run cargo fmt in codex-rs"

build:
	cd $(CODEX_RS) && $(CARGO_GIT_ENV) cargo build --release -p codex-cli --bin zcodex

build-fast:
	cd $(CODEX_RS) && cargo build -p codex-cli --bin zcodex

run:
	cd $(CODEX_RS) && cargo run -p codex-cli --bin zcodex

version:
	$(ZCODEX_BIN) --version

test:
	cd $(CODEX_RS) && cargo test -p codex-tui image_preview

fmt:
	cd $(CODEX_RS) && cargo fmt -- --config imports_granularity=Item 2>/dev/null

build-zcodex: build

build-zcodex-debug: build-fast

run-zcodex: run

version-zcodex: version

test-image-preview: test

fmt-zcodex: fmt
