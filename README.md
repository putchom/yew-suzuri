# yew-todo

## Development

### 1. Install `Rust` and `wasm-pack`

Follow the instructions at https://www.rust-lang.org/tools/install and follow the `installation` link at [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

### 2. Build

```
wasm-pack build --target web --out-name wasm --out-dir ./static
```

### 3. Run

```
cargo install miniserve
miniserve ./static --index index.html
```