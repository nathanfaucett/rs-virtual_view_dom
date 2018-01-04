

all: examples

counter:
	cargo web build --example counter --target-webasm
	make copy_counter

simple:
	cargo web build --example simple --target-webasm
	make copy_simple

examples: counter simple

copy_counter:
	cp target/wasm32-unknown-unknown/release/examples/counter.wasm examples

copy_simple:
	cp target/wasm32-unknown-unknown/release/examples/simple.wasm examples

clean:
	rm examples/counter.wasm examples/simple.wasm


.PHONY: all counter simple examples copy_counter copy_simple clean
