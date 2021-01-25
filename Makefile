APPLICATION := ./target/release/kubectl-spy
INSTALL_DIR := /usr/local/bin

.PHONY: build
build:
	cargo build --release

.PHONY: test-e2e
test-e2e:
	kubectl create namespace database
	kubectl apply -f kubernetes/
	$(APPLICATION) my-secret-name
	$(APPLICATION) my-secret-name another-secret
	$(APPLICATION) database-secret -n database

.PHONY: install
install:
	cp $(APPLICATION) $(INSTALL_DIR)
