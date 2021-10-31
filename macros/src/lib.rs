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

/// Parse [`Uuid`][uuid::Uuid]s from string literals at compile time.
/// ## Usage
/// This macro transforms the string literal representation of a [`Uuid`][uuid::Uuid] into the bytes representation,
/// raising a compilation error if it cannot properly be parsed.
///
/// ## Examples
/// Setting a global constant:
/// ```
/// # use uuid::{uuid, Uuid};
/// pub const SCHEMA_ATTR_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000000");
/// pub const SCHEMA_ATTR_UUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000001");
/// pub const SCHEMA_ATTR_NAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000002");
/// ```
/// Defining a local variable:
/// ```
/// # use uuid::{uuid, Uuid};
/// let uuid: Uuid = uuid!("urn:uuid:F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4");
/// ```
/// ## Compilation Failures
/// Invalid UUIDs are rejected:
/// ```ignore
/// # use uuid::{uuid, Uuid};
/// let uuid: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
/// ```
/// Provides the following compilation error:
/// ```txt
/// error: invalid character: expected an optional prefix of `urn:uuid:` followed by 0123456789abcdefABCDEF-, found Z at 9
///     |
///     |     let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
///     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
/// Tokens that aren't string literals are also rejected:
/// ```ignore
/// # use uuid::{uuid, Uuid};
/// let uuid_str: &str = "550e8400e29b41d4a716446655440000";
/// let uuid: Uuid = uuid!(uuid_str);
/// ```
/// Provides the following compilation error:
/// ```txt
/// error: expected string literal
///   |
///   |     let uuid: Uuid = uuid!(uuid_str);
///   |                            ^^^^^^^^
/// ```
///
/// [uuid::Uuid]: https://docs.rs/uuid/*/uuid/struct.Uuid.html
#[proc_macro]
pub fn uuid(input: TokenStream) -> TokenStream {
    build_uuid(input.clone()).unwrap_or_else(|e| {
        let msg = e.to_string();
        TokenStream::from(quote_spanned! {
            TokenStream2::from(input).span() =>
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
    let uuid_str = match syn::parse::<syn::Lit>(input) {
        Ok(syn::Lit::Str(ref literal)) => literal.value(),
        _ => return Err(Error::NonStringLiteral),
    };

    let bytes = parser::parse_str(&uuid_str).map_err(Error::UuidParse)?;

    let tokens = bytes
        .iter()
        .map(|byte| quote! { #byte, })
        .collect::<TokenStream2>();

    Ok(quote! {::uuid::Uuid::from_bytes([#tokens])}.into())
}
