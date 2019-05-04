extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let identifier = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #identifier {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#identifier));
            }
        }
    };
    gen.into()
}