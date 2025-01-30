use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

static mut FUNCS: Vec<&'static str> = Vec::new();

#[proc_macro_attribute]
pub fn auto_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as ItemFn);
    let fn_name = item_ast.sig.ident.to_string();

    unsafe {
        FUNCS.push(Box::new(fn_name).leak());
    }

    quote! {
        #item_ast
    }
    .into()
}

#[proc_macro]
pub fn expand_funcs(_item: TokenStream) -> TokenStream {
    let funcs_call: Vec<_> = unsafe {
        FUNCS
            .iter()
            .map(|&fn_name| {
                let fn_ident = syn::Ident::new(fn_name, proc_macro2::Span::call_site());
                quote! {
                    #fn_ident();
                }
            })
            .collect()
    };

    quote! {
        #(#funcs_call)*
    }
    .into()
}
