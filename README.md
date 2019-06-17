# rust-safe

[![Build Status](https://travis-ci.org/Centril/rust-safe.svg?branch=master)](https://travis-ci.org/Centril/rust-safe)
[![Crates.io](https://img.shields.io/crates/v/mdbook.svg)](https://crates.io/crates/safe)
[![Docs.rs](https://docs.rs/safe/badge.svg)](https://docs.rs/safe/badge.svg)

A `#[safe]` attribute for explaining why `unsafe { ... }` is OK.

## Getting Started

This crate is mainly meant as a way to document your `unsafe` code. The simplest
usage is to use a `#[safe(reason = "...")]` attribute:

```rust,skt-main
#[safe(reason = "All zeroes is a valid bit pattern for a `u8` array")]
unsafe {
  let buffer: [u8; 32] = std::mem::zeroed();
}
```

You can also provide pre- and post-conditions with the `requires` and `ensures`
arguments.

```rust,skt-main
const HELLO_WORLD: &[u8] = b"Hello, World!\0";

let mut buffer: *mut c_char = std::ptr::null_mut();

#[safe(reason = "This is a valid way to initialize a C-style string",
        requires = "buffer.is_null()",
        ensures = "libc::strlen(buffer) == HELLO_WORLD.len()-1")]
unsafe {
  buffer = libc::malloc(42) as *mut c_char;

  libc::strcpy(buffer, HELLO_WORLD.as_ptr() as *const c_char);
}
```

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
