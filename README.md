# Programming Bitcoin in Rust

This repository adapts the code from [Programming Bitcoin](https://learning.oreilly.com/library/view/programming-bitcoin/9781492031482/) by [Jimmy Song](https://github.com/jimmysong) to Rust.

## Current Status

Chapter 1, _Finite Fields_, has been converted to Rust. Its narrative is in `ch01.asciidoc`, and its runnable implementation is in `code-ch01`.

The remaining chapters retain their original source material until they are converted.

## Prerequisites

Install a current stable Rust toolchain with [rustup](https://rustup.rs/), then verify the installation:

```shell
rustc --version
cargo --version
```

## Chapter 1

Run the Chapter 1 test suite from the repository root:

```shell
cargo test --manifest-path code-ch01/Cargo.toml
```

Run the standard quality checks:

```shell
cargo fmt --manifest-path code-ch01/Cargo.toml --check
cargo clippy --manifest-path code-ch01/Cargo.toml --all-targets -- -D warnings
```

## Layout

`code-ch01` mirrors the Chapter 1 source layout in Rust:

- `ecc.rs` implements finite-field elements and arithmetic.
- `exercises.rs` contains runnable counterparts to the Chapter 1 notebook exercises.
- `answers.rs` contains the exercise prompts and source-tagged solutions.
- `examples.rs` contains runnable chapter examples.
- `helper.rs` contains small test helpers.
- `lib.rs` exposes the Chapter 1 modules.

## License

See [LICENSE](LICENSE) for this repository's license terms.