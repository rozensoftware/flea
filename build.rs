use std::path::Path;
use std::env;

fn main() 
{
    println!("cargo:rerun-if-changed=src/hideinjector.c");
    println!("cargo:rerun-if-changed=src/lockscreen.c");
    
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .file("src/hideinjector.c")
        .file("src/lockscreen.c")
        .compile("hideinjector");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();    
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());
}
