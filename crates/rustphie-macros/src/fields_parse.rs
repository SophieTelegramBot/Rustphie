use syn::{FieldsNamed, Type, PathArguments, FieldsUnnamed};
use quote::ToTokens;

pub fn impl_parse_args_named(
    data: &FieldsNamed,
    regex: Option<String>,
) -> proc_macro2::TokenStream {
    if data.named.is_empty() {
        quote::quote! {
            Self {}
        }
    } else {
        let get_arguments = create_parser(data.named.iter().map(|f| &f.ty), data.named.len(), regex.unwrap());
        let i = (0..data.named.len()).map(syn::Index::from);
        let name = data.named.iter().map(|f| f.ident.as_ref().unwrap());
        quote::quote! {
            #get_arguments
            Self { #(#name: arguments.#i),* }
        }
    }
}

pub fn impl_parse_args_unnamed(
    data: &FieldsUnnamed,
    regex: Option<String>,
) -> proc_macro2::TokenStream {
    if data.unnamed.is_empty() {
        quote::quote! {
            Self()
        }
    } else {
        let get_arguments = create_parser(data.unnamed.iter().map(|f| &f.ty), data.unnamed.len(), regex.unwrap());
        let i = (0..data.unnamed.len()).map(syn::Index::from);
        quote::quote! {
            #get_arguments
            Self(#(arguments.#i),*)
        }
    }
}

pub fn impl_parse_args_unit() -> proc_macro2::TokenStream {
    quote::quote! {
        Self
    }
}

fn create_parser<'a>(
    types: impl Iterator<Item = &'a Type>,
    count_args: usize,
    regex: String,
) -> proc_macro2::TokenStream {
    let i = 0..count_args;
    let types = extract_type(types);
    let inner2 = quote::quote! {
        // TODO: Remove this unwrap
        #(#types::from_str(captures_iter.next().unwrap().ok_or(ParseError::TooFewArguments {
            expected: #count_args,
            found: #i,
            message: format!("Expected but not found arg number {}", #i),
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
                    log::warn!("No captures found, regex: {} string: {}", #regex, s);
                    return Err(ParseError::NoCapturesFound(#regex.into(), s));
                },
                Some(val) => {
                    let actual_len = val.len() - 1;
                    match actual_len.cmp(&#count_args) {
                        std::cmp::Ordering::Less => {
                            return Err(ParseError::TooFewArguments {
                                expected: #count_args,
                                found: actual_len,
                                message: format!("Expected but not found arg number {}", actual_len),
                            });
                        },
                        std::cmp::Ordering::Greater => {
                            return Err(ParseError::TooManyArguments {
                                expected: #count_args,
                                found: actual_len,
                                message: format!("Excess arguments"),
                            });
                        },
                        _ => val,
                    }
                }
            };
            let mut captures_iter = captures.iter(); captures_iter.next(); // skip first capture, would be always whole-matched region
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

fn extract_type<'a>(types: impl Iterator<Item = &'a Type>) -> Vec<proc_macro2::TokenStream> {
    let mut extracted_ty = Vec::new();
    for ty in types {
        let __extracted = match ty {
            Type::Path(type_path) if type_path.qself.is_none() && type_path.path.leading_colon.is_none() => {
                // complexity! We only support single type param, if you really need more, DIY
                let mut type_params_iter = type_path.path.segments.iter();
                if type_params_iter.len() > 1 {
                    type_path.to_token_stream()
                } else {
                    // we are sure its not `None` value from above assertion
                    let path = type_params_iter.next().unwrap();
                    match &path.arguments {
                        PathArguments::AngleBracketed(type_) => {
                            let ident = &path.ident;
                            quote::quote! {
                                #ident::#type_
                            }
                        }
                        _ => type_path.to_token_stream()
                    }
                }
            }
            rest => rest.to_token_stream(),
        };
        extracted_ty.push(__extracted);
    }
    extracted_ty
}
