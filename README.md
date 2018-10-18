# rust-wasm-playground

## Install / configure

```bash
brew install rustup
rustup-init
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install wasm-gc
cargo install https
```

## Create a new project

1. Webpack

```bash
cd webpack
nvm use 10
npm init rust-webpack
```

If you get a [compiler error](https://github.com/rustwasm/rust-webpack-template/issues/44):

```
cd crate
cargo update
wasm-pack build
```

2. Plain / backend

```bash
cargo new --lib utils
```

Add inside Cargo.toml

```
[lib]
crate-type = ["cdylib"]
```

## Build

1. Plain 

```
cd utils
wasm-pack build
```

2. backend

```
cd backend/crate
cargo build --target wasm32-unknown-unknown --release
```

## Start 

1. Webpack

```
cd webpack
npm start
```

2. Plain

```
cd utils
npx webpack-dev-server
```

3. backend

```
cd backend
nvm use 10
node --experimental-modules --loader @wasm-tool/node index.mjs
```
