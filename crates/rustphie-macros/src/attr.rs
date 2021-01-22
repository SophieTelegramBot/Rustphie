use syn::{LitStr, Token};
use syn::parse::{Parse, ParseBuffer};

pub enum CommandAttributes {
    Parser,
    Command,
    Data,
}

impl Parse for CommandAttributes {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let argument = input.parse::<syn::Ident>()?;
        match argument.to_string().as_str() {
            "command" => Ok(Self::Command),
            "parser" => Ok(Self::Parser),
            "separator" | "sep" | "delim" | "regex" => Ok(Self::Data),
            _ => Err(input.error("Unexpected argument")),
        }
    }
}

pub enum CallbackQueryAttributes {
    Prefix,
    Delimiter,
}

impl Parse for CallbackQueryAttributes {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let argument = input.parse::<syn::Ident>()?;
        match argument.to_string().as_str() {
            "prefix" => Ok(Self::Prefix),
            "delim" | "separator" => Ok(Self::Delimiter),
            _ => Err(input.error("Unexpected argument"))
        }
    }
}

pub struct Attr<T> {
    name: T,
    value: String,
}

impl<T: Parse> Parse for Attr<T> {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let name = input.parse::<T>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?.value();
        Ok(Self { name, value })
    }
}

impl<T> Attr<T> {
    pub fn name(&self) -> &T { &self.name}

    pub fn value(&self) -> String { self.value.clone() }
}

pub struct VecAttrs<T> {
    pub data: Vec<Attr<T>>
}

impl<T: Parse> Parse for VecAttrs<T> {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let mut data = vec![];
        while !input.is_empty() {
            data.push(input.parse()?);
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { data })
    }
}
