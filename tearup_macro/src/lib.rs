use proc_macro::TokenStream;
use syn::{parse_quote, Ident, ItemFn};

mod asyncc;
mod sync;

#[proc_macro_attribute]
pub fn tearup(attr: TokenStream, input: TokenStream) -> TokenStream {
    tearup_body(attr, input, false)
}

#[proc_macro_attribute]
pub fn tearup_test(attr: TokenStream, input: TokenStream) -> TokenStream {
    tearup_body(attr, input, true)
}

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
