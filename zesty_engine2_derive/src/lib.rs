use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};


#[proc_macro_derive(Matrix)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Matrix for #ident {}
    };

    output.into()
}
