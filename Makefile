SNIPPETS_DIR := ../rust_cp_workspace/.vscode
SNIPPETS_FILE := libraries.code-snippets
KATEX_FLAG := "--html-in-header katex.html"

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check:
	cargo +1.42.0 check

test:
	cargo test

snippet:
	mkdir -p $(SNIPPETS_DIR)
	fd "Cargo.toml" crates | xargs dirname | xargs -i sh -c 'cd {}; cargo snippet -t vscode lib.rs > snippet;'
	cargo snippet -t vscode > $(SNIPPETS_DIR)/$(SNIPPETS_FILE)

build: fmt lint check test snippet

doc:
	RUSTDOCFLAGS=$(KATEX_FLAG) cargo doc --no-deps
