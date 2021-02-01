#![feature(log_syntax)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::{parse_quote, Error, parse_macro_input, DeriveInput, Data};
use syn::spanned::Spanned;

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);


    let recurse = match &input.data {
        Data::Struct(st) => {
            st.fields.iter().map(|field| -> Option<syn::Type> {
                match &field.ty {
                    syn::Type::BareFn(f) => {
                        match &f.output {
                            syn::ReturnType::Default => None,
                            syn::ReturnType::Type(_, ty) => {
                                Some((**ty).clone())
                            },
                        }
                    },
                    _ => panic!("only function is supported"),
                }
            })
            .filter(|t| t.is_some()) 
            .enumerate()
            .map(|(i, ty)| {
                let copy_ident = syn::Ident::new(&format!("_AssertCopy_{}", i), proc_macro2::Span::call_site());
                let sync_ident = syn::Ident::new(&format!("_AssertSync_{}", i), proc_macro2::Span::call_site());
                let ty = ty.unwrap();
                quote_spanned! {ty.span()=>
                    struct #copy_ident where #ty: std::marker::Copy;
                    struct #sync_ident where #ty: std::marker::Sync;
                }
            })
        }
        _ => panic!("only struct is supported"),
    };

    let expanded = quote! {
        #(#recurse)*
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn AssertReturnSync(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input: syn::ItemFn = syn::parse(item).unwrap();

    // // Generate assertions for function argument types.
    // let assertions: Vec<syn::ItemStruct> = input.sig.inputs
    //     .iter()
    //     .filter_map(|arg| {
    //         if let syn::FnArg::Typed(arg) = arg {
    //             if let syn::Type::Path(arg_type) = &*arg.ty {
    //                 let arg_type = 
    //                 let assertion_ident = format_ident!("_AssertCopyFor_{}", arg_type);
    //                 return Some(parse_quote! {
    //                     struct #assertion_ident where #arg_type: ::core::marker::Copy;
    //                 });
    //             }
    //         }
    //         None
    //     })
    //     .collect();

    // Generate an assertion for function return type, if there is one.

    let assertion = match &input.sig.output {
        syn::ReturnType::Type(_, return_type) => {
            match &**return_type {
                syn::Type::Path(return_type) => {
                    let return_type = &return_type.path.segments[0].ident;
                    let assertion_ident = format_ident!("_AssertSyncFor{}_", return_type);
                    quote!{
                        struct #assertion_ident where #return_type: ::core::marker::Sync;
                    }
                },
                _ => quote!{},
            }
        },
        _ => quote!{},
    };
    
    // Returns the original input and the generated assertions back to the compiler.
    TokenStream::from(quote! {
        #input

        #assertion
    })
}