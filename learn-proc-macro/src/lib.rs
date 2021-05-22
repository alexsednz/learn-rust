use proc_macro::TokenStream;

#[proc_macro_attribute]
/// Example taken from https://doc.rust-lang.org/reference/procedural-macros.html
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    // prints here would appear during the compile
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}
