use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

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
        _ => panic!("ast.data not a `Struct`"),
    };

    let mut reads = quote! {};
    // let mut partial_fields = quote! {};
    // let mut partial_inserts = quote! {};

    for field in fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_type = match &field.ty {
            syn::Type::Path(b) => b.clone(),
            _ => panic!("field.ty not a `TypePath`"),
        };

        match field_type.clone().into_token_stream().to_string().as_ref() {
            "String" => {
                reads = quote! {
                    #reads
                    #field_ident: m
                        .get(#field_name)
                        .ok_or(DeezError::UnknownAttributeValueKey(#field_name.to_string()))?
                        .as_s()?
                        .clone(),
                };
            }

            "bool" => {
                reads = quote! {
                    #reads
                    #field_ident: m
                        .get(#field_name)
                        .ok_or(DeezError::UnknownAttributeValueKey(#field_name.to_string()))?
                        .as_bool()?
                        .clone(),
                };
            }

            // DynamoDB attribute of type Number can store 126-bit integers (or
            // 127-bit unsigned integers, with serious caveats).
            // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
            "usize" | "isize" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" => {
                reads = quote! {
                    #reads
                    #field_ident: m
                        .get(#field_name)
                        .ok_or(DeezError::UnknownAttributeValueKey(#field_name.to_string()))?
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

        // partial_fields = quote! {
        //     #partial_fields
        //     pub #field_ident: Option<#field_type>,
        // };
    }

    let name = &ast.ident;
    // let partial_name = format_ident!("{}Partial", name);

    let (ig, tg, wc) = ast.generics.split_for_impl();
    let output = quote! {
        impl #ig DeezEntity for #name #tg #wc {
            fn from_av_map(m: &HashMap<String, AttributeValue>) -> Result<#name, DeezError> {
                Ok(#name {
                    #reads
                    // ..Default::default()
                })
            }
        }
    };

    output.into()
}
