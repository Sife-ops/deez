mod macros;
mod types;

use crate::types::{Composite, FieldType, IndexKey, IndexKeys};
use attribute_derive::Attribute;
use macros::{attr_derive, compose_key, insert_gsi, insert_index, insert_read, read_attr};
use proc_macro::{self, TokenStream};
use quote::{format_ident, quote};
use std::{collections::HashMap, fmt::Debug};
use syn::DeriveInput;

attr_derive!();

// todo: no unwrap and use span for errors(?)
#[proc_macro_derive(
    Deez,
    attributes(
        deez_schema,
        deez_ignore,
        deez_vec,
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

    let struct_data = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("could not parse struct"),
    };

    let mut index_meta = HashMap::new();
    let mut index_name_fns = quote! {};
    let mut index_key_fns = quote! {};
    let mut index_keys_fns = quote! {};
    let mut index_inserts = quote! {};
    let mut struct_impl = quote! {};

    if let Ok(s) = DeezSchema::from_attributes(&attrs) {
        insert_index!(index_meta, "primary", s.primary_hash, s.primary_range);
        insert_gsi!(index_meta, index_name_fns, "gsi1", s.gsi1_name, s.gsi1_hash, s.gsi1_range);
        insert_gsi!(index_meta, index_name_fns, "gsi2", s.gsi2_name, s.gsi2_hash, s.gsi2_range);
        insert_gsi!(index_meta, index_name_fns, "gsi3", s.gsi3_name, s.gsi3_hash, s.gsi3_range);
        insert_gsi!(index_meta, index_name_fns, "gsi4", s.gsi4_name, s.gsi4_hash, s.gsi4_range);
        insert_gsi!(index_meta, index_name_fns, "gsi5", s.gsi5_name, s.gsi5_hash, s.gsi5_range);
        insert_gsi!(index_meta, index_name_fns, "gsi6", s.gsi6_name, s.gsi6_hash, s.gsi6_range);
        insert_gsi!(index_meta, index_name_fns, "gsi7", s.gsi7_name, s.gsi7_hash, s.gsi7_range);
        insert_gsi!(index_meta, index_name_fns, "gsi8", s.gsi8_name, s.gsi8_hash, s.gsi8_range);
        insert_gsi!(index_meta, index_name_fns, "gsi9", s.gsi9_name, s.gsi9_hash, s.gsi9_range);
        insert_gsi!(index_meta, index_name_fns, "gsi10", s.gsi10_name, s.gsi10_hash, s.gsi10_range);
        insert_gsi!(index_meta, index_name_fns, "gsi11", s.gsi11_name, s.gsi11_hash, s.gsi11_range);
        insert_gsi!(index_meta, index_name_fns, "gsi12", s.gsi12_name, s.gsi12_hash, s.gsi12_range);
        insert_gsi!(index_meta, index_name_fns, "gsi13", s.gsi13_name, s.gsi13_hash, s.gsi13_range);
        insert_gsi!(index_meta, index_name_fns, "gsi14", s.gsi14_name, s.gsi14_hash, s.gsi14_range);
        insert_gsi!(index_meta, index_name_fns, "gsi15", s.gsi15_name, s.gsi15_hash, s.gsi15_range);
        insert_gsi!(index_meta, index_name_fns, "gsi16", s.gsi16_name, s.gsi16_hash, s.gsi16_range);
        insert_gsi!(index_meta, index_name_fns, "gsi17", s.gsi17_name, s.gsi17_hash, s.gsi17_range);
        insert_gsi!(index_meta, index_name_fns, "gsi18", s.gsi18_name, s.gsi18_hash, s.gsi18_range);
        insert_gsi!(index_meta, index_name_fns, "gsi19", s.gsi19_name, s.gsi19_hash, s.gsi19_range);
        insert_gsi!(index_meta, index_name_fns, "gsi20", s.gsi20_name, s.gsi20_hash, s.gsi20_range);

        for field in struct_data.fields.iter() {
            // todo: remove length check?
            if field.attrs.len() > 0 {
                read_attr!(index_meta, field, DeezPrimary, "primary");
                read_attr!(index_meta, field, DeezGsi1, "gsi1");
                read_attr!(index_meta, field, DeezGsi2, "gsi2");
                read_attr!(index_meta, field, DeezGsi3, "gsi3");
                read_attr!(index_meta, field, DeezGsi4, "gsi4");
                read_attr!(index_meta, field, DeezGsi5, "gsi5");
                read_attr!(index_meta, field, DeezGsi6, "gsi6");
                read_attr!(index_meta, field, DeezGsi7, "gsi7");
                read_attr!(index_meta, field, DeezGsi8, "gsi8");
                read_attr!(index_meta, field, DeezGsi9, "gsi9");
                read_attr!(index_meta, field, DeezGsi10, "gsi10");
                read_attr!(index_meta, field, DeezGsi11, "gsi11");
                read_attr!(index_meta, field, DeezGsi12, "gsi12");
                read_attr!(index_meta, field, DeezGsi13, "gsi13");
                read_attr!(index_meta, field, DeezGsi14, "gsi14");
                read_attr!(index_meta, field, DeezGsi15, "gsi15");
                read_attr!(index_meta, field, DeezGsi16, "gsi16");
                read_attr!(index_meta, field, DeezGsi17, "gsi17");
                read_attr!(index_meta, field, DeezGsi18, "gsi18");
                read_attr!(index_meta, field, DeezGsi19, "gsi19");
                read_attr!(index_meta, field, DeezGsi20, "gsi20");
            }
        }

        for (k, v) in index_meta.iter() {
            let service = s.service.clone();
            let entity = s.entity.clone();
            let hash_field = v.hash.field.clone();
            let range_field = v.range.field.clone();

            let composed_hash = compose_key!(v.hash);
            let composed_range = compose_key!(v.range);

            let index_key_fn_name = format_ident!("{}_key", k);
            index_key_fns = quote! {
                #index_key_fns
                pub fn #index_key_fn_name(&self, key: Key) -> IndexKey {
                    let mut index_key = IndexKey {
                        ..Default::default()
                    };

                    match key {
                        Key::Hash => {
                            index_key.field = #hash_field.to_string();
                            index_key.composite.push_str(&format!("${}#{}", #service, #entity));
                            #composed_hash
                        }
                        Key::Range => {
                            index_key.field = #range_field.to_string();
                            index_key.composite.push_str(&format!("${}", #entity));
                            #composed_range
                        }
                    }
                }
            };

            let index_keys_fn_name = format_ident!("{}_keys", k);
            index_keys_fns = quote! {
                #index_keys_fns
                pub fn #index_keys_fn_name(&self) -> IndexKeys {
                    IndexKeys {
                        hash: self.#index_key_fn_name(Key::Hash),
                        range: self.#index_key_fn_name(Key::Range),
                    }
                }
            };

            index_inserts = quote! {
                #index_inserts
                {
                    let keys = item.#index_keys_fn_name();
                    // m.insert(keys.hash.field, AttributeValue::S(keys.hash.composite));
                    m.insert(keys.hash.field(), keys.hash.av());
                    m.insert(keys.range.field(), keys.range.av());
                }
            };
        }

        let table = s.table;
        struct_impl = quote! {
            impl #ident {
                #index_name_fns
                #index_key_fns
                #index_keys_fns

                pub fn table_name() -> String {
                    #table.to_string()
                }

                pub fn table__name(&self) -> String {
                    #table.to_string()
                }
            }
        };
    }

    let mut field_inserts = quote! {};
    let mut field_reads = quote! {};

    // todo: Null?
    for field in struct_data.fields.iter() {
        if field.attrs.len() > 0 {
            if let Ok(attribute) = DeezIgnore::from_attributes(&field.attrs) {
                if attribute.ignore {
                    continue;
                }
            }
        }

        let field_type = FieldType::new(&field.ty);
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let mut attribute_value_insert: proc_macro2::TokenStream;
        let attribute_value_read: proc_macro2::TokenStream;

        macro_rules! read_option {
            ($variant:ident, $e:expr) => {{
                let e = $e;
                quote! {
                    match &item[#field_name] {
                        AttributeValue::$variant(inner) => Some(inner #e),
                        AttributeValue::Null(_) => None,
                        _ => panic!("unexpected AttributeValue variant"),
                    }
                }
            }};
        }

        match field_type.inner_name.as_str() {
            "String" => {
                let list_insert = quote! {
                    .iter()
                    .map(|x| AttributeValue::S(x.clone()))
                    .collect::<Vec<AttributeValue>>()
                };
                let list_read = quote! {
                    .iter()
                    .map(|x| x.as_s().unwrap().clone())
                    .collect::<Vec<String>>()
                };

                insert_read!(
                    field_ident, field_type, field.attrs, attribute_value_insert, attribute_value_read;

                    insert_list: quote! { AttributeValue::L(item.#field_ident #list_insert) },
                    read_list: quote! { item[#field_name].as_l().unwrap() #list_read },
                    insert_set: quote! { AttributeValue::Ss(item.#field_ident) },
                    read_set: quote! { item[#field_name].as_ss().unwrap().clone() },
                    insert_scalar: quote! { AttributeValue::S(item.#field_ident) },
                    read_scalar: quote! { item[#field_name].as_s().unwrap().clone() },

                    insert_option_list: quote! { AttributeValue::L(inner #list_insert) },
                    read_option_list: read_option!(L, quote! { #list_read }),
                    insert_option_set: quote! { AttributeValue::Ss(inner) },
                    read_option_set: read_option!(Ss, quote! { .clone() }),
                    insert_option_scalar: quote! { AttributeValue::S(inner) },
                    read_option_scalar: read_option!(S, quote! { .clone() })
                );
            }

            "f64" => {
                let list_insert = quote! {
                    .iter()
                    .map(|x| AttributeValue::N(x.to_string()))
                    .collect::<Vec<AttributeValue>>()
                };
                let set_insert = quote! {
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                };
                let list_read = quote! {
                    .iter()
                    .map(|x| x.as_n().unwrap().parse::<f64>().unwrap())
                    .collect::<Vec<f64>>()
                };
                let set_read = quote! {
                    .iter()
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>()
                };

                insert_read!(
                    field_ident, field_type, field.attrs, attribute_value_insert, attribute_value_read;

                    insert_list: quote! { AttributeValue::L(item.#field_ident #list_insert) },
                    read_list: quote! { item[#field_name].as_l().unwrap() #list_read },
                    insert_set: quote! { AttributeValue::Ns(item.#field_ident #set_insert) },
                    read_set: quote! { item[#field_name].as_ns().unwrap() #set_read },
                    insert_scalar: quote! { AttributeValue::N(item.#field_ident.to_string()) },
                    read_scalar: quote! { item[#field_name].as_n().unwrap().parse::<f64>().unwrap() },

                    insert_option_list: quote! { AttributeValue::L(inner #list_insert) },
                    read_option_list: read_option!(L, quote! { #list_read }),
                    insert_option_set: quote! { AttributeValue::Ns(inner #set_insert) },
                    read_option_set: read_option!(Ns, quote! { #set_read }),
                    insert_option_scalar: quote! { AttributeValue::N(inner.to_string()) },
                    read_option_scalar: read_option!(N, quote! { .clone().parse::<f64>().unwrap() })
                );
            }

            "bool" => {
                let list_insert = quote! {
                    .iter()
                    .map(|x| AttributeValue::Bool(x.clone()))
                    .collect::<Vec<AttributeValue>>()
                };
                let list_read = quote! {
                    .iter()
                    .map(|x| x.as_bool().unwrap().clone())
                    .collect::<Vec<bool>>()
                };

                insert_read!(
                    field_ident, field_type, field.attrs, attribute_value_insert, attribute_value_read;

                    insert_list: quote! { AttributeValue::L(item.#field_ident #list_insert) },
                    read_list: quote! { item[#field_name].as_l().unwrap() #list_read },
                    insert_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    read_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    insert_scalar: quote! { AttributeValue::Bool(item.#field_ident) },
                    read_scalar: quote! { item[#field_name].as_bool().unwrap().clone() },

                    insert_option_list: quote! { AttributeValue::L(inner #list_insert) },
                    read_option_list: read_option!(L, quote! { #list_read }),
                    insert_option_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    read_option_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    insert_option_scalar: quote! { AttributeValue::Bool(inner) },
                    read_option_scalar: read_option!(Bool, quote! { .clone() })
                );
            }

            // todo: better pattern matching for "Blob"
            "Blob" => {
                let list_insert = quote! {
                    .iter()
                    .map(|x| AttributeValue::B(x.clone()))
                    .collect::<Vec<AttributeValue>>()
                };
                let list_read = quote! {
                    .iter()
                    .map(|x| x.as_b().unwrap().clone())
                    .collect::<Vec<Blob>>()
                };

                insert_read!(
                    field_ident, field_type, field.attrs, attribute_value_insert, attribute_value_read;

                    insert_list: quote! { AttributeValue::L(item.#field_ident #list_insert) },
                    read_list: quote! { item[#field_name].as_l().unwrap() #list_read },
                    insert_set: quote! { AttributeValue::Bs(item.#field_ident) },
                    read_set: quote! { item[#field_name].as_bs().unwrap().clone() },
                    insert_scalar: quote! { AttributeValue::B(item.#field_ident) },
                    read_scalar: quote! { item[#field_name].as_b().unwrap().clone() },

                    insert_option_list: quote! { AttributeValue::L(inner #list_insert) },
                    read_option_list: read_option!(L, quote! { #list_read }),
                    insert_option_set: quote! { AttributeValue::Bs(inner) },
                    read_option_set: read_option!(Bs, quote! { .clone() }),
                    insert_option_scalar: quote! { AttributeValue::B(inner) },
                    read_option_scalar: read_option!(B, quote! { .clone() })
                );
            }

            _ => {
                let field_type_ident = format_ident!("{}", field_type.inner_name);
                let list_insert = quote! {
                    .into_iter()
                    .map(|x| AttributeValue::M(x.into()))
                    .collect::<Vec<AttributeValue>>()
                };
                let list_read = quote! {
                    .iter()
                    .map(|x| x.as_m().unwrap().into())
                    .collect::<Vec<#field_type_ident>>()
                };

                insert_read!(
                    field_ident, field_type, field.attrs, attribute_value_insert, attribute_value_read;

                    insert_list: quote! { AttributeValue::L(item.#field_ident #list_insert) },
                    read_list: quote! { item[#field_name].as_l().unwrap() #list_read },
                    insert_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    read_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    insert_scalar: quote! { AttributeValue::M(item.#field_ident.into()) },
                    read_scalar: quote! { item[#field_name].as_m().unwrap().into() },

                    insert_option_list: quote! { AttributeValue::L(inner #list_insert) },
                    read_option_list: quote! {
                        match &item[#field_name] {
                            AttributeValue::L(inner) => Some(inner #list_read),
                            AttributeValue::Null(_) => None,
                            _ => panic!("unexpected AttributeValue variant"),
                        }
                    },
                    insert_option_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    read_option_set: panic!("invalid type for `set`: {}", field_type.inner_name),
                    insert_option_scalar: quote! { AttributeValue::M(inner.into()) },
                    read_option_scalar: read_option!(M, quote! { .into() })
                );
            }
        }

        field_inserts = quote! {
            #field_inserts
            m.insert(#field_name.to_string(), #attribute_value_insert);
        };

        field_reads = quote! {
            #field_reads
            i.#field_ident = #attribute_value_read;
        };
    }

    let response_items = format_ident!("{}Items", ident);

    let out = quote! {
        #struct_impl

        impl From<#ident> for HashMap<String, AttributeValue> {
            fn from(item: #ident) -> HashMap<String, AttributeValue> {
                let mut m: HashMap<String, AttributeValue> = HashMap::new();
                #index_inserts
                #field_inserts
                m
            }
        }

        impl From<&HashMap<String, AttributeValue>> for #ident {
            fn from(item: &HashMap<String, AttributeValue>) -> #ident {
                let mut i = #ident {
                    ..Default::default()
                };
                #field_reads
                i
            }
        }

        #[derive(Debug)]
        pub struct #response_items(pub Vec<#ident>);

        impl #response_items {
            pub fn items(self) -> Vec<#ident> {
                self.0
            }
        }

        impl From<&[HashMap<String, AttributeValue>]> for #response_items {
            fn from(item: &[HashMap<String, AttributeValue>]) -> #response_items {
                let mut items: Vec<#ident> = Vec::new();
                for i in item {
                    items.push(i.into());
                }
                #response_items(items)
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
