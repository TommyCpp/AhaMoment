use proc_macro::TokenStream;

use syn::{braced, parse_macro_input, parse_quote, token, Field, Ident, Result, Token, ItemFn, ItemType, FnArg, PatType, ReturnType, Block};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use quote::quote;
use syn::token::Token;
use syn::export::{ToTokens, Span};

#[proc_macro_attribute]
pub fn auto_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let original_func = function.clone();
    // Check number of input, if
    // 1. There is no input
    // 2. The only input is self or &self.
    //
    // Throw a compile error.
    if function.sig.inputs.len() < 1 ||
        function.sig.inputs.len() == 1 &&
            function.sig.receiver().is_some() {
        return throw_compile_error("Should have more than one input");
    }

    let mut input_pats = Vec::with_capacity(function.sig.inputs.len());
    // process the input
    let mut receiver = None;
    for input in function.sig.inputs.iter_mut() {
        match input {
            FnArg::Typed(PatType { ty, pat, .. }) => {
                input_pats.push(pat.clone());
                *input = parse_quote! {#pat: Vec<#ty>};
            }
            FnArg::Receiver(_) => {
                receiver = Some(input.clone());
            } // for &self and self, do nothing
        }
    };

    //process the output
    function.sig.output = {
        match function.sig.output {
            ReturnType::Type(_, ty) => {
                parse_quote! {
                  -> Vec<#ty>
                }
            }
            ReturnType::Default => {
                // no return type, throw a error
                return throw_compile_error("Should have return value");
            }
        }
    };

    //process the ident
    let ident = function.sig.ident.clone();
    function.sig.ident = Ident::new(format!("{}_vec", function.sig.ident).as_str(),
                                    function.sig.ident.span());


    //process the body
    //Get all input
    let pat = input_pats[0].clone();
    // Convert into TokenStream
    let args: Vec<proc_macro2::TokenStream> = input_pats.iter()
        .map(|pat| quote! {#pat})
        .collect();
    // If we have receiver as first input, then use self.
    let receiver_token = if let Some(token) = receiver {
        quote! {self.}
    } else {
        quote! {}
    };
    function.block = Box::new(parse_quote!({
            let len = #pat.len();
            #(assert_eq!(#args.len(), len);)*
            let mut res = Vec::with_capacity(len);
            for _idx in 0..#pat.len(){
                res.push(#receiver_token#ident(#(#args[_idx].clone(),)*));
            }
            res
        }));


    // let sig = function.sig.clone();
    return quote! {
        #function
        #original_func
    }.into();
}


fn throw_compile_error(msg: &str) -> TokenStream {
    syn::Error::new(Span::call_site(), msg)
        .to_compile_error()
        .into()
}
