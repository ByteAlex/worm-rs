use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Field, parse_macro_input};
use std::str::FromStr;

fn option_type(field: &Field) -> Option<Ident> {
    let mut iter = field.ty.to_token_stream().into_iter();
    if let Some(token) = iter.next() {
        if let TokenTree::Ident(ident) = token {
            if ident.to_string().eq("Option") {
                iter.next(); // skip opening punctuation `<`
                let token = iter.next().expect("Option requires next token");
                if let TokenTree::Ident(inner) = token {
                    return Some(inner);
                } else {
                    println!("Matched {:?}", token);
                    panic!("Unexpected token!");
                }
            }
        }
    }
    None
}

#[proc_macro_attribute]
pub fn rest_object(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rest(item)
}

#[proc_macro]
pub fn rest(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let struct_data = parse_macro_input!(item as syn::ItemStruct);

    let name = &struct_data.ident;
    let rest_name = format!("Rest{}", name);
    let rest_name = TokenStream::from_str(rest_name.as_str()).unwrap();

    let mut fields_list = "".to_owned();
    let mut field_apply = "".to_owned();
    let mut field_clone = "".to_owned();

    for field in &struct_data.fields {
        let ident = field.ident.clone().expect("Fields need identifiers");
        if let Some(ty) = option_type(field) {
            fields_list.push_str(format!(
                r##"#[serde(default, deserialize_with = "worm::nullable::deserialize_optional_nullable")]
                pub {}: Option<worm::nullable::Nullable<{}>>,"##,
                ident,
                ty
            ).as_str());
        } else {
            fields_list.push_str(format!(
                "pub {}: Option<{}>,",
                ident,
                field.ty.to_token_stream()
            ).as_str());
        }

        field_apply.push_str(format!(
            "worm::apply_some!(self, {} = from.{});\n",
            ident, ident
        ).as_str());

        field_clone.push_str(format!(
            "{}: Some(self.{}.clone().into()),\n",
            ident, ident
        ).as_str());
    }

    let fields_list = TokenStream::from_str(fields_list.as_str()).unwrap();
    let field_apply = TokenStream::from_str(field_apply.as_str()).unwrap();
    let field_clone = TokenStream::from_str(field_clone.as_str()).unwrap();

    let result = quote! {

        #struct_data

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct #rest_name {
            #fields_list
        }

        impl worm::Apply<#rest_name> for #name {
            fn apply(&mut self, from: #rest_name) {
                #field_apply
            }
        }

        impl worm::CloneInto<#rest_name> for #name {
            fn clone_into(&self) -> #rest_name {
                #rest_name {
                    #field_clone
                }
            }
        }
    };
    return result.into();
}