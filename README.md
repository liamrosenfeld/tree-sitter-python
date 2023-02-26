# tree-sitter-python-c2rust

Python grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter) converted to Rust so that a Rust project targeting web assembly can use it.

## Conversion

### `parser.c`

Converted using [c2rust](https://github.com/immunant/c2rust).

Steps:

1. Update `compile_commands.json` to have the proper path for your system
2. Run `c2rust transpile --emit-modules compile_commands.json`
3. Run `mv src/parser.rs bindings/rust/parser.rs`
4. Delete the `INIT_ARRAY` at the bottom of the file and move the `TSLangage` initialization inline
5. Replace lines 1-19 with:

```rs
#![allow(warnings)]
use std::os::raw as libc;
use crate::scanner::*;
```

### `scanner.cc`

Manually converted

## Verification

The corpus tests are run via `rust-tester`. It is just the code from the `tree-sitter test` command ([source](https://github.com/tree-sitter/tree-sitter/blob/master/cli/src/test.rs)] bound directly to the rust library.
