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
