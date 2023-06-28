macro_rules! insert_index {
    (
        $index_meta:ident,
        $index:expr,
        $hash_name:expr,
        $range_name:expr
    ) => {
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
pub(crate) use insert_index;

macro_rules! insert_gsi {
    (
        $index_meta:ident,
        $index_name_fns:ident,
        $index:expr,
        $index_name:expr,
        $hash_name:expr,
        $range_name:expr
    ) => {
        if let Some(index_name) = $index_name {
            let hash_name = match $hash_name {
                Some(x) => x,
                None => panic!("hash key name not provided for index: {}", $index),
            };
            let range_name = match $range_name {
                Some(x) => x,
                None => panic!("range key name not provided for index: {}", $index),
            };
            insert_index!($index_meta, $index, hash_name, range_name); // todo: remove unwrap
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
pub(crate) use insert_gsi;

macro_rules! read_attr {
    ($index_meta:ident, $field:expr, $index_attr:ident, $index:expr) => {
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
pub(crate) use read_attr;

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
                if let Some(inner) = &self.#field_ident {
                    index_key.composite.push_str(&format!("#{}_{}", #field_name, inner.to_string()));
                } else {
                    return index_key;
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
pub(crate) use compose_key;

macro_rules! insert_read {
    (
        $field_ident:ident, $field_type:ident, $field_attrs:expr, $insert:ident, $read:ident;

        insert_list: $insert_list:expr,
        read_list: $read_list:expr,
        insert_set: $insert_set:expr,
        read_set: $read_set:expr,
        insert_scalar: $insert_scalar:expr,
        read_scalar: $read_scalar:expr,
        insert_option_list: $insert_option_list:expr,
        read_option_list: $read_option_list:expr,
        insert_option_set: $insert_option_set:expr,
        read_option_set: $read_option_set:expr,
        insert_option_scalar: $insert_option_scalar:expr,
        read_option_scalar: $read_option_scalar:expr
    ) => {
        if $field_type.is_option {
            if $field_type.is_vec {
                let deez_vec = DeezVec::from_attributes(&$field_attrs).unwrap();
                $insert = match deez_vec.dynamo_type.as_str() {
                    "list" => $insert_option_list,
                    "set" => $insert_option_set,
                    _ => panic!("dynamo_type must be either `list` or `set`"),
                };
                $read = match deez_vec.dynamo_type.as_str() {
                    "list" => $read_option_list,
                    "set" => $read_option_set,
                    _ => panic!("dynamo_type must be either `list` or `set`"),
                };
            } else {
                $insert = $insert_option_scalar;
                $read = $read_option_scalar;
            };
            $insert = quote! {
                match item.#$field_ident {
                    Some(inner) => #$insert,
                    None => AttributeValue::Null(true),
                }
            };
        } else {
            if $field_type.is_vec {
                let deez_vec = DeezVec::from_attributes(&$field_attrs).unwrap();
                $insert = match deez_vec.dynamo_type.as_str() {
                    "list" => $insert_list,
                    "set" => $insert_set,
                    _ => panic!("dynamo_type must be either `list` or `set`"),
                };
                $read = match deez_vec.dynamo_type.as_str() {
                    "list" => $read_list,
                    "set" => $read_set,
                    _ => panic!("dynamo_type must be either `list` or `set`"),
                };
            } else {
                $insert = $insert_scalar;
                $read = $read_scalar;
            };
        }
    };
}
pub(crate) use insert_read;

macro_rules! attr_derive {
    () => {
        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_vec)]
        struct DeezVec {
            dynamo_type: String,
        }

        // todo: cant use empty struct???
        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_ignore)]
        struct DeezIgnore {
            #[attribute(optional = false, default = true)]
            ignore: bool,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_schema)]
        // #[attribute(invalid_field = "ok")]
        struct DeezSchema {
            service: String,
            table: String,
            entity: String,

            primary_hash: String,
            primary_range: String,

            gsi1_name: Option<String>,
            gsi1_hash: Option<String>,
            gsi1_range: Option<String>,

            gsi2_name: Option<String>,
            gsi2_hash: Option<String>,
            gsi2_range: Option<String>,

            gsi3_name: Option<String>,
            gsi3_hash: Option<String>,
            gsi3_range: Option<String>,

            gsi4_name: Option<String>,
            gsi4_hash: Option<String>,
            gsi4_range: Option<String>,

            gsi5_name: Option<String>,
            gsi5_hash: Option<String>,
            gsi5_range: Option<String>,

            gsi6_name: Option<String>,
            gsi6_hash: Option<String>,
            gsi6_range: Option<String>,

            gsi7_name: Option<String>,
            gsi7_hash: Option<String>,
            gsi7_range: Option<String>,

            gsi8_name: Option<String>,
            gsi8_hash: Option<String>,
            gsi8_range: Option<String>,

            gsi9_name: Option<String>,
            gsi9_hash: Option<String>,
            gsi9_range: Option<String>,

            gsi10_name: Option<String>,
            gsi10_hash: Option<String>,
            gsi10_range: Option<String>,

            gsi11_name: Option<String>,
            gsi11_hash: Option<String>,
            gsi11_range: Option<String>,

            gsi12_name: Option<String>,
            gsi12_hash: Option<String>,
            gsi12_range: Option<String>,

            gsi13_name: Option<String>,
            gsi13_hash: Option<String>,
            gsi13_range: Option<String>,

            gsi14_name: Option<String>,
            gsi14_hash: Option<String>,
            gsi14_range: Option<String>,

            gsi15_name: Option<String>,
            gsi15_hash: Option<String>,
            gsi15_range: Option<String>,

            gsi16_name: Option<String>,
            gsi16_hash: Option<String>,
            gsi16_range: Option<String>,

            gsi17_name: Option<String>,
            gsi17_hash: Option<String>,
            gsi17_range: Option<String>,

            gsi18_name: Option<String>,
            gsi18_hash: Option<String>,
            gsi18_range: Option<String>,

            gsi19_name: Option<String>,
            gsi19_hash: Option<String>,
            gsi19_range: Option<String>,

            gsi20_name: Option<String>,
            gsi20_hash: Option<String>,
            gsi20_range: Option<String>,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_primary)]
        struct DeezPrimary {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi1)]
        struct DeezGsi1 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi2)]
        struct DeezGsi2 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi3)]
        struct DeezGsi3 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi4)]
        struct DeezGsi4 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi5)]
        struct DeezGsi5 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi6)]
        struct DeezGsi6 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi7)]
        struct DeezGsi7 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi8)]
        struct DeezGsi8 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi9)]
        struct DeezGsi9 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi10)]
        struct DeezGsi10 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi11)]
        struct DeezGsi11 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi12)]
        struct DeezGsi12 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi13)]
        struct DeezGsi13 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi14)]
        struct DeezGsi14 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi15)]
        struct DeezGsi15 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi16)]
        struct DeezGsi16 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi17)]
        struct DeezGsi17 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi18)]
        struct DeezGsi18 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi19)]
        struct DeezGsi19 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }

        #[derive(Attribute, Debug)]
        #[attribute(ident = deez_gsi20)]
        struct DeezGsi20 {
            #[attribute(default = 0)]
            position: usize,
            key: String,
        }
    };
}

pub(crate) use attr_derive;
