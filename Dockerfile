FROM scratch

COPY target/wasm32-wasi/release/wasm_config_dev.wasm /plugin.wasm
