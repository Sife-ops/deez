use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Expr, Lit};

// code references:
// https://github.com/ex0dus-0x/structmap/blob/main/structmap-derive/src/lib.rs
// https://github.com/imbolc/rust-derive-macro-guide

// todo: implementation of other dynamodb types
// https://github.com/awslabs/aws-sdk-rust/blob/main/sdk/dynamodb/src/types/_attribute_value.rs
// B(::aws_smithy_types::Blob),
// Bs(::std::vec::Vec<::aws_smithy_types::Blob>),
// L(::std::vec::Vec<crate::types::AttributeValue>),
// M(::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>),
// Ns(::std::vec::Vec<::std::string::String>),
// Null(bool),
// Ss(::std::vec::Vec<::std::string::String>),

fn trim(c: &str) -> &str {
    let mut d = c.chars();
    d.next();
    d.next_back();
    d.as_str()
}

#[proc_macro_derive(DeezEntity, attributes(deez))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        syn::Data::Struct(st) => st.fields,
        _ => panic!("ast.data not a `Struct`"),
    };

    let mut inserts = quote! {};
    let mut reads = quote! {};
    for field in fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        let mut field_name = field_ident.to_string();
        let mut field_skip = false;

        // todo: sus af, not enough experience with syn
        // todo: parse floats when reading from the table
        for attr in field.attrs.iter() {
            if let Ok(ex) = attr.parse_args::<Expr>() {
                match ex {
                    Expr::Assign(ea) => {
                        if let Expr::Path(ep) = *ea.left {
                            match ep.path.segments.first().unwrap().ident.to_string().as_str() {
                                "rename" => {
                                    if let Expr::Lit(el) = *ea.right {
                                        if let Lit::Str(ls) = el.lit {
                                            let rename = ls.token().to_string();
                                            field_name = trim(&rename).to_string();
                                        }
                                    }
                                }
                                &_ => {
                                    // todo: do nothing or panic?
                                }
                            }
                        }
                    }
                    Expr::Path(ep) => {
                        match ep.path.segments.first().unwrap().ident.to_string().as_str() {
                            "skip" => {
                                field_skip = true;
                            }
                            &_ => {
                                // todo: do nothing or panic?
                            }
                        }
                    }
                    _ => {
                        // todo: do nothing or panic?
                    }
                }
            }
        }

        if field_skip {
            continue;
        }

        let field_type = match &field.ty {
            syn::Type::Path(b) => b.clone(),
            _ => panic!("field.ty not a `TypePath`"),
        };

        match field_type.clone().into_token_stream().to_string().as_ref() {
            "String" => {
                inserts = quote! {
                    #inserts
                    av_map.insert(#field_name.to_string(), AttributeValue::S(self.#field_ident.to_string()));
                };
                reads = quote! {
                    #reads
                    #field_ident: av_map
                        .get(#field_name)
                        .ok_or(DeezError::MapKey(#field_name.to_string()))?
                        .as_s()?
                        .clone(),
                }
            }
            "bool" => {
                inserts = quote! {
                    #inserts
                    av_map.insert(#field_name.to_string(), AttributeValue::Bool(self.#field_ident));
                };
                reads = quote! {
                    #reads
                    #field_ident: av_map
                        .get(#field_name)
                        .ok_or(DeezError::MapKey(#field_name.to_string()))?
                        .as_bool()?
                        .clone(),
                }
            }
            // DynamoDB attribute of type Number can store 126-bit integers (or
            // 127-bit unsigned integers, with serious caveats).
            // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
            "usize" | "isize" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" => {
                inserts = quote! {
                    #inserts
                    av_map.insert(#field_name.to_string(), AttributeValue::N(self.#field_ident.to_string()));
                };
                reads = quote! {
                    #reads
                    #field_ident: av_map
                        .get(#field_name)
                        .ok_or(DeezError::MapKey(#field_name.to_string()))?
                        .as_n()?
                        .clone()
                        .parse::<#field_type>()?,
                };
            }
            &_ => panic!(
                "unsupported type: {}",
                field_type.clone().into_token_stream().to_string()
            ),
        }
    }

    let name = &ast.ident;

    let macro_put = format_ident!("put_{}", name);
    // let macro_batch_write = format_ident!("batch_write_{}", name);
    let macro_query = format_ident!("query_{}", name);
    let macro_delete = format_ident!("delete_{}", name);

    let (ig, tg, wc) = ast.generics.split_for_impl();

    let output = quote! {
        impl #ig DeezEntity for #name #tg #wc {
            fn to_av_map(&self) -> HashMap<String, AttributeValue> {
                let mut av_map = HashMap::new();
                #inserts
                av_map
            }
            // todo: return two hashmaps
            fn to_av_map_with_keys(&self) -> Result<HashMap<String, AttributeValue>, DeezError> {
                let mut av_map = self.to_av_map();
                let indexes = self.indexes();
                for (_, index) in indexes.iter() {
                    av_map.insert(
                        index.partition_key.field.to_string(),
                        AttributeValue::S(format!(
                            "${}#{}{}",
                            self.meta().service,
                            self.meta().entity,
                            index.partition_key._join_composite(&av_map)?,
                        ))
                    );
                    av_map.insert(
                        index.sort_key.field.to_string(),
                        AttributeValue::S(format!(
                            "#{}{}",
                            self.meta().entity,
                            index.sort_key._join_composite(&av_map)?,
                        ))
                    );
                }
                Ok(av_map)
            }
            fn from_av_map(av_map: &HashMap<String, AttributeValue>) -> Result<#name, DeezError> {
                Ok(#name {
                    #reads
                    ..Default::default()
                })
            }
        }

        #[macro_export]
        macro_rules! #macro_put {
            ($d: ident, $e: expr) => {{
                $d
                    .put(&$e)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
            }};
        }

        // // todo: variadic, conditional expansion...
        // #[macro_export]
        // macro_rules! #macro_batch_write {
        // }

        #[macro_export]
        macro_rules! #macro_query {
            ($d: ident, $i: ident, $e: expr) => {{
                let a = $d
                    .query($i, &$e)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                let b = a.items().unwrap();
                #name::from_av_map_slice(b).unwrap()
            }};
        }

        #[macro_export]
        macro_rules! #macro_delete {
            ($d: ident, $e: expr) => {{
                $d
                    .delete(&$e)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
            }};
        }
    };
    output.into()
}
