# rust-demo

## Installing Rustup
Rustup is the primary distribution of Rust.
It provides both stable and nightly versions of the rustc compiler
and the cargo build tool.
It can be installed with the command:  

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Once rustup is installed, Rust can be updated with the command:  
```shell
rustup update
```

## Using Cargo

The following commands provide a brief overview of the cargo workflow:

```shell
# create a new binary project
cargo new my-project #use --lib for a library project
cd my-project

# edit Cargo.toml to add dependencies in the form
# dep-name = "x.x.x"

# edit src/main.rs then compile with:
# this also downloads and builds dependencies
cargo build

# run your binary
# this will also compile if there are changes
cargo run
```

