fn main() {
    let src_dir = std::path::Path::new("src");

    let parser_path = src_dir.join("parser.c");
    let scanner_path = src_dir.join("scanner.c");

    let mut c_config = cc::Build::new();

    // set minimal C sysroot if wasm32-unknown-unknown
    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        let sysroot_dir = std::path::Path::new("bindings/rust/wasm-sysroot");
        c_config
            .file(sysroot_dir.join("impl").join("string.c"))
            .flag("-mbulk-memory")
            .archiver("llvm-ar")
            .include(&sysroot_dir.join("headers"));
    }

    c_config
        .include(src_dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .file(&parser_path)
        .file(&scanner_path)
        .compile("parser");

    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
}
