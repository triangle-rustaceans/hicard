build:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js

install:
	cargo install --force wasm-pack
	npm install --global rollup

