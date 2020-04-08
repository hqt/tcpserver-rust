build:
	cargo build
.PHONY: build

server: build
	./target/debug/server
.PHONY:client

client: build
	./target/debug/client
.PHONY:client
