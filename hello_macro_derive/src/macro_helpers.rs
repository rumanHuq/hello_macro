
use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
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

pub fn expand_summable_fields(ast: &syn::DeriveInput) -> TokenStream {
  let identifier = &ast.ident;
  let body = &ast.data;
  let size = match body {
    syn::Data::Struct(ref item) => item.fields.iter().len(),
    _ => panic!("Only Struct is allowed"),
  };
  let gen = quote! {
      impl CountStruct for #identifier {
          fn count_struct()->usize {
              #size
          }
      }
  };
  gen.into()
}