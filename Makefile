.PHONY: prepare
prepare:
	npm install

.PHONY: test
test:
	npm test

.PHONY: test-e2e
test-e2e:
	kubectl create namespace database
	kubectl apply -f kubernetes/
	./bin/kubectl-spy my-secret-name
	./bin/kubectl-spy my-secret-name another-secret
	./bin/kubectl-spy database-secret -n database

.PHONY: build
build:
	./node_modules/.bin/pkg . --out-dir bin

.PHONY: install
install:
	cp ./bin/kubectl-spy /usr/local/bin/
