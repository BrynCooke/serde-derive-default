#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Ident};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("struct field did not have an ident")]
    MissingFieldIdent,
    #[error("enums cannot derive Default")]
    EnumNotSupported,
    #[error("unions cannot derive Default")]
    UnionNotSupported,
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
                let initializer_ident = Ident::new(initializer.as_str(), ident.span());
                quote! {
                    #ident: #initializer_ident()
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
            // We're looking for the pattern `default="foo"` and need to extract foo
            let regex = regex::Regex::new(r#"default\s*=\s*"([^"]*)""#).unwrap();
            let mut fields = Vec::new();
            for field in s.fields.iter() {
                let ident = field.ident.as_ref().ok_or(Error::MissingFieldIdent)?;
                let initializer = field
                    .attrs
                    .iter()
                    .find(|a| a.path.is_ident("serde"))
                    .map(|a| a.tts.to_string())
                    .and_then(|serde| {
                        if let Some(capture) = regex.captures(&serde) {
                            capture.get(1).map(|m| m.as_str().to_string())
                        } else {
                            None
                        }
                    });

                fields.push(Field {
                    ident: ident.clone(),
                    initializer,
                });
            }
            Ok(fields)
        }
        Data::Enum(_) => Err(Error::EnumNotSupported),
        Data::Union(_) => Err(Error::UnionNotSupported),
    }
}
