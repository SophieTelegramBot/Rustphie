use syn::{LitStr, Token};
use syn::parse::{Parse, ParseBuffer};

pub enum CommandAttributes {
    Regex,
    Parser,
    Command
}

impl Parse for CommandAttributes {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let argument = input.parse::<syn::Ident>()?;
        match argument.to_string().as_str() {
            "regex" => Ok(Self::Regex),
            "command" => Ok(Self::Command),
            _ => Err(input.error("Unexpected argument")),
        }
    }
}

pub struct Attr {
    name: CommandAttributes,
    value: String,
}

impl Parse for Attr {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let name = input.parse::<CommandAttributes>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?.value();
        Ok(Self { name, value })
    }
}

impl Attr {
    pub fn name(&self) -> &CommandAttributes { &self.name}

    pub fn value(&self) -> String { self.value.clone() }
}

pub struct VecAttrs {
    pub data: Vec<Attr>
}

impl Parse for VecAttrs {
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
