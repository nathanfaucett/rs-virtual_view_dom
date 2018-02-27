all: examples

children:
	cargo web build --example children --target wasm32-unknown-unknown
	make copy_children

counter:
	cargo web build --example counter --target wasm32-unknown-unknown
	make copy_counter

simple:
	cargo web build --example simple --target wasm32-unknown-unknown
	make copy_simple

examples: children counter simple

copy_children:
	cp target/wasm32-unknown-unknown/release/examples/children.js examples
	cp target/wasm32-unknown-unknown/release/examples/children.wasm examples

copy_counter:
	cp target/wasm32-unknown-unknown/release/examples/counter.js examples
	cp target/wasm32-unknown-unknown/release/examples/counter.wasm examples

copy_simple:
	cp target/wasm32-unknown-unknown/release/examples/simple.js examples
	cp target/wasm32-unknown-unknown/release/examples/simple.wasm examples

clean:
	cargo clean
	find examples/*.js -exec rm {} \;
	find examples/*.wasm -exec rm {} \;


.PHONY: all children counter simple examples copy_children copy_counter copy_simple clean
