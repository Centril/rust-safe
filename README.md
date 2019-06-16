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

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
