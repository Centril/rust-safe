//! A custom attribute for declaring why an `unsafe` block is correct.
//!
//! # Examples
//!
//! The most basic way this attribute can be used is by annotating an `unsafe`
//! block and providing a `reason` for why it is valid.
//!
//! ```rust
//! # #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! # use safe::safe;
//! let mut x: u32 = 42;
//! let x_ptr = &mut x as *mut u32;
//!
//! #[safe(reason = "This is the only reference to x")]
//! unsafe {
//!     x_ptr.write(7);
//! }
//! ```
//!
//! Unsafe code often has constraints that must be valid before or after the
//! code is run.
//!
//! You can provide an assertion which will be executed with `debug_assert!()`
//! immediately before the block executes by adding a `requires = "..."` to
//! the attribute.
//!
//! ```rust
//! # #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! # use safe::safe;
//! let x: u32 = 42;
//! let x_ptr = &x as *const u32;
//!
//! #[safe(
//!     reason = "The pointer points to a valid integer",
//!     requires = "!x_ptr.is_null()"
//! )]
//! unsafe {
//!     assert_eq!(42, x_ptr.read());
//! }
//! ```
//!
//! Likewise, for conditions which must be upheld after the `unsafe` block, you
//! can provide an `ensures` assertion.
//!
//! ```rust
//! # #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! # use safe::safe;
//! let mut allocated: *mut u32;
//!
//! #[safe(
//!     reason = "Malloc will always return a non-null pointer unless there is an out-of-memory error",
//!     ensures = "!allocated.is_null()"
//! )]
//! unsafe {
//!     allocated = libc::malloc(std::mem::size_of::<u32>()) as *mut u32;
//! }
//! ```

extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Expr, ExprUnsafe, LitStr};

/// A custom attribute for explaining why an `unsafe` block is valid and any
/// invariants which must be upheld.
///
/// See crate-level docs for example usage.
#[proc_macro_attribute]
pub fn safe(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ExprUnsafe);

    let args = match Args::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let requires = match args.requires {
        Some(ref assertion) => {
            let assertion: TokenStream = assertion.value().parse().unwrap();
            let assertion = parse_macro_input!(assertion as Expr);
            let reason = &args.reason;
            quote! {
                debug_assert!(#assertion, "\"{}\" is invalid because the required condition failed", #reason);
            }
        }
        None => quote! {},
    };

    let ensures = match args.ensures {
        Some(ref assertion) => {
            let assertion: TokenStream = assertion.value().parse().unwrap();
            let assertion = parse_macro_input!(assertion as Expr);
            let reason = &args.reason;
            quote! {
                debug_assert!(#assertion, "\"{}\" is invalid because the ensured condition failed", #reason);
            }
        }
        None => quote! {},
    };

    let block = item.block;

    let output = quote! {
        unsafe {
            #requires

            let got = #block;

            #ensures

            got
        }
    };

    output.into()
}

#[derive(Debug, Clone, FromMeta)]
struct Args {
    /// A message explaining why this `unsafe` block is valid.
    reason: String,
    /// An assertion which will be run before entering the block in debug
    /// builds.
    #[darling(default)]
    requires: Option<LitStr>,
    /// An assertion which will be run after entering the unsafe block in debug
    /// builds.
    #[darling(default)]
    ensures: Option<LitStr>,
}
