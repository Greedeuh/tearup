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

/// ```
/// use async_trait::async_trait;
/// use tearup::{tearup_test, AsyncWaitingContext, FromAsyncContext, ReadyFn};
///
/// // First define your context
/// struct YourContext {
///     something_you_need_in_test: SomethingYouSetup,
/// }
///
/// // Second implement your setup/teardown
/// #[async_trait]
/// impl<'a> AsyncWaitingContext<'a> for YourContext {
///     async fn setup(ready: ReadyFn) -> Self {
///         /* do your stuff... */
///         ready(); // notify that your setup id ready
///         Self { something_you_need_in_test: SomethingYouSetup{} }
///     }
///
///     async fn teardown(&mut self, ready: ReadyFn) {
///         /* do your stuff... */
///         ready(); // notify that your setup id ready
///     }
/// }
///
/// // Optionnaly define some setup accessor
/// // if you need to access something from your setup (like db connection, seed, etc)
/// #[derive(Clone)]
/// pub struct SomethingYouSetup;
/// #[async_trait]
/// impl FromAsyncContext<'_, YourContext> for SomethingYouSetup {
///     async fn from_context(context: &YourContext) -> Self {
///         context.something_you_need_in_test.clone()
///     }
/// }
///
/// // And write your tests !
/// #[tearup_test(YourContext)]
/// async fn is_should_do_that(mut something_you_need_in_test: SomethingYouSetup) {
///     // assert something using something_you_need_in_test
/// }
/// ```
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
