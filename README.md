# wasm_config_dev

### Requirements
- Kubernetes
```
k8s > 1.24
istio > 1.18
envoy > 1.28
# also can run `higress` wasm filter
```
- Rust
add new target
```shell
rustup update
rustup target add wasm32-wasip1
```

- Compile
```shell
make build
```

### Check
```shell
curl 127.0.0.1:8080/headers
## response
{
  "headers": {
    "Accept": "*/*",
    "Aa": "bb",
    "Host": "127.0.0.1",
    "User-Agent": "curl/8.12.1",
    "Ab": "bc",
    "X-Amzn-Trace-Id": "Root=1-68bbad97-00ddb97e43b8db570bc9c1b7",
    "X-Envoy-Expected-Rq-Timeout-Ms": "15000"
  }
}
```
