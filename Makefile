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
	cargo +1.42.0 test

snippet:
	mkdir -p $(SNIPPETS_DIR)
	cargo snippet -t vscode > $(SNIPPETS_DIR)/$(SNIPPETS_FILE)

build: fmt lint test snippet

doc:
	RUSTDOCFLAGS=$(KATEX_FLAG) cargo doc --no-deps