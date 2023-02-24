use std::path::Path;
use std::env;

fn main() 
{
    cc::Build::new()
        .file("src/hideinjector.c")
        .compile("hideinjector");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());
}
