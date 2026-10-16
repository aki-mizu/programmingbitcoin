# Project Guidelines

## Chapter Conversions

When converting a book chapter from Python to Rust:

- Work in the corresponding `code-chNN` directory and create a self-contained Rust crate with `Cargo.toml` and `lib.rs`.
- Mirror the chapter's source roles with Rust modules such as `ecc.rs`, `answers.rs`, `examples.rs`, and `helper.rs` when those counterparts exist.
- Preserve the mathematical and narrative portions of `chNN.asciidoc`; change only language-specific headings, explanations, code blocks, include paths, and exercise API names.
- Keep the AsciiDoc include tags synchronized with the Rust source tags used by the chapter.
- Once a Rust counterpart exists and the chapter no longer references it, remove the superseded Python module. Remove Python-only notebooks and Jupyter metadata for that chapter as well.
- Update the root README when the supported language, setup workflow, converted-chapter status, or validation command changes.
- Do not alter unconverted chapters or their Python tooling unless the task explicitly requires it.

## Rust Validation

For a converted chapter, run these commands from the repository root:

```shell
cargo fmt --manifest-path code-chNN/Cargo.toml --check
cargo clippy --manifest-path code-chNN/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path code-chNN/Cargo.toml
```

Before finishing, verify that `chNN.asciidoc` has no stale Python code paths, Python source fences, or Python-specific API references for the converted chapter.