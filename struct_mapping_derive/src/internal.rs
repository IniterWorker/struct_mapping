use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, FieldsNamed};

struct StructuralContext<'a> {
    fields: Vec<StructuralFied<'a>>,
}

/// `StructuralFied` represents a field in the structure
struct StructuralFied<'a> {
    /// Reference to `syn::Ident`
    ident: &'a syn::Ident,
    is_skip: bool,
    is_skip_setter: bool,
    is_skip_getter: bool,
    names: Vec<String>,
}

#[derive(FromAttributes, Default)]
#[darling(attributes(struct_mapping))]
struct MyTrait {
    #[darling(default)]
    /// will skip the field
    skip: Option<bool>,

    #[darling(default)]
    /// will prevent getter/accessor implementation
    skip_getter: Option<bool>,

    #[darling(default)]
    /// will prevent setter/accessor implementation
    skip_setter: Option<bool>,

    /// will rename the field name
    #[darling(default)]
    rename: Option<String>,

    /// will provide an alias for the field
    #[darling(default)]
    alias: Option<String>,
}

fn parse_structural_context(
    input: &mut syn::DeriveInput,
) -> Result<StructuralContext, Vec<syn::Error>> {
    let DeriveInput { data, .. } = input;

    let fields = match data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents = named.iter();
                let idents = idents.fold(Vec::new(), |mut acc, elem| match &elem.ident {
                    Some(idt) => {
                        let parsed: MyTrait = FromAttributes::from_attributes(&elem.attrs).unwrap();
                        let field_name = idt.to_string();
                        let field = StructuralFied {
                            ident: idt,
                            is_skip: parsed.skip.unwrap_or(false),
                            is_skip_getter: parsed.skip_getter.unwrap_or(false),
                            is_skip_setter: parsed.skip_setter.unwrap_or(false),
                            names: vec![
                                match parsed.rename {
                                    None => Some(field_name),
                                    Some(renamed) => Some(renamed),
                                },
                                parsed.alias,
                            ]
                            .into_iter()
                            .filter_map(|i| i)
                            .collect::<Vec<String>>(),
                        };
                        acc.push(field);
                        acc
                    }
                    None => acc,
                });
                idents
            }
            _ => unimplemented!("is not implemented"),
        },
        _ => unimplemented!("is not implemented"),
    };

    Ok(StructuralContext { fields })
}

pub fn expand_derive_struct_mapping(
    input: &mut syn::DeriveInput,
) -> Result<TokenStream, Vec<syn::Error>> {
    let context = parse_structural_context(input)?;

    let setters = context
        .fields
        .iter()
        .filter(|elem| !elem.is_skip && !elem.is_skip_setter)
        .fold(Vec::new(), |mut acc, elem| {
            let ident = elem.ident;
            let names = &elem.names;

            acc.push(quote! {
                if vec![#(#names),*].contains(&key) {
                    self.#ident = ToStructMappingField::from_string(value)?;
                    return Ok(())
                }
            });
            acc
        });

    let getters = context
        .fields
        .iter()
        .filter(|elem| !elem.is_skip && !elem.is_skip_getter)
        .fold(Vec::new(), |mut acc, elem| {
            let ident = elem.ident;
            let names = &elem.names;

            acc.push(quote! {
                if vec![#(#names),*].contains(&key) {
                    return Ok(self.#ident.to_string())
                }
            });
            acc
        });

    let setters_tokens = quote! {
        #(#setters)*
    };

    let getters_tokens = quote! {
        #(#getters)*
    };

    let ident = &input.ident;
    let output = quote! {
        impl #ident {
            pub fn field_get(&self, key: &'static str) -> Result<String, struct_mapping::Error> {
                #getters_tokens
                Err(struct_mapping::Error::InvalidKey(key))
            }

            pub fn field_set(&mut self, key: &'static str, value: String) -> Result<(), struct_mapping::Error> {
                #setters_tokens
                Err(struct_mapping::Error::InvalidKey(key))
            }
        }
    };

    Ok(output.into())
}
