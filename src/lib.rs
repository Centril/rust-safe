extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Expr, ExprUnsafe, LitStr};

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
