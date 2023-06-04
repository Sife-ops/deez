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

#[proc_macro_derive(ToMap, attributes(deez))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        syn::Data::Struct(st) => st.fields,
        _ => panic!("todo"),
    };

    let mut inserts = quote! {};
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

        // rename
        // todo: sus af
        if let Some(first) = field.attrs.first() {
            match first.parse_args().unwrap() {
                Expr::Assign(ea) => match *ea.right {
                    Expr::Lit(el) => match el.lit {
                        syn::Lit::Str(el) => {
                            let t = el.token().to_string();
                            let mut c = t.chars();
                            c.next();
                            c.next_back();
                            field_name = c.as_str().to_string();
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            };
        }

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
            // DynamoDB attribute of type Number can store 126-bit integers (or
            // 127-bit unsigned integers, with serious caveats).
            // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
            "usize" | "isize" | "u8" | "i8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" => {
                inserts = quote! {
                    #inserts
                    m.insert(#field_name.to_string(), AttributeValue::N(self.#ident.to_string()));
                };
            }
            &_ => continue,
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
            // todo: from_av_map
        }
    };
    output.into()
}
