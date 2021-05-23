use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
/// Example taken from https://doc.rust-lang.org/reference/procedural-macros.html
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    // prints here would appear during the compile
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_derive(CustomDebug)]
pub fn custom_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        use std::fmt;
        impl fmt::Debug for #name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", stringify!(#name))
            }
        }
    };

    TokenStream::from(expanded)
}
