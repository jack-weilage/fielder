//! # `fielder-proc`
//!
//! This crate contains the proc-macros used by
//! [`fielder`](https://docs.rs/fielder/latest/fielder).

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctest;

use proc_macro::TokenStream;
use syn::parse_macro_input;

extern crate proc_macro;

mod bitfield;

/// Generate a complex bitfield.
///
/// Definitions are similar to [`bitflags`](https://docs.rs/bitflags/latest/bitflags), with some
/// important differences.
///
/// # Example
///
/// ```edition2021
/// use fielder::bitfield;
///
/// bitfield! {
///     // The struct definition can include a visiblity modifier (`pub(crate)`) and must include
///     // a type (`u8`).
///     pub(crate) struct Field: u8 {
///         // Fields can either span a single bit...
///         FlagOne: 0;
///         // ...or multiple.
///         FieldTwo: 1..2;
///         // Fields can have a value assigned...
///         FieldThree: 2..3 = 0b11;
///         // ...or use a default. Fields covering multiple bits will default to `0`, while
///         // fields covering a single bit will default to `1`.
///         FieldFourEmpty: 4..5;
///         // Fields can also overlap, allowing for an enum-like interface.
///         FieldFourPartial: 4..5 = 0b10;
///         // Fields act as "counters" when the value is "!0", allowing for the exact bit value
///         // to be get/set
///         Rest: 6..7 = !0;
///     }
/// }
/// ```
#[proc_macro]
pub fn bitfield(input: TokenStream) -> TokenStream {
    bitfield::to_tokens(parse_macro_input!(input as bitfield::Bitfield))
}
