# RustUse Crate Template

Use this checklist when adding a new focused crate or expanding the `use-physics` facade.

## Target Layout

```text
crates/use-example/
  Cargo.toml
  README.md
  examples/
    basic_usage.rs
  src/
    lib.rs
    prelude.rs
  tests/
    basic_usage.rs
```

## Cargo.toml Pattern

```toml
[package]
name = "use-example"
description = "Composable example helpers for RustUse"
publish = false
version.workspace = true
edition.workspace = true
homepage.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
readme = "README.md"
documentation = "https://docs.rs/use-example"
keywords = ["id", "example"]
categories = ["data-structures"]

[lints]
workspace = true
```

Checklist:

- Keep package metadata inherited from the workspace wherever possible.
- Default new crates to `publish = false` until they are intentionally part of a release wave.
- Prefer lightweight dependencies and keep them crate-local when possible.

## src/lib.rs Pattern

```rust
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod prelude;
```

Checklist:

- Re-export the focused public API at the crate root.
- Keep APIs string-backed and explicit unless there is a clear ergonomic gain from stronger typing.

## README Structure

Keep crate README files short and consistent.

Required sections:

- title and one-line summary
- `Install`
- `Foundation`
- `When to use directly`
- `Scope`
- `Status`

## Testing Checklist

- Add unit tests for each public function or method that introduces logic.
- Add an integration test for the intended crate-level workflow.
- Add a small example under `examples/`.

## Validation Checklist

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```
