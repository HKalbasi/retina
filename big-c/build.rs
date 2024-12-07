use std::{env, path::Path};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=big");
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("src").display());
}
