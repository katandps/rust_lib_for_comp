SNIPPETS_DIR := ../rust_cp_workspace/.vscode
SNIPPETS_FILE := libraries
SNIPPETS_SUFFIX := code-snippets
KATEX_FLAG := "--html-in-header katex.html"

VERIFY_FILE_PATH := ".competitive-verifier/verify.json"
VERIFY_RESULT_PATH := ".competitive-verifier/result.json"

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

verify-resolve:
	competitive-verifier oj-resolve > $(VERIFY_FILE_PATH)

verify-download: verify-resolve
	competitive-verifier download --verify-json $(VERIFY_FILE_PATH)

# verify libraries
verify: verify-download
	competitive-verifier verify --verify-json $(VERIFY_FILE_PATH) --output $(VERIFY_RESULT_PATH)

verify-doc:
	competitive-verifier docs --verify-json $(VERIFY_FILE_PATH) $(VERIFY_RESULT_PATH)

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
	cd .competitive-verifier/_jekyll && bundle add webrick &&bundle exec jekyll serve --incremental