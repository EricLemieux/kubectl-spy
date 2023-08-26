.PHONY: build
build:
	cargo build --release

.PHONY: test-e2e
test-e2e:
	kubectl apply -f kubernetes/
	./scripts/end-to-end-test.sh

.PHONY: install
install:
	cargo install --path .
