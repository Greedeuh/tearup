use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Semi, Attribute, FnArg, Ident, Stmt};

pub fn body(
    context: Ident,
    sig: syn::Signature,
    attrs: Vec<Attribute>,
    stmts: &Vec<Stmt>,
) -> TokenStream {
    let name = sig.ident.clone();
    let let_args = define_args(&sig);

    let result = quote! {

        #(#attrs)* async fn #name() {
            use tearup::{AsyncContext, AsyncSharedContext, FutureExt};

            let mut shared_context = AsyncSharedContext::default();
            let mut context = #context::launch_setup(shared_context.clone()).await;

            #let_args

            let text_execution = context.launch_test(move || {
                async move {
                    #(#stmts)*
                }.boxed()
            }).await;

            context.launch_teardown(shared_context).await;

            if let Err(err) = text_execution {
                std::panic::resume_unwind(err)
            }
        }

    };
    result.into()
}

fn define_args(sig: &syn::Signature) -> Punctuated<proc_macro2::TokenStream, Semi> {
    sig.inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(arg) => define_arg(arg),
            _ => panic!("You should not pass this 'self' args"),
        })
        .collect::<Punctuated<proc_macro2::TokenStream, Semi>>()
}

fn define_arg(arg: &syn::PatType) -> proc_macro2::TokenStream {
    let name = &arg.pat;
    let ty = &arg.ty;
    quote! {
        let #name: #ty = shared_context.get().await.unwrap();
    }
    .to_token_stream()
}
