# yew-suzuri

## Development

### 1. Install

Follow the instructions at https://www.rust-lang.org/tools/install .

```sh
npm install                      # install Sass & Nachiguro
cargo install wasm-pack          # Compile Rust to Wasm and generate JS interop code
cargo install cargo-make         # Task runner
cargo install simple-http-server # Simple server to serve assets
```

### 2. Set Constants

Copy `src/constants.rs.example` to create `src/constants.rs`.
Enter the api key of the application you created at https://suzuri.jp/developer/apps into the `API_KEY` field in `src/constants.rs`.

### 3. Build

```sh
cargo make build
```

### 4. Run

```sh
cargo make serve
```

## Deployment

### 1. Install

```sh
npm install -g firebase-tools
```

### 2. Login

```sh
firebase login
```

### 3. Deploy

```sh
firebase deploy
```