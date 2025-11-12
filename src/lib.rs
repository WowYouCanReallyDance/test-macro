use std::sync::Mutex;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

static FUNCS: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

#[proc_macro_attribute]
pub fn auto_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as ItemFn);
    let fn_name = item_ast.sig.ident.to_string();

    FUNCS
        .lock()
        .map(|mut n| n.push(Box::new(fn_name).leak()))
        .map_err(|e| eprintln!("ERROR! There is something WRONG: {e}"))
        .ok();

    quote! {
        #item_ast
    }
    .into()
}

#[proc_macro]
pub fn expand_funcs(_item: TokenStream) -> TokenStream {
    let funcs_call: Vec<_> = FUNCS
        .lock()
        .map(|n| {
            n.iter()
                .map(|&fn_name| {
                    let fn_ident = syn::Ident::new(fn_name, proc_macro2::Span::call_site());
                    quote! {
                        #fn_ident();
                    }
                })
                .collect()
        })
        .map_err(|e| eprintln!("ERROR! There is something WRONG: {e}"))
        .unwrap_or_default();

    quote! {
        #(#funcs_call)*
    }
    .into()
}
