mod macros;

use attribute_derive::Attribute;
use macros::attr_derive;
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

    let mut index_meta = HashMap::new();
    let mut index_name_fns = quote! {};

    macro_rules! insert_index {
        ($index_meta: ident, $index: expr, $hash_name: expr, $range_name: expr) => {
            $index_meta.insert(
                $index.to_string(),
                IndexKeys {
                    hash: IndexKey {
                        field: $hash_name,
                        ..Default::default()
                    },
                    range: IndexKey {
                        field: $range_name,
                        ..Default::default()
                    },
                },
            );
        };
    }

    macro_rules! insert_gsi {
        ($index_meta: ident, $index_name_fns: ident, $index: expr, $index_name: expr, $hash_name: expr, $range_name: expr) => {
            if let Some(index_name) = $index_name {
                insert_index!($index_meta, $index, $hash_name.unwrap(), $range_name.unwrap());
                let index_name_fn_name = format_ident!("{}_name", $index);
                $index_name_fns = quote! {
                    #$index_name_fns
                    pub fn #index_name_fn_name() -> String {
                        #index_name.to_string()
                    }
                };
            }
        };
    }

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

            macro_rules! read_attr {
                ($index_meta: ident, $field: expr, $index_attr: ident, $index: expr) => {
                    if let Ok(attribute) = $index_attr::from_attributes(&$field.attrs) {
                        let composite = Composite {
                            position: attribute.position,
                            syn_field: $field.clone(),
                        };
                        if let Some(index) = $index_meta.get_mut($index) {
                            match attribute.key.as_str() {
                                "hash" => index.hash.composite.push(composite),
                                "range" => index.range.composite.push(composite),
                                _ => panic!("key must be either `hash` or `range`"),
                            }
                        } else {
                            panic!("unknown index: {}", $index);
                        }
                    }
                };
            }

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
                        .unwrap_or(&AttributeValue::S("".to_string())) // todo: sus
                        .as_s()
                        .unwrap_or(&"".to_string())
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
                        .unwrap_or(&AttributeValue::N("0".to_string()))
                        .as_n()
                        .unwrap_or(&"0".to_string())
                        .clone()
                        .parse::<f64>()
                        .unwrap_or(0.0),
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
                        .unwrap_or(&AttributeValue::Bool(false))
                        .as_bool()
                        .unwrap_or(&false)
                        .clone(),
                };
            }
            _ => panic!("unsupported type: {}", type_name),
        }
    }

    let mut index_key_fns = quote! {};
    let mut index_keys_fns = quote! {};
    let mut index_inserts = quote! {};

    for (k, v) in index_meta.iter() {
        let service = s.service.clone();
        let entity = s.entity.clone();
        let hash_field = v.hash.field.clone();
        let range_field = v.range.field.clone();

        macro_rules! compose_key {
            ($index_key: expr) => {{
                let mut c = quote! {};

                for (i, _) in $index_key.composite.iter().enumerate() {
                    let composite = match $index_key.composite.iter().find(|c| c.position == i) {
                        Some(c) => c,
                        None => panic!("missing composite for {} at position: {}", $index_key.field, i),
                    };
                    let field_ident = match composite.syn_field.ident.as_ref() {
                        Some(i) => i,
                        None => panic!("could not parse field ident for index: {}", $index_key.field),
                    };
                    let field_name = field_ident.to_string();

                    c = quote! {
                        #c
                        {
                            let value_string = self.#field_ident.to_string();
                            if value_string == "" || value_string == "0" {
                                return index_key;
                            }
                            index_key.composite.push_str(&format!("#{}_{}", #field_name, value_string));
                        }
                    };
                }

                c = quote! {
                    #c
                    return index_key;
                };

                c
            }};
        }

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
    let response_items = format_ident!("{}Items", ident);

    let out = quote! {
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
