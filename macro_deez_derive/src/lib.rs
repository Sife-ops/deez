use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Sugon)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        syn::Data::Struct(st) => st.fields,
        _ => panic!("todo"),
    };

    let mut inserts = quote! {};
    for field in &fields {
        let ident = match field.ident.as_ref() {
            Some(ident) => ident,
            _ => continue,
        };
        // todo: rename keys
        let field_name = ident.to_string();
        let field_type = match &field.ty {
            syn::Type::Path(tp) => tp.clone().into_token_stream().to_string(),
            _ => continue,
        };
        match field_type.as_ref() {
            "String" => {
                inserts = quote! {
                    #inserts
                    m.insert(#field_name.to_string(), AttributeValue::S(self.#ident.to_string()));
                };
            }
            "bool" => {
                inserts = quote! {
                    #inserts
                    m.insert(#field_name.to_string(), AttributeValue::Bool(self.#ident));
                };
            }
            "usize" | "isize" => {
                inserts = quote! {
                    #inserts
                    m.insert(#field_name.to_string(), AttributeValue::N(self.#ident.to_string()));
                };
            }
            // todo: all other types
            &_ => {}
        }
    }

    let name = &ast.ident;
    let (impl_gen, type_gen, where_clause) = ast.generics.split_for_impl();

    let output = quote! {
        impl #impl_gen DeezMaps for #name #type_gen #where_clause {
            fn to_av_map(&self) -> HashMap<String, AttributeValue> {
                let mut m = HashMap::new();
                #inserts
                m
            }
        }
    };
    output.into()
}
