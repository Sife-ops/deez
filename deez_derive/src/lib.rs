use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Expr};

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

#[proc_macro_derive(DeezEntity, attributes(deez))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        syn::Data::Struct(st) => st.fields,
        _ => panic!("todo"),
    };

    let mut inserts = quote! {};
    let mut reads = quote! {};
    for field in fields.iter() {
        let ident = match field.ident.as_ref() {
            Some(ident) => ident,
            _ => continue,
        };

        let field_type = match &field.ty {
            syn::Type::Path(tp) => tp.clone().into_token_stream().to_string(),
            _ => continue,
        };

        let mut field_name = ident.to_string();

        // todo: improve on parsing the option attributes, add options for
        // skipping fields and defining floats
        if let Some(first) = field.attrs.first() {
            if let Ok(ex) = first.parse_args::<Expr>() {
                if let Expr::Assign(ea) = ex {
                    if let Expr::Lit(el) = *ea.right {
                        if let syn::Lit::Str(ls) = el.lit {
                            let t = ls.token().to_string();
                            let mut c = t.chars();
                            c.next();
                            c.next_back();
                            field_name = c.as_str().to_string();
                        }
                    }
                }
            }
        }

        // todo: there may be a syn type that would greatly simplify the
        // matching of these types
        match field_type.as_ref() {
            "String" => {
                inserts = quote! {
                    #inserts
                    av_map.insert(#field_name.to_string(), AttributeValue::S(self.#ident.to_string()));
                };
                reads = quote! {
                    #reads
                    #ident: av_map
                        .get(#field_name)
                        .ok_or(DeezError::MapKey(#field_name.to_string()))?
                        .as_s()?
                        .clone(),
                }
            }
            "bool" => {
                inserts = quote! {
                    #inserts
                    av_map.insert(#field_name.to_string(), AttributeValue::Bool(self.#ident));
                };
                reads = quote! {
                    #reads
                    #ident: av_map
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
                    av_map.insert(#field_name.to_string(), AttributeValue::N(self.#ident.to_string()));
                };
                match field_type.as_ref() {
                    "usize" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<usize>()?,
                        }
                    }
                    "isize" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<isize>()?,
                        }
                    }
                    "u8" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<u8>()?,
                        }
                    }
                    "i8" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<i8>()?,
                        }
                    }
                    "u16" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<u16>()?,
                        }
                    }
                    "i16" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<i16>()?,
                        }
                    }
                    "u32" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<u32>()?,
                        }
                    }
                    "i32" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<i32>()?,
                        }
                    }
                    "u64" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<u64>()?,
                        }
                    }
                    "i64" => {
                        reads = quote! {
                            #reads
                            #ident: av_map
                                .get(#field_name)
                                .ok_or(DeezError::MapKey(#field_name.to_string()))?
                                .as_n()?
                                .clone()
                                .parse::<i64>()?,
                        }
                    }
                    &_ => continue, // todo: what to do if field is skipped
                }
            }
            &_ => continue, // todo: what to do if field is skipped
        }
    }

    let name = &ast.ident;
    let (ig, tg, wc) = ast.generics.split_for_impl();

    let output = quote! {
        impl #ig DeezEntity for #name #tg #wc {
            fn to_av_map(&self) -> Result<HashMap<String, AttributeValue>, DeezError> {
                let mut av_map = HashMap::new();
                #inserts

                let index_keys = self.index_keys();
                for (_, index) in index_keys.iter() {
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
            fn from_av_map(av_map: HashMap<String, AttributeValue>) -> Result<#name, DeezError> {
                Ok(#name {
                    #reads
                    ..Default::default()
                })
            }
        }
    };
    output.into()
}
