/// The convention for structuring crates and macro crates is as follows: 
/// for a crate named 'foo', a custom derive procedural macro crate is called 'foo_derive'. 

use proc_macro::TokenStream;
use quote::quote;

//The hello_macro_derive function will be called when a user of our library specifies #[derive(HelloMacro)] on a type.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // abstracting the macro login in to impl_hello_macro
    // this makes writing a procedural macro more convenient. 
    // we can reuse this pattern in almost every procedural macro
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let temp_code = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    temp_code.into()
}