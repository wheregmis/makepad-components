run:
	cargo run

run_wasm:
	cargo makepad wasm run -p makepad-gallery --release

build_wasm:
	./scripts/build_wasm.sh -p makepad-gallery --profile small --no-threads
