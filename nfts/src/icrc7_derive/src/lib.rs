use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

mod icrc37;
mod icrc7;

#[proc_macro_derive(Icrc7, attributes(icrc7))]
pub fn derive_icrc7(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    icrc7::expand_derive_icrc7(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Icrc37, attributes(icrc37))]
pub fn derive_icrc37(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    icrc37::expand_derive_icrc37(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
