SNIPPETS_DIR := ../rust_cp_workspace/.vscode
SNIPPETS_FILE := libraries
SNIPPETS_SUFFIX := code-snippets
KATEX_FLAG := "--html-in-header katex.html"

fmt:
	cargo fmt

lint:
	cargo clippy --workspace -- -D warnings

check:
	cargo +1.70.0 check --workspace

test:
	cargo test --workspace

verify:
	oj-verify run

coverage:
	cargo +nightly llvm-cov --workspace --doctests --lcov --output-path lcov.info
	cargo +nightly llvm-cov --workspace --examples --lcov --output-path lcov_ex.info

snippet:
	mkdir -p $(SNIPPETS_DIR)
	cargo snippet -t vscode crates > $(SNIPPETS_DIR)/$(SNIPPETS_FILE).$(SNIPPETS_SUFFIX)

build: fmt lint check coverage verify snippet

doc:
	RUSTDOCFLAGS=$(KATEX_FLAG) cargo doc --workspace --no-deps
