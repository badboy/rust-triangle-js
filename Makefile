build: triangle.js

triangle.js: target/asmjs-unknown-emscripten/release/triangle.js
	cp target/asmjs-unknown-emscripten/release/triangle.js .

target/asmjs-unknown-emscripten/release/triangle.js:
	cargo build --target asmjs-unknown-emscripten --release -v
