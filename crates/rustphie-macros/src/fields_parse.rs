use syn::{FieldsNamed, Type};
use quote::ToTokens;

pub fn impl_parse_args_named(
    data: &FieldsNamed,
    variant: impl ToTokens,
    regex: String,
) -> proc_macro2::TokenStream {
    let get_arguments = create_parser(data.named.iter().map(|f| &f.ty), data.named.len(), regex);
    let i = (0..data.named.len()).map(syn::Index::from);
    let name = data.named.iter().map(|f| f.ident.as_ref().unwrap());
    quote::quote! {
        #get_arguments
        #variant { #(#name: arguments.#i),* }
    }
}

fn create_parser<'a>(
    types: impl Iterator<Item = &'a Type>,
    count_args: usize,
    regex: String,
) -> proc_macro2::TokenStream {
    let inner2 = quote::quote! {
        // TODO: Remove this unwrap
        #(#types::from_str(captures_iter.next().unwrap().ok_or(ParseError::TooFewArguments {
            expected: #count_args,
            found: 0,
            // TODO: replace 0
            message: format!("Expected but not found arg number {}", 0),
        })?.as_str()).map_err(|e|/*ParseError::IncorrectFormat({ let e: Box<dyn std::error::Error + Send + Sync + 'static> = e.into(); e })*/ {
            let e: Box<dyn std::error::Error + Send + Sync + 'static> = e.into();
            ParseError::IncorrectFormat(e)
        })?,)*

    };
    let function_to_parse = quote::quote! {
        (|s: String| {
            lazy_static::lazy_static! {
                static ref REGEX: regex::Regex = regex::Regex::new(#regex).unwrap();
            };
            let captures = match REGEX.captures(s.as_str()) {
                None => {
                    // TODO: show regex in log
                    log::warn!("No captures found, regex: {{todo}} string: {}", s);
                    return Err(ParseError::NoCapturesFound(#regex.into(), s));
                },
                Some(val) => {
                    let actual_len = val.len() - 1;
                    match actual_len.cmp(&#count_args) {
                        std::cmp::Ordering::Less => {
                            return Err(ParseError::TooFewArguments {
                                expected: #count_args,
                                found: 0, // not implemented, todo
                                message: format!("Expected but not found arg number {}", 0 + 1), // not implemented, todo
                            });
                        },
                        std::cmp::Ordering::Greater => {
                            return Err(ParseError::TooManyArguments {
                                expected: #count_args,
                                found: #count_args + 1,
                                message: format!("Excess arguments"),
                            });
                        },
                        _ => val,
                    }
                }
            };
            let mut captures_iter = captures.iter();
            let res = (#inner2);
            match splited.next() {
                Some(d) => Err(ParseError::TooManyArguments {
                    expected: #count_args,
                    found: #count_args + 1,
                    message: format!("Excess argument: {}", d),
                }),
                None => Ok(res)
            }
        })
    };
    quote::quote! {
        let arguments = #function_to_parse(args)?;
    }
}
