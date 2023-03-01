# tree-sitter-python-wasm-compatible

Python grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter) modified so that a Rust project targeting web assembly can use it.

## Modifications

- `scanner.cc` was manually ported to `scanner.rs`
- `build.rs` was modified so it would use a minimal wasm sysroot and `llvm-ar` when compiling for wasm
- uses `tree-sitter-c2rust`

## Verification

The corpus tests are run via `rust-tester`. It is just the code from the `tree-sitter test` command ([source](https://github.com/tree-sitter/tree-sitter/blob/master/cli/src/test.rs)] bound directly to the rust library.

`scanner.rs` was also run through miri because of the large amount of unsafe code.
