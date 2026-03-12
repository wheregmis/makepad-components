run:
	cargo run

run_wasm:
	cargo makepad wasm run -p makepad-example-component-gallery --release

build_wasm:
	./scripts/build_wasm.sh -p makepad-example-component-gallery --profile small --no-threads
