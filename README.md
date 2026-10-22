# Programming Bitcoin in Rust

This repository adapts the code from [Programming Bitcoin](https://learning.oreilly.com/library/view/programming-bitcoin/9781492031482/) by [Jimmy Song](https://github.com/jimmysong) to Rust.

## Current Status

Chapters 1 through 3 have been converted to Rust. Their narratives are in `ch01.asciidoc`, `ch02.asciidoc`, and `ch03.asciidoc`, and their runnable implementations are in `code-ch01`, `code-ch02`, and `code-ch03`.

Chapter 3 depends on Chapter 1's arbitrary-precision finite-field implementation so the shared arithmetic is implemented once and can represent secp256k1's 256-bit values.

The remaining chapters retain their original source material until they are converted.

## Prerequisites

Install a current stable Rust toolchain with [rustup](https://rustup.rs/), then verify the installation:

```shell
rustc --version
cargo --version
```

## Converted Chapters

### Chapter 1

Run the Chapter 1 test suite from the repository root:

```shell
cargo test --manifest-path code-ch01/Cargo.toml
```

### Chapter 2

Run the Chapter 2 test suite from the repository root:

```shell
cargo test --manifest-path code-ch02/Cargo.toml
```

### Chapter 3

Run the Chapter 3 test suite from the repository root:

```shell
cargo test --manifest-path code-ch03/Cargo.toml
```

Run the standard quality checks for any converted chapter:

```shell
cargo fmt --manifest-path code-chNN/Cargo.toml --check
cargo clippy --manifest-path code-chNN/Cargo.toml --all-targets -- -D warnings
```

## Layout

Each converted `code-chNN` directory is a Rust crate. The source layout includes:

- `ecc.rs` implements the chapter's mathematical types and operations. Chapter 3 reuses Chapter 1's `FieldElement` through a local path dependency.
- `exercises.rs` contains runnable counterparts to the chapter notebook exercises.
- `answers.rs` contains the exercise prompts and source-tagged solutions.
- `examples.rs` contains runnable chapter examples.
- `helper.rs` contains small test helpers.
- `lib.rs` exposes the chapter modules.

## License

See [LICENSE](LICENSE) for this repository's license terms.