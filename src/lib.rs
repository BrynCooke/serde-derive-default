#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Data, DeriveInput, Expr, Ident, Lit, Meta};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("struct field did not have an ident")]
    MissingFieldIdent,
    #[error("enums cannot derive Default")]
    EnumNotSupported,
    #[error("unions cannot derive Default")]
    UnionNotSupported,
    #[error("failed to parse serde attribute arguments {0}")]
    ParseArgs(String),
}

#[proc_macro_derive(Default)]
pub fn derive_default(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree

    let input = parse_macro_input!(input as DeriveInput);

    match process_input(&input) {
        Ok(derived_token_stream) => derived_token_stream,
        Err(err) => {
            TokenStream::from(syn::Error::new(input.span(), err.to_string()).to_compile_error())
        }
    }
}

fn process_input(input: &DeriveInput) -> Result<TokenStream, Error> {
    let fields = process_data(&input.data)?;
    let ident = &input.ident;

    let fields = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            if let Some(initializer) = &f.initializer {
                let initializer = Ident::new(initializer, ident.span());
                quote! {
                    #ident: #initializer()
                }
            } else {
                quote! {
                    #ident: core::default::Default::default()
                }
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {

        impl Default for #ident {
            fn default() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }

    };

    Ok(expanded.into())
}

struct Field {
    ident: Ident,
    initializer: Option<String>,
}

fn process_data(data: &Data) -> Result<Vec<Field>, Error> {
    match data {
        Data::Struct(s) => {
            let mut fields = Vec::new();
            for field in s.fields.iter() {
                let ident = field.ident.as_ref().ok_or(Error::MissingFieldIdent)?;
                let serde = field.attrs.iter().find(|a| a.path().is_ident("serde"));
                if let Some(serde) = serde {
                    let meta_list = serde
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                        .map_err(|e| Error::ParseArgs(e.to_string()))?;

                    fields.push(Field {
                        ident: ident.clone(),
                        initializer: find_default(meta_list),
                    });
                } else {
                    fields.push(Field {
                        ident: ident.clone(),
                        initializer: None,
                    });
                }
            }
            Ok(fields)
        }
        Data::Enum(_) => Err(Error::EnumNotSupported),
        Data::Union(_) => Err(Error::UnionNotSupported),
    }
}

fn find_default(meta_list: Punctuated<Meta, Comma>) -> Option<String> {
    for meta in meta_list {
        if let Meta::NameValue(name_value) = meta {
            if name_value.path.is_ident("default") {
                if let Expr::Lit(val) = &name_value.value {
                    if let Lit::Str(val) = &val.lit {
                        return Some(val.value().clone());
                    }
                }
            }
        }
    }
    None
}
