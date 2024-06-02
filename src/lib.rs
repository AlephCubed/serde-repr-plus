//! [![github]](https://github.com/dtolnay/serde-repr)&ensp;[![crates-io]](https://crates.io/crates/serde_repr)&ensp;[![docs-rs]](https://docs.rs/serde_repr)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! Derive `Serialize` and `Deserialize` that delegates to the underlying repr
//! of a C-like enum.
//!
//! # Examples
//!
//! ```
//! use serde_repr_plus::{Serialize_repr_clamp, Deserialize_repr_clamp};
//!
//! #[derive(Serialize_repr_clamp, Deserialize_repr_clamp, PartialEq, Debug)]
//! #[repr(u8)]
//! enum SmallPrime {
//!     Two = 2,
//!     Three = 3,
//!     Five = 5,
//!     Seven = 7,
//! }
//!
//! fn main() -> serde_json::Result<()> {
//!     let j = serde_json::to_string(&SmallPrime::Seven)?;
//!     assert_eq!(j, "7");
//!
//!     let p: SmallPrime = serde_json::from_str("2")?;
//!     assert_eq!(p, SmallPrime::Two);
//!
//!     Ok(())
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/serde_repr/0.1.19")]
#![allow(clippy::single_match_else)]

extern crate proc_macro;

mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parse::Input;

#[proc_macro_derive(Serialize_repr_clamp)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;
    let repr = input.repr;

    let match_variants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            #ident::#variant => #ident::#variant as #repr,
        }
    });

    TokenStream::from(quote! {
        #[allow(deprecated)]
        impl serde::Serialize for #ident {
            #[allow(clippy::use_self)]
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                let value: #repr = match *self {
                    #(#match_variants)*
                };
                serde::Serialize::serialize(&value, serializer)
            }
        }
    })
}

#[proc_macro_derive(Deserialize_repr_clamp, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;
    let repr = input.repr;
    let variants = input.variants.iter().map(|variant| &variant.ident);

    let declare_discriminants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            (#ident::#variant as #repr, #ident::#variant),
        }
    });

    TokenStream::from(quote! {
        #[allow(deprecated)]
        impl<'de> serde::Deserialize<'de> for #ident {
            #[allow(clippy::use_self)]
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let values = [
                    #(#declare_discriminants)*
                ];

                let mut output = values.first().unwrap();

                for value in &values {
                    if #repr::abs_diff(value.0, target) < output.0 {
                        output = value;
                    }

                    if output.0 == 0 {
                        break;
                    }
                }

                Ok(output.1.clone())
            }
        }
    })
}
