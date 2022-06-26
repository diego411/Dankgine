# Dankgen

## Description

*TODO*
Something something Physics. Something something balls.

## Build

1. Use `wasm-pack` to compile our Rust files into a wasm/js bundle.

- With SIMD support.

To enable the simd feature, we must build on nightly. In which case, use the following.
```sh
rustup run nightly \
  wasm-pack build \
  --target web \
  --out-dir ../web-debug/pkg \
  --release \
  -- \
  --all-features
```

- Without SIMD (default)

Use stable for crate features that don't need nightly features.
```sh 
wasm-pack build \
  --target web \ 
  --out-dir ../web-debug/pkg \
  --release
```


2. Run an HTTP server in the `./web-debug` directory to start the visualization.

```sh
python3 -m http.server --bind 0.0.0.0 8000 --directory ./web-debug
```
