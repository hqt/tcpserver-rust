build:
	cargo build
.PHONY: build

test:
	cargo test
.PHONY: test

server: build
	./target/debug/server
.PHONY:client

client: build
	./target/debug/client
.PHONY:client
