mod derive_with_docs;
mod konfig;

use proc_macro::TokenStream;
use syn::{DeriveInput, LitStr};

#[proc_macro]
pub fn konfig(arg: TokenStream) -> TokenStream {
    let src = syn::parse_macro_input!(arg as LitStr);

    self::konfig::expand(src).into()
}

#[proc_macro_derive(WithDocs)]
pub fn derive_with_docs(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    self::derive_with_docs::ImplCodegen::expand(input).into()
}
