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

#[proc_macro_derive(DeezMaps, attributes(deez))]
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

        let field_name_original = ident.to_string();

        // rename
        // todo: sus af
        // todo: skips
        let mut field_name_renamed: Option<String> = None;
        if let Some(first) = field.attrs.first() {
            if let Ok(ex) = first.parse_args::<Expr>() {
                if let Expr::Assign(ea) = ex {
                    if let Expr::Lit(el) = *ea.right {
                        if let syn::Lit::Str(ls) = el.lit {
                            let t = ls.token().to_string();
                            let mut c = t.chars();
                            c.next();
                            c.next_back();
                            field_name_renamed = Some(c.as_str().to_string());
                        }
                    }
                }
            }
        }

        let field_name_dynamo = match field_name_renamed {
            Some(n) => n,
            None => field_name_original,
        };

        // todo: use type enum instead of matching strings
        match field_type.as_ref() {
            "String" => {
                inserts = quote! {
                    #inserts
                    m.insert(#field_name_dynamo.to_string(), AttributeValue::S(self.#ident.to_string()));
                };
                reads = quote! {
                    #reads
                    #ident: m.get(#field_name_dynamo).unwrap().as_s().unwrap().clone(),
                }
            }
            "bool" => {
                inserts = quote! {
                    #inserts
                    m.insert(#field_name_dynamo.to_string(), AttributeValue::Bool(self.#ident));
                };
                reads = quote! {
                    #reads
                    #ident: m.get(#field_name_dynamo).unwrap().as_bool().unwrap().clone(),
                }
            }
            // DynamoDB attribute of type Number can store 126-bit integers (or
            // 127-bit unsigned integers, with serious caveats).
            // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
            "usize" | "isize" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" => {
                inserts = quote! {
                    #inserts
                    m.insert(#field_name_dynamo.to_string(), AttributeValue::N(self.#ident.to_string()));
                };
                match field_type.as_ref() {
                    "usize" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<usize>().unwrap(),
                        }
                    }
                    "isize" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<isize>().unwrap(),
                        }
                    }
                    "u8" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<u8>().unwrap(),
                        }
                    }
                    "i8" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<i8>().unwrap(),
                        }
                    }
                    "u16" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<u16>().unwrap(),
                        }
                    }
                    "i16" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<i16>().unwrap(),
                        }
                    }
                    "u32" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<u32>().unwrap(),
                        }
                    }
                    "i32" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<i32>().unwrap(),
                        }
                    }
                    "u64" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<u64>().unwrap(),
                        }
                    }
                    "i64" => {
                        reads = quote! {
                            #reads
                            #ident: m.get(#field_name_dynamo).unwrap().as_n().unwrap().clone().parse::<i64>().unwrap(),
                        }
                    }
                    &_ => continue, // todo: panic?
                }
            }
            &_ => continue,
        }
    }

    let name = &ast.ident;
    let (ig, tg, wc) = ast.generics.split_for_impl();

    let output = quote! {
        impl #ig DeezMaps for #name #tg #wc {
            fn to_av_map(&self) -> HashMap<String, AttributeValue> {
                let mut m = HashMap::new();
                #inserts
                m
            }
            // todo: return Result?
            fn from_av_map(m: HashMap<String, AttributeValue>) -> Self {
                #name {
                    #reads
                    ..Default::default() // todo: remove default?
                }
            }
        }
    };
    output.into()
}
