# Rust

`wasm-pack` is a Command Line Interface tool written in Rust, and distributed with `cargo`.
As a result, you'll need Rust and `cargo` to use `wasm-pack`.

### Installing Rust and Cargo

To install Rust, visit this [page](https://www.rust-lang.org/en-US/install.html), which will
walk you through installing Rust and `cargo` on your machine using a tool called `rustup`.

To confirm you have Rust and `cargo` installed, run:

```
rustc --version
cargo --version
```

### `nightly` Rust

`wasm-pack` depends on `wasm-bindgen` which currently requires Rust features that
have not yet been stabilized. As a result, you'll need to use the latest nightly
version of Rust to run `wasm-pack`.

⚠️ `nightly` Rust, is by definition, unstable! Sometimes things will break or not work. You
can expect that they'll be fixed within a day or so, but it's important to know that 
breakage is both possible, and often expected.

You can install the `nightly` channel of Rust by running:

```
rustup install nightly
```

You can update your `nightly` version by running:

```
rustup update nightly
```

If you find yourself switching between Rust channels often, you can set the default
channel for a project directory by running this command in that directory:

```
rustup override set nightly
```
