# yew-suzuri

## Development

### 1. Install

Follow the instructions at https://www.rust-lang.org/tools/install .

```sh
cargo install wasm-pack          # Compile Rust to Wasm and generate JS interop code
cargo install cargo-make         # Task runner
cargo install simple-http-server # Simple server to serve assets
```

### 2. Set Environment vairables

Copy `.env.example` to create `.env`.
Enter the api key of the application you created at https://suzuri.jp/developer/apps into the `API_KEY` field in `.env`.

### 3. Build

```sh
cargo make build
```

### 4. Run

```sh
cargo make serve
```