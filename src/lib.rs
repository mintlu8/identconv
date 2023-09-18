//! Convinient macros to turn [`Idents`](proc_macro::Ident) into idents or strings by case converting.

use proc_macro::{TokenStream, TokenTree, Literal, Ident};
use convert_case::{Casing, Case::*};
use litrs::StringLit;

fn strify(s: TokenStream, f: impl Fn(&str) -> String) -> TokenStream{
    let mut iter = s.into_iter();
    let next = iter.next();
    if iter.next().is_some(){
        panic!("Too many items, expect an ident or a string literal.")
    }
    match next {
        Some(TokenTree::Ident(ident)) => {
            let s = f(&ident.to_string());
            TokenTree::Literal(Literal::string(s.as_str())).into()
        },
        Some(TokenTree::Literal(lit)) => {
            let lit = StringLit::try_from(lit).expect("Expect an ident or a string literal");
            let s = f(lit.value());
            TokenTree::Literal(Literal::string(s.as_str())).into()
        }
        _ => panic!("Expect an ident or a string literal"),
    }
}

fn ident_conv(s: TokenStream, f: impl Fn(&str) -> String) -> TokenStream{
    let mut iter = s.into_iter();
    let next = iter.next();
    if iter.next().is_some(){
        panic!("Too many items, expect an ident or a string literal.")
    }
    match next {
        Some(TokenTree::Ident(ident)) => {
            let span = ident.span().clone();
            let s = f(&ident.to_string());
            TokenTree::Ident(Ident::new(&s, span)).into()
        },
        Some(TokenTree::Literal(lit)) => {
            let span = lit.span().clone();
            let lit = StringLit::try_from(lit).expect("Expect an ident or a string literal");
            let s = f(lit.value());
            TokenTree::Ident(Ident::new(&s, span)).into()
        }
        _ => panic!("Expect an ident or a string literal"),
    }
}


/// Convert an [`Ident`](proc_macro::Ident) to a flatlowercase [`&'static str`](str)
#[proc_macro]
pub fn lower_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(Flat))
}

/// Convert an [`Ident`](proc_macro::Ident) to a FLATUPPERCASE [`&'static str`](str)
#[proc_macro]
pub fn upper_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(UpperFlat))
}

/// Convert an [`Ident`](proc_macro::Ident) to a snake_case [`&'static str`](str)
#[proc_macro]
pub fn snake_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(Snake))
}


/// Convert an [`Ident`](proc_macro::Ident) to a UPPER_SNAKE_CASE [`&'static str`](str)
#[proc_macro]
pub fn usnake_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(Upper))
}

/// Convert an [`Ident`](proc_macro::Ident) to a camelCase [`&'static str`](str)
#[proc_macro]
pub fn camel_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(Camel))
}

/// Convert an [`Ident`](proc_macro::Ident) to a PascalCase [`&'static str`](str)
#[proc_macro]
pub fn pascal_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(Pascal))
}

/// Convert an [`Ident`](proc_macro::Ident) to a kebab-case [`&'static str`](str)
#[proc_macro]
pub fn kebab_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(Kebab))
}

/// Convert an [`Ident`](proc_macro::Ident) to a UPPER-KEBAB-CASE [`&'static str`](str)
#[proc_macro]
pub fn ukebab_strify(s: TokenStream) -> TokenStream{
    strify(s, |ident| ident.to_case(UpperKebab))
}

/// Convert an [`Ident`](proc_macro::Ident) to a flatlowercase [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn lower(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(Flat))
}

/// Convert an [`Ident`](proc_macro::Ident) to a FLATUPPERCASE [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn upper(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(UpperFlat))
}

/// Convert an [`Ident`](proc_macro::Ident) to a snake_case [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn snake(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(Snake))
}

/// Convert an [`Ident`](proc_macro::Ident) to a UPPER_SNAKE_CASE [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn usnake(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(Upper))
}

/// Convert an [`Ident`](proc_macro::Ident) to a camelCase [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn camel(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(Camel))
}

/// Convert an [`Ident`](proc_macro::Ident) to a PascalCase [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn pascal(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(Pascal))
}

/// Convert an [`Ident`](proc_macro::Ident) to a kebab-case [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn kebab(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(Kebab))
}

/// Convert an [`Ident`](proc_macro::Ident) to a UPPER-KEBAB-CASE [`Ident`](proc_macro::Ident)
#[proc_macro]
pub fn ukebab(s: TokenStream) -> TokenStream{
    ident_conv(s, |ident| ident.to_case(UpperKebab))
}
