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
cd rust-webpack
nvm use 10
npm init rust-webpack
```

If you get a [compiler error](https://github.com/rustwasm/rust-webpack-template/issues/44):

```
cd crate
cargo update
```

2. Plain

```bash
cargo new --lib utils
```

Add inside Cargo.toml

```
[lib]
crate-type = ["cdylib"]
```

## Build

```
cd utils
wasm-pack build
```

## Start 

```
cd utils
npx webpack-dev-server
```