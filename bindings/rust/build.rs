use std::path::Path;
extern crate cc;

fn main() {
    let src_dir = Path::new("src");
    let parser_path = src_dir.join("parser.c");
    let sysroot_dir = Path::new("wasm-sysroot");

    let mut compiler = cc::Build::new();

    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        compiler
            .archiver("llvm-ar")
            .include(&sysroot_dir);
    }

    compiler
        .include(&src_dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .file(&parser_path)
        .compile("parser");

    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
}
