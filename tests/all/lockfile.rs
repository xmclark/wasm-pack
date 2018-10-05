use utils::fixture;
use wasm_pack::lockfile::Lockfile;

#[test]
fn it_gets_wasm_bindgen_version() {
    let fixture = fixture::js_hello_world();
    fixture.cargo_check();
    let lock = Lockfile::new(&fixture.path).unwrap();
    assert_eq!(lock.wasm_bindgen_version(), Some("0.2.21"),);
}

#[test]
fn it_gets_wasm_bindgen_test_version() {
    let fixture = fixture::wbg_test_node();
    fixture.cargo_check();
    let lock = Lockfile::new(&fixture.path).unwrap();
    assert_eq!(lock.wasm_bindgen_test_version(), Some("0.2.21"),);
}

#[test]
fn it_gets_wasm_bindgen_version_in_crate_inside_workspace() {
    let fixture = fixture::Fixture::new();
    fixture
        .file(
            "Cargo.toml",
            r#"
                [workspace]
                members = ["./blah"]
            "#,
        )
        .file(
            "blah/Cargo.toml",
            r#"
                [package]
                authors = ["The wasm-pack developers"]
                description = "so awesome rust+wasm package"
                license = "WTFPL"
                name = "blah"
                repository = "https://github.com/rustwasm/wasm-pack.git"
                version = "0.1.0"

                [lib]
                crate-type = ["cdylib"]

                [dependencies]
                wasm-bindgen = "=0.2.21"
            "#,
        )
        .file(
            "blah/src/lib.rs",
            r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn hello() -> u32 { 42 }
            "#,
        );
    fixture.cargo_check();
    let lock = Lockfile::new(&fixture.path.join("blah")).unwrap();
    assert_eq!(lock.wasm_bindgen_version(), Some("0.2.21"),);
}

#[test]
fn it_gets_wasm_bindgen_version_from_dependencies() {
    let fixture = fixture::Fixture::new();
    fixture
        .file(
            "Cargo.toml",
            r#"
                [workspace]
                members = ["./parent", "./child"]
            "#,
        )
        .file(
            "child/Cargo.toml",
            r#"
                [package]
                authors = ["The wasm-pack developers"]
                description = "so awesome rust+wasm package"
                license = "WTFPL"
                name = "child"
                repository = "https://github.com/rustwasm/wasm-pack.git"
                version = "0.1.0"

                [lib]
                crate-type = ["cdylib"]

                [dependencies]
                wasm-bindgen = "=0.2.21"
            "#,
        )
        .file(
            "child/src/lib.rs",
            r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn hello() -> u32 { 42 }
            "#,
        )
        .file(
            "parent/Cargo.toml",
            r#"
                [package]
                authors = ["The wasm-pack developers"]
                description = "so awesome rust+wasm package"
                license = "WTFPL"
                name = "parent"
                repository = "https://github.com/rustwasm/wasm-pack.git"
                version = "0.1.0"

                [lib]
                crate-type = ["cdylib"]
            "#,
        )
        .file(
            "parent/src/lib.rs",
            r#"
                // Just re-export all of `child`.
                extern crate child;
                pub use child::*;
            "#,
        );
    fixture.cargo_check();
    let lock = Lockfile::new(&fixture.path.join("parent")).unwrap();
    assert_eq!(lock.wasm_bindgen_version(), Some("0.2.21"),);
}
