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

refresh-dropbox-token:
	curl https://api.dropbox.com/oauth2/token \
		-d grant_type=refresh_token \
		-d refresh_token=${DROPBOX_REFRESH_TOKEN} \
		-u ${DROPBOX_APP_KEY}:${DROPBOX_APP_SECRET} | \
		python3 -c "import sys, json; data=json.load(sys.stdin); print(data['access_token'], end='')" > temp
	DROPBOX_TOKEN=$(cat temp)
	rm temp

verify-resolve:
	competitive-verifier oj-resolve > $(VERIFY_FILE_PATH)

verify-download: verify-resolve refresh-dropbox-token
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