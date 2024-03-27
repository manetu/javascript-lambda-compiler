# Manetu Javascript Compiler (mjsc) for Lambda Functions

This project implements a Javascript to WebAssembly compiler for writing Lambda functions for the Manetu platform in Javascript and Javascript-based languages (e.g., Typescript, Clojurescript, etc.).  It is based on [Javy](https://github.com/bytecodealliance/javy), following the [Extending Javy](https://github.com/bytecodealliance/javy/blob/main/docs/extending.md) guidelines.

## Runtime requirements

When running the official mjsc binary on Linux, `glibc` 2.31 or greater must be available.  If you are using an older version of `glibc,` you may need to update your operating system.

## Requirements to build

- On Ubuntu, `sudo apt-get install curl pkg-config libssl-dev clang`
- [rustup](https://rustup.rs/)
- Stable Rust, installed via `rustup install stable && rustup default stable`
- wasm32-wasi, can be installed via `rustup target add wasm32-wasi`
- cmake, depending on your operating system and architecture, it might not be
  installed by default.  On MacOS, it can be installed with `homebrew` via `brew
  install cmake`  On Ubuntu, `sudo apt-get install cmake`
- Rosetta 2, if running MacOS on Apple Silicon, can be installed via
  `softwareupdate --install-rosetta`

## How to build

Inside this repository, run:
```
$ cargo build -p mjsc-core --target=wasm32-wasi -r
$ cargo build -p mjsc-cli -r
```

Alternatively, if you want to install the tool chain globally, inside this repository, run:
```
$ cargo build -p mjsc-core --target=wasm32-wasi -r
$ cargo install --path crates/mjsc-cli
```
If you are going to recompile frequently, you may want to prepend CARGO_PROFILE_RELEASE_LTO=off to cargo build for the CLI to speed up the build.