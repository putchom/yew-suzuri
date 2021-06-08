# yew-suzuri

## Development

### 1. Install `Rust` and `wasm-pack`

Follow the instructions at https://www.rust-lang.org/tools/install and follow the `installation` link at [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

### 2. Environment vairables

Copy `.env.example` to create `.env`.
Enter the api key of the application you created at https://suzuri.jp/developer/apps into the `API_KEY` field in `.env`.

### 3. Build

```
wasm-pack build --target web --out-name wasm --out-dir ./static
```

### 4. Run

```
cargo install miniserve
miniserve ./static --index index.html
```