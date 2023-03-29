use proc_macro::TokenStream;
use syn::{parse_quote, Ident, ItemFn};

#[cfg(feature = "async")]
mod asyncc;
mod sync;

/// Same as `tearup_test` but does not add turn you function into a `#[test]`
#[proc_macro_attribute]
pub fn tearup(attr: TokenStream, input: TokenStream) -> TokenStream {
    tearup_body(attr, input, false)
}

#[proc_macro_attribute]
pub fn tearup_test(attr: TokenStream, input: TokenStream) -> TokenStream {
    tearup_body(attr, input, true)
}

#[cfg(not(feature = "async"))]
fn tearup_body(attr: TokenStream, input: TokenStream, test: bool) -> TokenStream {
    let context = syn::parse_macro_input!(attr as Ident);
    let input = syn::parse_macro_input!(input as ItemFn);

    let ItemFn {
        mut attrs,
        sig,
        block,
        ..
    } = input;
    let stmts = &block.stmts;

    if sig.asyncness.is_some() {
        panic!("You need to turn on the 'async' feature on tearup to use it on async fn.")
    }

    if test {
        attrs.push(parse_quote!(#[test]));
    }
    sync::body(context, sig, attrs, stmts)
}

#[cfg(feature = "async")]
fn tearup_body(attr: TokenStream, input: TokenStream, test: bool) -> TokenStream {
    let context = syn::parse_macro_input!(attr as Ident);
    let input = syn::parse_macro_input!(input as ItemFn);

    let ItemFn {
        mut attrs,
        sig,
        block,
        ..
    } = input;
    let stmts = &block.stmts;

    if sig.asyncness.is_some() {
        if test {
            attrs.push(parse_quote!(#[tokio::test]));
        }
        asyncc::body(context, sig, attrs, stmts)
    } else {
        if test {
            attrs.push(parse_quote!(#[test]));
        }
        sync::body(context, sig, attrs, stmts)
    }
}
