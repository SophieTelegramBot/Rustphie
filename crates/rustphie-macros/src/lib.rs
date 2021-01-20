extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, parse_macro_input, Result as SynResult, spanned::Spanned};

use crate::attr::{Attr, VecAttrs};
use crate::command::CommandData;
use crate::errors::BasicErrors;
use crate::fields_parse::{impl_parse_args_named, impl_parse_args_unit, impl_parse_args_unnamed};

mod attr;
mod command;
mod fields_parse;
mod parsers;
mod errors;

macro_rules! get_or_return {
    ($($some:tt)*) => {
        match $($some)* {
            Ok(elem) => elem,
            Err(e) => return e
        };
    }
}

#[proc_macro_derive(Command, attributes(command))]
pub fn derive_command(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let struct_data = get_or_return!(parse_struct(&input.data).map_err(|e| e.compile_error()));
    let attrs = get_or_return!(parse_attributes(&input.attrs).map_err(|e| e.to_compile_error().into()));
    let command = get_or_return!(CommandData::try_from(&attrs.as_slice()).map_err(|e| TokenStream::from(Error::new(input.span(), format!("{}", e)).to_compile_error())));

    let ident = &input.ident;
    let parser = match &struct_data.fields {
        Fields::Named(data) => {
            if !data.named.is_empty() && command.regex.is_none() {
                return TokenStream::from(quote! { compile_error!("Found empty regex field") });
            }
            impl_parse_args_named(data, &command)
        }
        Fields::Unnamed(data) => {
            if !data.unnamed.is_empty() && command.regex.is_none() {
                return TokenStream::from(quote! { compile_error!("Found empty regex field") });
            }
            impl_parse_args_unnamed(data, &command)
        }
        Fields::Unit => impl_parse_args_unit()
    };
    let fn_parse = impl_parse(command, parser);
    let res = TokenStream::from(
        quote! {
                impl rustphie_helpers::Command for #ident {
                    #fn_parse
                }
            }
    );
    // eprintln!("{}", res.clone());
    res
}

fn parse_attributes(input: &[syn::Attribute]) -> SynResult<Vec<Attr>> {
    let mut struct_attrs = Vec::new();
    for attr in input.iter() {
        struct_attrs.append(&mut attr.parse_args::<VecAttrs>()?.data)
    };
    Ok(struct_attrs)
}

fn parse_struct(input: &Data) -> Result<DataStruct, BasicErrors> {
    if let Data::Struct(data) = input {
        Ok(data.clone())
    } else {
        Err(BasicErrors::CanBeUsedOnlyInStruct)
    }
}

fn impl_parse(info: CommandData, parser: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
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
