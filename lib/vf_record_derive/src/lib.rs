extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse,
    DeriveInput,
    Data,
};

fn impl_vf_record_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(ref data) => { &data.fields }
        _ => { panic!("VFRecord can only be implemented against a struct") }
    };

    let gen = quote! {
        struct #name {
            id: Option<Address>,
            note: Option<String>,
            #(#fields),*
        }
    };

    gen.into()
}

#[proc_macro_derive(VFRecord)]
pub fn vf_record_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse(input).unwrap();

    // Build the trait implementation
    impl_vf_record_macro(&ast)
}
