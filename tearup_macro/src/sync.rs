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
    let args = define_args(&sig, &context);

    let result = quote! {

        #(#attrs)* fn #name() {
            let ready_flag = std::sync::Arc::new(std::sync::Mutex::new(false));

            let ready_flag_given = ready_flag.clone();
            let ready = Box::new(move || {
                let mut ready = ready_flag_given.lock().unwrap();
                *ready = true;
            });

            let mut context = #context::setup(ready);

            #args

            context.test(move || {
                #(#stmts)*
            }, ready_flag);
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
        let #name = <#ty as tearup::FromContext<#context>>::from_setup(&context);
    }
    .to_token_stream()
}
