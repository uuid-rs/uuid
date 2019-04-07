extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Expr, ExprArray, ExprLit,
    IntSuffix, Lit, LitInt, LitStr,
};
use uuid_parser::parse_str;

#[proc_macro_hack]
pub fn uuid(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as LitStr);

    // FIXME: Handle error better
    let bytes = parse_str(&s.value()).unwrap();

    let mut values = Punctuated::new();
    
    for byte in &bytes {
        values.push(Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new(
                *byte as u64,
                IntSuffix::U8,
                Span::call_site(),
            )),
        }));
    }

    let expr = ExprArray {
        attrs: vec![],
        bracket_token: syn::token::Bracket::default(),
        elems: values,
    };

    let tokens = quote! {
        uuid::Uuid::from_bytes(#expr)
    };

    tokens.into()
}
