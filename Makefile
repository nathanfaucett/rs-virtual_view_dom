all: examples

children:
	cargo web build --example children --target wasm32-unknown-emscripten
	make copy_children

counter:
	cargo web build --example counter --target wasm32-unknown-emscripten
	make copy_counter

simple:
	cargo web build --example simple --target wasm32-unknown-emscripten
	make copy_simple

examples: children counter simple

copy_children:
	cp target/wasm32-unknown-emscripten/debug/examples/children.js examples
	find target/wasm32-unknown-emscripten/debug/examples/children-*.wasm -exec cp {} examples \;

copy_counter:
	cp target/wasm32-unknown-emscripten/debug/examples/counter.js examples
	find target/wasm32-unknown-emscripten/debug/examples/counter-*.wasm -exec cp {} examples \;

copy_simple:
	cp target/wasm32-unknown-emscripten/debug/examples/simple.js examples
	find target/wasm32-unknown-emscripten/debug/examples/simple-*.wasm -exec cp {} examples \;

clean:
	cargo clean
	find examples/*.js -exec rm {} \;
	find examples/*.wasm -exec rm {} \;


.PHONY: all children counter simple examples copy_children copy_counter copy_simple clean
