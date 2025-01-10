//! # `fielder`
//!
//! `fielder` provides a single macro ([`bitfield`]) which (like
//! [`bitflags`](https://docs.rs/bitflags/latest/bitflags)) allows for defining complex structures
//! packed into only a few bytes. These structures differ from those generated by similar crates by
//! allowing significantly more complex fields to be defined. These fields can span multiple bits,
//! can act as a simple counter, and behave like you'd expect. Everything is `no_std` and `no_alloc`,
//! making `fielder` a perfect fit for embedded protocols.
//!
//! # Example
//!
//! ```edition2021
//! use fielder::bitfield;
//!
//! bitfield! {
//!     // This bitfield uses a `u8` under the hood.
//!     struct ComplexField: u8 {
//!         // Fields only spanning a single bit will act like flags.
//!         FirstFlag: 0;
//!         // Fields can span multiple bits.
//!         SomeField: 1..2 = 0;
//!         SecondField: 1..2 = 1;
//!         ThirdField: 1..2 = 2;
//!         FourthField: 1..2 = 3;
//!
//!         // This field will be used in combination with `get_literal` to retrieve the literal
//!         // value of the contained bits.
//!         CounterField: 3..7 = 0;
//!     }
//! }
//!
//! // Fields are constructed from an integer via `from_bits`.
//! let field = ComplexField::from_bits(0b01010_11_0);
//!
//! // The first bit (FirstFlag) isn't set.
//! assert!(!field.contains(ComplexField::FirstFlag));
//! // The second and third bits are set, so FourthField is set.
//! assert!(field.contains(ComplexField::FourthField));
//! // Importantly, even though its bit is set, SecondField isn't set.
//! assert!(!field.contains(ComplexField::SecondField));
//! // The current literal value of CounterField can be read.
//! assert_eq!(field.get_literal(ComplexField::CounterField), 0b01010);
//! ```
#![no_std]

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
///         // The value of a field will be masked with the bits it should cover, so using `!0`
///         // works fine to create a full value.
///         FieldFourFull: 4..5 = !0;
///     }
/// }
/// ```
pub use fielder_proc::bitfield;

/// A struct defining the parts of a field. This struct is automatically constructed via the
/// [`bitfield`](crate::bitfield) macro.
#[derive(Debug, Clone, Copy)]
pub struct Field<Bits> {
    pub name: &'static str,

    pub start_bit: Bits,
    pub end_bit: Bits,

    pub mask: Bits,
    pub value: Bits,
}
