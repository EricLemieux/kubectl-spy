APPLICATION := ./target/release/kubectl-spy
INSTALL_DIR := /usr/local/bin

.PHONY: build
build:
	cargo build --release

.PHONY: test-e2e
test-e2e:
	kubectl apply -f kubernetes/
	./scripts/end-to-end-test.sh

.PHONY: install
install:
	cp $(APPLICATION) $(INSTALL_DIR)
