
mod macro_helpers;
extern crate proc_macro;

use macro_helpers::{expand_summable_fields, impl_hello_macro};
use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_hello_macro(&ast)
}
#[proc_macro_derive(CountStruct)]
pub fn count_struct_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).expect("failed to parse input into `syn::Item`");
  expand_summable_fields(&ast)
}