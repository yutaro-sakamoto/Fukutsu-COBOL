all:
	cargo build --release

install:
	cargo build --release
	sudo install target/release/fcbl /usr/bin

test:
	fcbl test/hello.cbl hello.js
	node hello.js

node:
	wasm-pack build --target=nodejs --out-dir=./fcbl-nodejs

web:
	wasm-pack build --target=web --out-dir=./fcbl-web

.PHONY: all test node web