use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_icrc37(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let impl_block = quote! {};
    Ok(impl_block)
}
