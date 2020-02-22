.PHONY: build
build: prepare
	./node_modules/.bin/pkg . --out-dir bin

.PHONY: prepare
prepare:
	npm install

.PHONY: install
install:
	cp ./bin/kubectl-spy /usr/local/bin/
