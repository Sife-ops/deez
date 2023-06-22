mod macro_rules;

use attribute_derive::Attribute;
use macro_rules::{attr_derive, compose_key, insert_gsi, insert_index, read_attr};
use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::{collections::HashMap, fmt::Debug};
use syn::{DeriveInput, Field};

attr_derive!();

struct IndexKeys {
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

#[proc_macro_derive(
    Deez,
    attributes(
        deez_schema,
        deez_ignore,
        deez_primary,
        deez_gsi1,
        deez_gsi2,
        deez_gsi3,
        deez_gsi4,
        deez_gsi5,
        deez_gsi6,
        deez_gsi7,
        deez_gsi8,
        deez_gsi9,
        deez_gsi10,
        deez_gsi11,
        deez_gsi12,
        deez_gsi13,
        deez_gsi14,
        deez_gsi15,
        deez_gsi16,
        deez_gsi17,
        deez_gsi18,
        deez_gsi19,
        deez_gsi20,
    )
)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, data, ident, .. } = syn::parse(input).unwrap();

    let s = DeezSchema::from_attributes(&attrs).unwrap();

    let mut index_name_match = quote! {};
    let mut index_meta = HashMap::new();

    insert_index!(index_meta, "Primary", s.primary_hash, s.primary_range);
    insert_gsi!(index_meta, index_name_match, "Gsi1", s.gsi1_name, s.gsi1_hash, s.gsi1_range);
    insert_gsi!(index_meta, index_name_match, "Gsi2", s.gsi2_name, s.gsi2_hash, s.gsi2_range);
    insert_gsi!(index_meta, index_name_match, "Gsi3", s.gsi3_name, s.gsi3_hash, s.gsi3_range);
    insert_gsi!(index_meta, index_name_match, "Gsi4", s.gsi4_name, s.gsi4_hash, s.gsi4_range);
    insert_gsi!(index_meta, index_name_match, "Gsi5", s.gsi5_name, s.gsi5_hash, s.gsi5_range);
    insert_gsi!(index_meta, index_name_match, "Gsi6", s.gsi6_name, s.gsi6_hash, s.gsi6_range);
    insert_gsi!(index_meta, index_name_match, "Gsi7", s.gsi7_name, s.gsi7_hash, s.gsi7_range);
    insert_gsi!(index_meta, index_name_match, "Gsi8", s.gsi8_name, s.gsi8_hash, s.gsi8_range);
    insert_gsi!(index_meta, index_name_match, "Gsi9", s.gsi9_name, s.gsi9_hash, s.gsi9_range);
    insert_gsi!(index_meta, index_name_match, "Gsi10", s.gsi10_name, s.gsi10_hash, s.gsi10_range);
    insert_gsi!(index_meta, index_name_match, "Gsi11", s.gsi11_name, s.gsi11_hash, s.gsi11_range);
    insert_gsi!(index_meta, index_name_match, "Gsi12", s.gsi12_name, s.gsi12_hash, s.gsi12_range);
    insert_gsi!(index_meta, index_name_match, "Gsi13", s.gsi13_name, s.gsi13_hash, s.gsi13_range);
    insert_gsi!(index_meta, index_name_match, "Gsi14", s.gsi14_name, s.gsi14_hash, s.gsi14_range);
    insert_gsi!(index_meta, index_name_match, "Gsi15", s.gsi15_name, s.gsi15_hash, s.gsi15_range);
    insert_gsi!(index_meta, index_name_match, "Gsi16", s.gsi16_name, s.gsi16_hash, s.gsi16_range);
    insert_gsi!(index_meta, index_name_match, "Gsi17", s.gsi17_name, s.gsi17_hash, s.gsi17_range);
    insert_gsi!(index_meta, index_name_match, "Gsi18", s.gsi18_name, s.gsi18_hash, s.gsi18_range);
    insert_gsi!(index_meta, index_name_match, "Gsi19", s.gsi19_name, s.gsi19_hash, s.gsi19_range);
    insert_gsi!(index_meta, index_name_match, "Gsi20", s.gsi20_name, s.gsi20_hash, s.gsi20_range);

    let struct_data = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("could not parse struct"),
    };

    let mut field_inserts = quote! {};
    let mut field_reads = quote! {};

    for field in struct_data.fields.iter() {
        if field.attrs.len() > 0 {
            if let Ok(attribute) = DeezIgnore::from_attributes(&field.attrs) {
                if attribute.ignore {
                    continue;
                }
            }

            read_attr!(index_meta, field, DeezPrimary, "Primary");
            read_attr!(index_meta, field, DeezGsi1, "Gsi1");
            read_attr!(index_meta, field, DeezGsi2, "Gsi2");
            read_attr!(index_meta, field, DeezGsi3, "Gsi3");
            read_attr!(index_meta, field, DeezGsi4, "Gsi4");
            read_attr!(index_meta, field, DeezGsi5, "Gsi5");
            read_attr!(index_meta, field, DeezGsi6, "Gsi6");
            read_attr!(index_meta, field, DeezGsi7, "Gsi7");
            read_attr!(index_meta, field, DeezGsi8, "Gsi8");
            read_attr!(index_meta, field, DeezGsi9, "Gsi9");
            read_attr!(index_meta, field, DeezGsi10, "Gsi10");
            read_attr!(index_meta, field, DeezGsi11, "Gsi11");
            read_attr!(index_meta, field, DeezGsi12, "Gsi12");
            read_attr!(index_meta, field, DeezGsi13, "Gsi13");
            read_attr!(index_meta, field, DeezGsi14, "Gsi14");
            read_attr!(index_meta, field, DeezGsi15, "Gsi15");
            read_attr!(index_meta, field, DeezGsi16, "Gsi16");
            read_attr!(index_meta, field, DeezGsi17, "Gsi17");
            read_attr!(index_meta, field, DeezGsi18, "Gsi18");
            read_attr!(index_meta, field, DeezGsi19, "Gsi19");
            read_attr!(index_meta, field, DeezGsi20, "Gsi20");
        }

        let type_name = match &field.ty {
            syn::Type::Path(p) => p.to_token_stream().to_string(),
            _ => panic!("could not parse field type as path"),
        };

        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();

        match type_name.as_str() {
            "String" => {
                field_inserts = quote! {
                    #field_inserts
                    m.insert(#field_name.to_string(), AttributeValue::S(item.#field_ident.clone()));
                };
                field_reads = quote! {
                    #field_reads
                    #field_ident: item
                        .get(#field_name)
                        .unwrap() // todo: sus
                        .as_s()
                        .unwrap()
                        .clone(),
                };
            }
            "f64" => {
                field_inserts = quote! {
                    #field_inserts
                    m.insert(#field_name.to_string(), AttributeValue::N(item.#field_ident.to_string()));
                };
                field_reads = quote! {
                    #field_reads
                    #field_ident: item
                        .get(#field_name)
                        .unwrap()
                        .as_n()
                        .unwrap()
                        .clone()
                        .parse::<f64>()
                        .unwrap(),
                };
            }
            "bool" => {
                field_inserts = quote! {
                    #field_inserts
                    m.insert(#field_name.to_string(), AttributeValue::Bool(item.#field_ident));
                };
                field_reads = quote! {
                    #field_reads
                    #field_ident: item
                        .get(#field_name)
                        .unwrap()
                        .as_bool()
                        .unwrap()
                        .clone(),
                };
            }
            _ => panic!("unsupported type: {}", type_name),
        }
    }

    let mut index_key_match = quote! {};
    let mut index_keys_match = quote! {};
    let mut index_inserts = quote! {};

    for (k, v) in index_meta.iter() {
        let hash_composite = compose_key!(v.hash);
        let range_composite = compose_key!(v.range);

        let service = s.service.clone();
        let entity = s.entity.clone();
        let hash_field = v.hash.field.clone();
        let range_field = v.range.field.clone();
        let index_enum_variant = format_ident!("{}", k);
        // let index_enum_variant_inner = index_enum_variant!(k, &v.index_name);

        index_key_match = quote! {
            #index_key_match
            Index::#index_enum_variant => {
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
            Index::#index_enum_variant => {
                return IndexKeys {
                    hash: self.index_key(Index::#index_enum_variant, Key::Hash),
                    range: self.index_key(Index::#index_enum_variant, Key::Range),
                }
            }
        };

        index_inserts = quote! {
            #index_inserts
            {
                let keys = item.index_keys(Index::#index_enum_variant);
                m.insert(keys.hash.field, AttributeValue::S(keys.hash.composite));
                m.insert(keys.range.field, AttributeValue::S(keys.range.composite));
            }
        };
    }

    let table = s.table;
    let response_items = format_ident!("{}Items", ident);

    let out = quote! {
        // use deez::{Index, Key, IndexKey, IndexKeys};
        // use aws_sdk_dynamodb::types::AttributeValue;
        // use std::collections::HashMap;

        impl #ident {
            pub fn table_name() -> String {
                #table.to_string()
            }

            pub fn index_name(index: Index) -> String {
                match index {
                    #index_name_match
                    _ => panic!("unknown entity index: {}", index), // todo: custom error
                }
            }

            pub fn index_key(&self, index: Index, key: Key) -> IndexKey<String> {
                match index {
                    #index_key_match
                    _ => panic!("unknown entity index: {}", index),
                }
            }

            pub fn index_key_av(&self, index: Index, key: Key) -> IndexKey<AttributeValue> {
                let k = self.index_key(index, key);
                IndexKey {
                    field: k.field,
                    composite: AttributeValue::S(k.composite),
                }
            }

            pub fn index_keys(&self, index: Index) -> IndexKeys<String> {
                match index {
                    #index_keys_match
                    _ => panic!("unknown entity index: {}", index),
                }
            }

            pub fn index_keys_av(&self, index: Index) -> IndexKeys<AttributeValue> {
                let k = self.index_keys(index);
                IndexKeys {
                    hash: IndexKey {
                        field: k.hash.field,
                        composite: AttributeValue::S(k.hash.composite),
                    },
                    range: IndexKey {
                        field: k.range.field,
                        composite: AttributeValue::S(k.range.composite),
                    }
                }
            }
        }

        impl From<#ident> for HashMap<String, AttributeValue> {
            fn from(item: #ident) -> HashMap<String, AttributeValue> {
                let mut m: HashMap<String, AttributeValue> = HashMap::new();
                #field_inserts
                #index_inserts
                m
            }
        }

        impl From<&HashMap<String, AttributeValue>> for #ident {
            fn from(item: &HashMap<String, AttributeValue>) -> #ident {
                #ident {
                    #field_reads
                    ..Default::default()
                }
            }
        }

        #[derive(Debug)]
        pub struct #response_items(pub Vec<#ident>);

        impl From<&[HashMap<String, AttributeValue>]> for #response_items {
            fn from(item: &[HashMap<String, AttributeValue>]) -> #response_items {
                let mut items: Vec<#ident> = Vec::new();
                for i in item {
                    items.push(i.into());
                }
                #response_items(items)
            }
        }

        impl #response_items {
            pub fn items(self) -> Vec<#ident> {
                self.0
            }
        }

        // // todo: delete?
        // impl From<HashMap<String, AttributeValue>> for #ident {
        //     fn from(item: HashMap<String, AttributeValue>) -> #ident {
        //         #ident {
        //             #field_reads
        //             ..Default::default()
        //         }
        //     }
        // }

        // todo: makes no sense why this doesn't work
        // #[derive(Debug)]
        // pub struct #response_items {
        //     pub items: Vec<#ident>,
        // };

        // impl From<&[HashMap<String, AttributeValue>]> for #response_items {
        //     fn from(item: &[HashMap<String, AttributeValue>]) -> #response_items {
        //         let mut items: Vec<#ident> = Vec::new();
        //         for i in item {
        //             items.push(i.into());
        //         }
        //         #response_items{items}
        //     }
        // }
    };

    out.into()
}
