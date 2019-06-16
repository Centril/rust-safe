# rust-safe

[![Crates.io](https://img.shields.io/crates/v/mdbook.svg)](https://crates.io/crates/safe)
[![Docs.rs](https://docs.rs/safe/badge.svg)](https://docs.rs/safe/badge.svg)

A `#[safe]` attribute for explaining why `unsafe { ... }` is OK.

## Nightly Rust

Unfortunately, you'll need to be using `nightly` when this custom attribute is
applied to an expression. These feature flags are:

- `stmt_expr_attributes`
- `proc_macro_hygiene`

For more discussion, see [#3](https://github.com/Centril/rust-safe/issues/3).
