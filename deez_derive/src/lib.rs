mod macro_rules;

use attribute_derive::Attribute;
use macro_rules::{compose_key, insert_gsi, insert_index};
use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::{collections::HashMap, fmt::Debug};
use syn::{DeriveInput, Field, Ident};

#[proc_macro_derive(Deez, attributes(ligma_schema, ligma_attribute, ligma_ignore))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, data, ident, .. } = syn::parse(input).unwrap();

    let s = DeezSchema::from_attributes(&attrs).unwrap();

    let mut m = HashMap::new();
    insert_index!(m, "primary".to_string(), s.hash, s.range, format_ident!("Primary"));
    insert_gsi!(m, s.gsi1, s.gsi1_hash, s.gsi1_range, format_ident!("Gsi1"));
    insert_gsi!(m, s.gsi2, s.gsi2_hash, s.gsi2_range, format_ident!("Gsi2"));
    insert_gsi!(m, s.gsi3, s.gsi3_hash, s.gsi3_range, format_ident!("Gsi3"));
    insert_gsi!(m, s.gsi4, s.gsi4_hash, s.gsi4_range, format_ident!("Gsi4"));
    insert_gsi!(m, s.gsi5, s.gsi5_hash, s.gsi5_range, format_ident!("Gsi5"));
    insert_gsi!(m, s.gsi6, s.gsi6_hash, s.gsi6_range, format_ident!("Gsi6"));
    insert_gsi!(m, s.gsi7, s.gsi7_hash, s.gsi7_range, format_ident!("Gsi7"));
    insert_gsi!(m, s.gsi8, s.gsi8_hash, s.gsi8_range, format_ident!("Gsi8"));
    insert_gsi!(m, s.gsi9, s.gsi9_hash, s.gsi9_range, format_ident!("Gsi9"));
    insert_gsi!(m, s.gsi10, s.gsi10_hash, s.gsi10_range, format_ident!("Gsi10"));
    insert_gsi!(m, s.gsi11, s.gsi11_hash, s.gsi11_range, format_ident!("Gsi11"));
    insert_gsi!(m, s.gsi12, s.gsi12_hash, s.gsi12_range, format_ident!("Gsi12"));
    insert_gsi!(m, s.gsi13, s.gsi13_hash, s.gsi13_range, format_ident!("Gsi13"));
    insert_gsi!(m, s.gsi14, s.gsi14_hash, s.gsi14_range, format_ident!("Gsi14"));
    insert_gsi!(m, s.gsi15, s.gsi15_hash, s.gsi15_range, format_ident!("Gsi15"));
    insert_gsi!(m, s.gsi16, s.gsi16_hash, s.gsi16_range, format_ident!("Gsi16"));
    insert_gsi!(m, s.gsi17, s.gsi17_hash, s.gsi17_range, format_ident!("Gsi17"));
    insert_gsi!(m, s.gsi18, s.gsi18_hash, s.gsi18_range, format_ident!("Gsi18"));
    insert_gsi!(m, s.gsi19, s.gsi19_hash, s.gsi19_range, format_ident!("Gsi19"));
    insert_gsi!(m, s.gsi20, s.gsi20_hash, s.gsi20_range, format_ident!("Gsi20"));

    let struct_data = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("could not parse struct"),
    };

    let mut field_av_map = quote! {};

    for field in struct_data.fields.iter() {
        if field.attrs.len() > 0 {
            if let Ok(attribute) = DeezIgnore::from_attributes(&field.attrs) {
                if attribute.ignore {
                    continue;
                }
            }

            if let Ok(attribute) = DeezAttribute::from_attributes(&field.attrs) {
                let composite = Composite {
                    position: attribute.position,
                    syn_field: field.clone(),
                };

                if let Some(index) = m.get_mut(&attribute.index) {
                    match attribute.key.as_str() {
                        "hash" => index.hash.composite.push(composite),
                        "range" => index.range.composite.push(composite),
                        _ => panic!("key must be either `hash` or `range`"),
                    }
                } else {
                    panic!("unknown index: {}", attribute.index);
                }
            }
        }

        let type_name = match &field.ty {
            syn::Type::Path(p) => p.to_token_stream().to_string(),
            _ => panic!("could not parse field type as path"),
        };

        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        match type_name.as_str() {
            "String" => {
                field_av_map = quote! {
                    #field_av_map
                    m.insert(#field_name.to_string(), AttributeValue::S(item.#field_ident.clone()));
                }
            }
            "f64" => {
                field_av_map = quote! {
                    #field_av_map
                    m.insert(#field_name.to_string(), AttributeValue::N(item.#field_ident.to_string()));
                }
            }
            "bool" => {
                field_av_map = quote! {
                    #field_av_map
                    m.insert(#field_name.to_string(), AttributeValue::Bool(item.#field_ident));
                }
            }
            _ => panic!("unsupported type: {}", type_name),
        }
    }

    let mut index_key_match = quote! {};
    let mut index_keys_match = quote! {};
    let mut index_av_map = quote! {};

    for (_, v) in m.iter() {
        let hash_composite = compose_key!(v.hash);
        let range_composite = compose_key!(v.range);

        let index = v.index.clone();
        let service = s.service.clone();
        let entity = s.entity.clone();
        let hash_field = v.hash.field.clone();
        let range_field = v.range.field.clone();

        index_key_match = quote! {
            #index_key_match
            Index::#index => {
                let mut composed = String::new();
                match key {
                    Key::Hash => {
                        composed.push_str(&format!("${}#{}", #service, #entity));
                        #hash_composite
                        return IndexKey {
                            field: #hash_field.to_string(),
                            composite: composed,
                        }
                    }
                    Key::Range => {
                        composed.push_str(&format!("${}", #entity));
                        #range_composite
                        return IndexKey {
                            field: #range_field.to_string(),
                            composite: composed,
                        }
                    }
                }
            }
        };

        index_keys_match = quote! {
            #index_keys_match
            Index::#index => {
                return IndexKeys {
                    hash: self.index_key(Index::#index, Key::Hash),
                    range: self.index_key(Index::#index, Key::Range),
                }
            }
        };

        index_av_map = quote! {
            #index_av_map
            {
                let keys = item.index_keys(Index::#index);
                m.insert(keys.hash.field, AttributeValue::S(keys.hash.composite));
                m.insert(keys.range.field, AttributeValue::S(keys.range.composite));
            }
        };
    }

    let uses = quote! {
        use deez::{Index, Key, IndexKey, IndexKeys};
        use aws_sdk_dynamodb::types::AttributeValue;
    };

    let impl_self = quote! {
        impl #ident {
            pub fn index_key(&self, index: Index, key: Key) -> IndexKey {
                match index {
                    #index_key_match
                    _ => panic!("unknown entity index: {}", index), // todo: sus?
                }
            }
            pub fn index_keys(&self, index: Index) -> IndexKeys {
                match index {
                    #index_keys_match
                    _ => panic!("unknown entity index: {}", index),
                }
            }
        }
    };

    let impl_from = quote! {
        impl From<#ident> for HashMap<String, AttributeValue> {
            fn from(item: #ident) -> HashMap<String, AttributeValue> {
                let mut m: HashMap<String, AttributeValue> = HashMap::new();
                #field_av_map
                #index_av_map
                m
            }
        }
    };

    let o = quote! {
        #uses
        #impl_self
        #impl_from
    };

    o.into()
}

////////////////////////////////////////////////////////////////////////////////
///
struct IndexKeys {
    index: Ident,
    hash: IndexKey,
    range: IndexKey,
}

#[derive(Default)]
struct IndexKey {
    field: String,
    composite: Vec<Composite>,
}

struct Composite {
    position: usize,
    syn_field: Field,
}

////////////////////////////////////////////////////////////////////////////////
/// attributes
#[derive(Attribute, Debug)]
#[attribute(ident = ligma_attribute)]
struct DeezAttribute {
    index: String,
    key: String,
    #[attribute(default = 0)]
    position: usize,
}

// todo: cant use empty struct???
#[derive(Attribute, Debug)]
#[attribute(ident = ligma_ignore)]
struct DeezIgnore {
    #[attribute(optional = false, default = true)]
    ignore: bool,
}

#[derive(Attribute, Debug)]
#[attribute(ident = ligma_schema)]
// #[attribute(invalid_field = "ok")]
struct DeezSchema {
    service: String,
    table: String,
    entity: String,

    hash: String,
    range: String,

    gsi1: Option<String>,
    gsi1_hash: Option<String>,
    gsi1_range: Option<String>,

    gsi2: Option<String>,
    gsi2_hash: Option<String>,
    gsi2_range: Option<String>,

    gsi3: Option<String>,
    gsi3_hash: Option<String>,
    gsi3_range: Option<String>,

    gsi4: Option<String>,
    gsi4_hash: Option<String>,
    gsi4_range: Option<String>,

    gsi5: Option<String>,
    gsi5_hash: Option<String>,
    gsi5_range: Option<String>,

    gsi6: Option<String>,
    gsi6_hash: Option<String>,
    gsi6_range: Option<String>,

    gsi7: Option<String>,
    gsi7_hash: Option<String>,
    gsi7_range: Option<String>,

    gsi8: Option<String>,
    gsi8_hash: Option<String>,
    gsi8_range: Option<String>,

    gsi9: Option<String>,
    gsi9_hash: Option<String>,
    gsi9_range: Option<String>,

    gsi10: Option<String>,
    gsi10_hash: Option<String>,
    gsi10_range: Option<String>,

    gsi11: Option<String>,
    gsi11_hash: Option<String>,
    gsi11_range: Option<String>,

    gsi12: Option<String>,
    gsi12_hash: Option<String>,
    gsi12_range: Option<String>,

    gsi13: Option<String>,
    gsi13_hash: Option<String>,
    gsi13_range: Option<String>,

    gsi14: Option<String>,
    gsi14_hash: Option<String>,
    gsi14_range: Option<String>,

    gsi15: Option<String>,
    gsi15_hash: Option<String>,
    gsi15_range: Option<String>,

    gsi16: Option<String>,
    gsi16_hash: Option<String>,
    gsi16_range: Option<String>,

    gsi17: Option<String>,
    gsi17_hash: Option<String>,
    gsi17_range: Option<String>,

    gsi18: Option<String>,
    gsi18_hash: Option<String>,
    gsi18_range: Option<String>,

    gsi19: Option<String>,
    gsi19_hash: Option<String>,
    gsi19_range: Option<String>,

    gsi20: Option<String>,
    gsi20_hash: Option<String>,
    gsi20_range: Option<String>,
}
