use crate::Field;

/// A trait implemented on types which can be used as storage for bitfields. See [`Bitfield`]'s
/// documentation for an example of usage.
pub trait Bits: 'static {
    const BITS: u32;
}

/// A trait representing a bitfield. This trait is meant for use in programatic scenarios, like
/// retrieving the inner type of a bitfield.
///
/// # Example
///
/// A slightly involved example of using the [`Bitfield`] and [`Bits`] traits to implement a trait
/// named `FromBytes` which constructs a type from a byte array.
///
/// ```
/// use fielder::{bitfield, Bitfield};
///
/// bitfield! {
///     struct SomeField: u16 {
///         // ... some flags in here
///     }
/// };
///
/// trait FromBytes {
///     fn from_bytes(bytes: &[u8], offset: &mut usize) -> Self;
/// }
///
/// impl FromBytes for SomeField {
///     fn from_bytes(bytes: &[u8], offset: &mut usize) -> Self {
///         type Bits = <SomeField as Bitfield>::Bits;
///         const BYTE_COUNT: usize = (Bits::BITS / 8) as usize;
///
///         let le_bytes: [u8; BYTE_COUNT] = bytes[*offset..*offset + BYTE_COUNT]
///             .try_into().unwrap();
///         *offset += BYTE_COUNT;
///
///         Self::from_bits(Bits::from_le_bytes(le_bytes))
///     }
/// }
///
/// ```
pub trait Bitfield: 'static {
    type Bits: Bits;

    const FIELDS: &'static [Field<Self::Bits>];

    /// Convert an integer into a bitfield.
    fn to_bits(&self) -> Self::Bits;
    /// Convert the bitfield to its underlying representation.
    fn from_bits(bits: Self::Bits) -> Self;
}

macro_rules! impl_bits {
    ($($ty:ty),*) => {
        $(
            impl Bits for $ty {
                const BITS: u32 = <$ty>::BITS;
            }
        )*
    };
}

impl_bits!(u8, u16, u32, u64, u128);
