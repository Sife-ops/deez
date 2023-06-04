use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Sugon)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        syn::Data::Struct(st) => st.fields,
        _ => panic!("todo"),
    };

    let idents = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    let keys = idents
        .clone()
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    let name = &ast.ident;
    let (impl_gen, type_gen, where_clause) = ast.generics.split_for_impl();

    // let a = HashMap::new()

    let output = quote! {
        impl #impl_gen MyTrait for #name #type_gen #where_clause {
            fn to_avmap(&self) -> HashMap<String, String> {
                let mut m = HashMap::new();
                #(
                    m.insert(#keys.to_string(), self.#idents.to_string());
                )*
                m
            }
        }
    };
    output.into()
}
