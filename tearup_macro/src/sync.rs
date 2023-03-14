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
    let let_args = define_args(&sig, &context);

    let result = quote! {

        #(#attrs)* fn #name() {
            use tearup::Context;

            let mut context = #context::launch_setup();

            #let_args

            let text_execution = context.launch_test(move || {
                #(#stmts)*
            });

            context.launch_teardown();

            if let Err(err) = text_execution {
                std::panic::resume_unwind(err)
            }
        }

    };
    result.into()
}

fn define_args(
    sig: &syn::Signature,
    context: &Ident,
) -> Punctuated<proc_macro2::TokenStream, Semi> {
    sig.inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(arg) => define_arg(arg, context),
            _ => panic!("You should not pass this 'self' args"),
        })
        .collect::<Punctuated<proc_macro2::TokenStream, Semi>>()
}

fn define_arg(arg: &syn::PatType, context: &Ident) -> proc_macro2::TokenStream {
    let name = &arg.pat;
    let ty = &arg.ty;
    quote! {
        let #name = <#ty as tearup::FromContext<#context>>::from_context(&context);
    }
    .to_token_stream()
}
