# yew-suzuri

## Development

### 1. Install `Rust`, `wasm-pack` and `miniserve`

Follow the instructions at https://www.rust-lang.org/tools/install and follow the `installation` link at [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

And finally, install miniserve.
```
cargo install miniserve
```

### 2. Environment vairables

Copy `.env.example` to create `.env`.
Enter the api key of the application you created at https://suzuri.jp/developer/apps into the `API_KEY` field in `.env`.

### 3. Build

```
npm run build
```

### 4. Run

```
npm run dev
```