use quote::ToTokens;
use regex::Regex;
use syn::{Field, Type};

pub struct IndexKeys {
    pub hash: IndexKey,
    pub range: IndexKey,
}

#[derive(Default)]
pub struct IndexKey {
    pub field: String,
    pub composite: Vec<Composite>,
}

pub struct Composite {
    pub position: usize,
    pub syn_field: Field,
}

#[derive(Default)]
pub struct FieldType {
    pub is_option: bool,
    pub is_vec: bool,
    pub inner_name: String,
}

impl FieldType {
    pub fn new(ty: &Type) -> FieldType {
        let mut s = FieldType::default();
        if let syn::Type::Path(p) = ty {
            let re_option = Regex::new(r"^Option (:: )?< (.*) >$").unwrap();
            let re_vec = Regex::new(r"^Vec (:: )?< (.*) >$").unwrap();
            match re_option.captures(&p.to_token_stream().to_string()) {
                Some(x) => {
                    s.is_option = true;
                    let option_inner = x.get(2).unwrap().as_str();
                    match re_vec.captures(&option_inner) {
                        Some(x) => {
                            s.is_vec = true;
                            s.inner_name = x.get(2).unwrap().as_str().to_string();
                        }
                        None => {
                            s.inner_name = option_inner.to_string();
                        }
                    }
                }
                None => match re_vec.captures(&p.to_token_stream().to_string()) {
                    Some(x) => {
                        s.is_vec = true;
                        s.inner_name = x.get(2).unwrap().as_str().to_string();
                    }
                    None => {
                        s.inner_name = p.to_token_stream().to_string();
                    }
                },
            }
        } else {
            panic!("could not parse field type as path");
        }

        s
    }

    // pub fn inner_ident(&self) -> proc_macro2::Ident {
    //     format_ident!("{}", self.inner_name)
    // }
}
