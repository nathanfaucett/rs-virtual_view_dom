

all: examples

example_counter:
	cargo web build --example counter --target-webasm
	make copy_counter

example_simple:
	cargo web build --example simple --target-webasm
	make copy_simple

examples: example_counter example_simple

copy_counter:
	cp target/wasm32-unknown-unknown/release/examples/counter.wasm examples

copy_simple:
	cp target/wasm32-unknown-unknown/release/examples/simple.wasm examples

clean:
	rm examples/counter.wasm examples/simple.wasm


.PHONY: all example_counter example_simple examples copy_counter copy_simple clean
