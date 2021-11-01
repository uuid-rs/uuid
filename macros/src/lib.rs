//! Implementation details for the `uuid!` macro.
//!
//! This crate is not meant to be used directly. Instead,
//! you can use the `macros` feature of `uuid`:
//!
//! ```toml
//! [dependencies.uuid]
//! features = ["macros"]
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use std::fmt;
use syn::spanned::Spanned;

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

#[path = "../../shared/error.rs"]
mod error;

#[path = "../../shared/parser.rs"]
mod parser;

#[proc_macro]
#[doc(hidden)]
pub fn parse_lit(input: TokenStream) -> TokenStream {
    build_uuid(input.clone()).unwrap_or_else(|e| {
        let msg = e.to_string();
        let ts = TokenStream2::from(input);
        let span = match e {
            Error::UuidParse(error::Error(
                error::ErrorKind::InvalidCharacter { index, .. },
            )) => {
                let mut s = proc_macro2::Literal::string("");
                s.set_span(ts.span());
                s.subspan(index + 1..=index + 1).unwrap()
            }
            Error::UuidParse(error::Error(
                error::ErrorKind::InvalidGroupLength { found, group, .. },
            )) => {
                let start =
                    parser::GROUP_LENS.iter().take(group).sum::<usize>()
                        + group
                        + 1;
                let mut s = proc_macro2::Literal::string("");
                s.set_span(ts.span());
                s.subspan(start..start + found).unwrap()
            }
            _ => ts.span(),
        };
        TokenStream::from(quote_spanned! {span=>
            compile_error!(#msg)
        })
    })
}

enum Error {
    NonStringLiteral,
    UuidParse(error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NonStringLiteral => f.write_str("expected string literal"),
            Error::UuidParse(ref e) => write!(f, "{}", e),
        }
    }
}

fn build_uuid(input: TokenStream) -> Result<TokenStream, Error> {
    let string = match syn::parse::<syn::Lit>(input) {
        Ok(syn::Lit::Str(literal)) => literal.value(),
        _ => return Err(Error::NonStringLiteral),
    };

    let bytes = parser::parse_str(&string).map_err(Error::UuidParse)?;

    let tokens = bytes
        .iter()
        .map(|byte| quote! { #byte, })
        .collect::<TokenStream2>();

    Ok(quote! {[#tokens]}.into())
}
