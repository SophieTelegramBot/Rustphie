use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Fields, Type};

use crate::callbackquery::CallbackDeriveData;
use crate::command::CommandData;

pub(crate) fn impl_parse(info: CommandData, parser: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let command = info.get_command();

    quote! {
        fn parse<N: Into<String>>(s: &str, bot_username: N) -> Result<Self, rustphie_helpers::ParseError> {
            use std::str::FromStr;
            use rustphie_helpers::ParseError;

            let mut words = s.splitn(2, ' ');
            let mut splited = words.next().expect("First item will be command").split('@');
            let command_raw = splited.next().expect("First item will be command");
            let bot = splited.next();
            match bot {
                Some(name) if name == bot_username.into() => {},
                None => {},
                Some(n) => return Err(ParseError::WrongBotName(n.to_string())),
            };
            let mut args = words.next().unwrap_or("").to_string();
            match command_raw {
                #command => Ok({ #parser }),
                _ => Err(ParseError::UnknownCommand(command_raw.to_string())),
            }
        }
    }
}

pub(crate) fn impl_parse_callbackquery(info: CallbackDeriveData, parser: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let prefix = info.prefix + "_";
    quote! {
        fn parse(data: String) -> Result<Self, rustphie_helpers::ParseError> {
            use std::str::FromStr;
            use rustphie_helpers::ParseError;

            let args = if data.starts_with(#prefix) {
                data.trim_start_matches(#prefix).to_string()
            } else {
                return Err(ParseError::UnknownCommand(#prefix.into()));
            };
            Ok({ #parser })
        }
    }
}

pub(crate) fn impl_callbackquery_derive_new_fn(fields: &Fields, data: CallbackDeriveData, ident: proc_macro2::Ident) -> proc_macro2::TokenStream {
    let prefix = data.prefix;
    let inner_function = match fields {
        Fields::Named(data) => {
            let ident = data.named.iter().map(|f| f.ident.clone().unwrap() /* we are sure its not tuple struct*/).collect::<Vec<Ident>>();
            let types = data.named.iter().map(|f| f.ty.clone()).collect::<Vec<Type>>();
            // let test: Vec<String> = Vec::from_iter(ident);
            // let test2 = Vec::from_iter(data.named.iter().map(|f| f.ty).collect());
            bare_func_gen(ident, types, prefix)
        }
        Fields::Unnamed(data) => {
            let types = data.unnamed.iter().map(|f| f.ty.clone()).collect::<Vec<Type>>();
            let ident = (0usize..data.unnamed.len()).map(|f| { let var = format!("__var{}", f); Ident::new(var.as_str(), Span::call_site()) }).collect::<Vec<Ident>>();
            bare_func_gen(ident, types, prefix)
        }
        Fields::Unit => quote! { fn new() -> String { #prefix.into() } }
    };
    quote! {
        impl #ident {
            #inner_function
        }
    }
}

pub(crate) fn bare_func_gen(ident: Vec<Ident>, types: Vec<Type>, prefix: String) -> proc_macro2::TokenStream {
    quote! {
        fn new(#(#ident: #types),*) -> String {
            let data = vec![#prefix.to_string(), #(#ident.to_string()),*];
            data.join("_".into())
        }
    }
}

pub(crate) fn command_trait_impl_gen(ident: Ident, func: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        impl rustphie_helpers::Command for #ident {
            #func
        }
    }
}
