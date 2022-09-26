use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;

mod internal;

use internal::expand_derive_struct_mapping;

#[proc_macro_derive(StructMapping, attributes(struct_mapping))]
pub fn derive_struct_mapping(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    expand_derive_struct_mapping(&mut input).unwrap().into()
}

// TODO: i would like something like serde
// fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
//     let compile_errors = errors.iter().map(syn::Error::to_compile_error);
//     quote!(#(#compile_errors)*)
// }
