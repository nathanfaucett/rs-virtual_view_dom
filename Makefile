all: examples

counter:
	cargo web build --example counter --target asmjs-unknown-emscripten
	make copy_counter

examples: counter

copy_counter:
	cp target/asmjs-unknown-emscripten/debug/examples/counter.js examples

clean:
	rm examples/counter.js


.PHONY: all counter examples copy_counter clean
