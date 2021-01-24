use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, parse_macro_input, Result as SynResult, spanned::Spanned};

use crate::attr::{Attr, CallbackQueryAttributes, CommandAttributes, VecAttrs};
use crate::callbackquery::CallbackDeriveData;
use crate::command::CommandData;
use crate::errors::BasicErrors;
use crate::fields_parse::{impl_parse_args_named, impl_parse_args_unit, impl_parse_args_unnamed};
use crate::gen::{command_trait_impl_gen, impl_callbackquery_derive_new_fn, impl_parse_callbackquery, impl_parse};
use crate::parsers::ParserType;

mod attr;
mod command;
mod fields_parse;
mod parsers;
mod errors;
mod callbackquery;
mod gen;

macro_rules! get_or_return {
    ($($some:tt)*) => {
        match $($some)* {
            Ok(elem) => elem,
            Err(e) => return e
        };
    }
}

#[proc_macro_derive(CallbackQuery, attributes(callback_query))]
pub fn derive_callbackquery(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let struct_data = get_or_return!(parse_struct(&input.data).map_err(|e| e.compile_error()));
    let attrs = get_or_return!(parse_attributes_callbackquery(&input.attrs).map_err(|e| TokenStream::from(Error::new(input.span(), e).to_compile_error())));
    let cb_data = get_or_return!(CallbackDeriveData::try_from(attrs.as_slice()).map_err(|e| TokenStream::from(Error::new(input.span(), e).to_compile_error())));

    let ident = &input.ident;
    let parser = get_or_return!(generate_field_parsers(&struct_data.fields, cb_data.parser.clone()).map_err(|e| TokenStream::from(Error::new(input.span(), e).to_compile_error())));
    let fn_parser = impl_parse_callbackquery(cb_data.clone(), parser);
    let new_fn = impl_callbackquery_derive_new_fn(&struct_data.fields, cb_data, ident.clone());
    let res = TokenStream::from(
        quote! {
            #new_fn
            impl rustphie_helpers::CallbackQuery for #ident {
                #fn_parser
            }
        }
    );
    // eprintln!("{}", res);
    res
}

#[proc_macro_derive(Command, attributes(command))]
pub fn derive_command(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let struct_data = get_or_return!(parse_struct(&input.data).map_err(|e| e.compile_error()));
    let attrs = get_or_return!(parse_attributes_command(&input.attrs).map_err(|e| e.to_compile_error().into()));
    let command = get_or_return!(CommandData::try_from(&attrs.as_slice()).map_err(|e| TokenStream::from(Error::new(input.span(), e).to_compile_error())));

    let ident = input.ident.clone();
    let parser = get_or_return!(generate_field_parsers(&struct_data.fields, command.parser_type.clone()).map_err(|e| TokenStream::from(Error::new(input.span(), e).to_compile_error())));
    let fn_parse = impl_parse(command, parser);
    let res = TokenStream::from(command_trait_impl_gen(ident, fn_parse));
    // eprintln!("{}", res);
    res
}

fn generate_field_parsers(field_type: &Fields, parser_type: Option<ParserType>) -> Result<proc_macro2::TokenStream, BasicErrors> {
    match field_type {
        Fields::Named(field_data) => {
            if parser_type.is_none() { return Err(BasicErrors::FailedToExtractParserType); }
            Ok(impl_parse_args_named(field_data, parser_type.unwrap()))
        },
        Fields::Unnamed(field_data) => {
            if parser_type.is_none() { return Err(BasicErrors::FailedToExtractParserType); }
            Ok(impl_parse_args_unnamed(field_data, parser_type.unwrap()))
        },
        Fields::Unit => Ok(impl_parse_args_unit()),
    }
}

fn parse_attributes_command(input: &[syn::Attribute]) -> SynResult<Vec<Attr<CommandAttributes>>> {
    let mut struct_attrs = Vec::new();
    for attr in input.iter() {
        struct_attrs.append(&mut attr.parse_args::<VecAttrs<CommandAttributes>>()?.data)
    };
    Ok(struct_attrs)
}

fn parse_attributes_callbackquery(input: &[syn::Attribute]) -> SynResult<Vec<Attr<CallbackQueryAttributes>>> {
    let mut attributes = Vec::new();
    for attr in input.iter() {
        attributes.append(&mut attr.parse_args::<VecAttrs<CallbackQueryAttributes>>()?.data)
    }
    Ok(attributes)
}

fn parse_struct(input: &Data) -> Result<DataStruct, BasicErrors> {
    if let Data::Struct(data) = input {
        Ok(data.clone())
    } else {
        Err(BasicErrors::CanBeUsedOnlyInStruct)
    }
}
