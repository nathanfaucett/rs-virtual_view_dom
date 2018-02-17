all: examples

children:
	cargo web build --example children --target asmjs-unknown-emscripten
	make copy_children

counter:
	cargo web build --example counter --target asmjs-unknown-emscripten
	make copy_counter

simple:
	cargo web build --example simple --target asmjs-unknown-emscripten
	make copy_simple

examples: children counter simple

copy_children:
	cp target/asmjs-unknown-emscripten/debug/examples/children.js examples

copy_counter:
	cp target/asmjs-unknown-emscripten/debug/examples/counter.js examples

copy_simple:
	cp target/asmjs-unknown-emscripten/debug/examples/simple.js examples

clean:
	rm examples/children.js
	rm examples/counter.js
	rm examples/simple.js


.PHONY: all children counter simple examples copy_children copy_counter copy_simple clean
