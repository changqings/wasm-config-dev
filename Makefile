.DEFAULT_GOAL := build
.PHONY: build clean run

VERSION := v0.0.1

build:
	cargo build \
	--target wasm32-wasip1 \
	--release

clean:
	cargo clean
run:
	sudo envoy -c ./envoy.yaml \
	--concurrency 1 \
	--log-format '%v'

build-image: build
	docker build -t xx/xxx/wasm_config_dev:${VERSION} .
	docker push  xx/xxx/wasm_config_dev:${VERSION}
