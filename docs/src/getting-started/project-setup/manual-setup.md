# Manual Setup

⚠️ This is not the recommended way to start a `wasm-pack` project! If you ended up
here by mistake, go check out our recommended project start, [Using A Template].

[Using A Template]: using-a-template.md

### Step 1: Create a New Rust Library Project

You can create a new Rust project named `my-lib` using this command.

```
cargo new --lib my-lib
```

The `--lib` flag specifies that the project is a library, which is important
because we will be calling this code from JavaScript.

### Step 2: Edit your `Cargo.toml` File


#### Add the `wasm-bindgen` depedency

You will need to add `wasm-bindgen` to your `Cargo.toml` in the dependencies
section. `wasm-bindgen` is a tool that facilitates interoperability between
wasm modules and JavaScript.

⚠️ If you are coming from JavaScript, you might note that when we add the dependency
there is no `^` or `~` symbol- it looks like we're locking to the `0.2` version. 
However, that's not the case! In Rust, the `^` is implied.

#### Add `crate-type`

Next, add a `[lib]` section, with a new field named `crate-type` set to
`"cdylib"`. This specifies that the library is a C compatible dynamic library,
which helps `cargo` pass the correct flags to the Rust compiler when targeting
`wasm32`.

After making these changes, your `Cargo.toml` file should look something like
this:

```
[package]
name = "hello-wasm
version = "0.1.0"
authors = ["Ashley Williams <ashley666ashley@gmail.com>"]
description = "babby's first wasm package"
license = "MIT/Apache-2.0"
repository = "https://github.com/ashleygwilliams/hello-wasm"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen="0.2"
```
