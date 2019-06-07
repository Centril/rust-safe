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

    let precondition = match args.precondition {
        Some(ref assertion) => {
            let assertion: TokenStream = assertion.value().parse().unwrap();
            let assertion = parse_macro_input!(assertion as Expr);
            let reason = &args.reason;
            quote! {
                debug_assert!(#assertion, "\"{}\" is invalid because the precondition failed", #reason);
            }
        }
        None => quote! {},
    };

    let output = quote! {
        {
            #precondition

            #item
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
    precondition: Option<LitStr>,
}
