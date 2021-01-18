extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, Fields, parse_macro_input};

use crate::attr::{Attr, VecAttrs};
use crate::command::CommandData;
use crate::fields_parse::impl_parse_args_named;

mod attr;
mod command;
mod fields_parse;

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
    let struct_data = {
        if let syn::Data::Struct(data) = &input.data {
            data
        } else {
            let error = "Commands can be used only on structs";
            return TokenStream::from(quote! {compile_error!(#error)});
        }
    };
    let attrs = get_or_return!(parse_attributes(&input.attrs));
    let command = match CommandData::try_from(attrs.as_slice()) {
        Ok(val) => val,
        Err(e) => return TokenStream::from(quote! {compile_error!(#e)}),
    };
    if let Fields::Named(fields) = &struct_data.fields {
        let ident = &input.ident;
        if command.regex.is_none() {
            return TokenStream::from(quote! { compile_error!("Found empty regex field in named struct") });
        }
        let parser = impl_parse_args_named(fields, quote! { Self }, command.regex.clone().unwrap());
        let fn_parse = impl_parse(command, parser);

        // eprintln!("TOKENS: {}", fn_parse.clone());
        TokenStream::from(
            quote! {
                impl crate::utils::Command for #ident {
                    #fn_parse
                }
            }
        )
    } else {
        TokenStream::from(
            quote! { compile_error("Only named structs are supported right now!")}
        )
    }
}

fn parse_attributes(input: &[syn::Attribute]) -> Result<Vec<Attr>, TokenStream> {
    let mut struct_attrs = Vec::new();
    for attr in input.iter() {
        match attr.parse_args::<VecAttrs>() {
            Ok(mut attrs_) => struct_attrs.append(attrs_.data.as_mut()),
            Err(e) => return Err(TokenStream::from(e.to_compile_error()))
        }
    };
    Ok(struct_attrs)
}

fn impl_parse(info: CommandData, parser: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let command = info.get_command();

    quote! {
        fn parse<N: Into<String>>(s: &str, bot_username: N) -> Result<Self, crate::utils::ParseError> {
            use std::str::FromStr;
            use crate::utils::ParseError;

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