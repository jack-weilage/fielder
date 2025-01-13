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
//!         CounterField: 3..7 = !0;
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

    pub is_counter: bool,
}
