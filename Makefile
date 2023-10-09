SNIPPETS_DIR := ../rust_cp_workspace/.vscode
SNIPPETS_FILE := libraries
SNIPPETS_SUFFIX := code-snippets
KATEX_FLAG := "--html-in-header katex.html"

# format project files
fmt:
	cargo fmt

# lint project files
lint:
	cargo clippy --workspace -- -D warnings

# compile check project files
check:
	cargo +1.70.0 check --workspace

# unit test
test:
	cargo test --workspace

# verify libraries
verify:
	competitive-verifier oj-resolve > .competitive-verifier/verify.json
	competitive-verifier verify --verify-json .competitive-verifier/verify.json --output .competitive-verifier/result.json

verify-doc:
	competitive-verifier docs --verify-json .competitive-verifier/verify.json .competitive-verifier/result.json

# test and make coverage
coverage:
	cargo +nightly llvm-cov --doctests --lcov --output-path lcov.info

# generate vscode snippet file
# see SNIPPETS_DIR
snippet:
	mkdir -p $(SNIPPETS_DIR)
	cargo snippet -t vscode crates > $(SNIPPETS_DIR)/$(SNIPPETS_FILE).$(SNIPPETS_SUFFIX)

# test and generate snippet
build: fmt lint check coverage verify snippet

# generate rustdoc
rustdoc:
	RUSTDOCFLAGS=$(KATEX_FLAG) cargo doc --workspace --no-deps

# build verification docs
doc: verify-doc
	cd .competitive-verifier/_jekyll && bundle exec jekyll serve --incremental