
run:
	cargo run

build:
	cargo build

install-wasm-prereqs:
	cargo install -f wasm-bindgen-cli
	cargo install wasm-server-runner

install-wasm: install-wasm-prereqs
	rustup target install wasm32-unknown-unknown


run-wasm: install-wasm
	# Run a minimal server with the game compiled into WASM
	cargo run --release --target wasm32-unknown-unknown

watch-wasm:
	cargo watch -cx "run --release --target wasm32-unknown-unknown"

build-wasm: install-wasm
	rm -r out/assets
	cp -r assets/ out/assets/
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/double_slit.wasm
