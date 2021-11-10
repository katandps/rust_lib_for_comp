SNIPPETS_FILE := ../rust_cp_workspace/.vscode/libraries.code-snippets

fmt:
	cargo fmt
test:
	cargo +1.42.0 test
snippet:
	cargo snippet -t vscode > $(SNIPPETS_FILE)

build: fmt test snippet