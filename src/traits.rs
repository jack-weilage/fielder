use core::ops::{BitAnd, BitOr, BitXor, Not};

pub struct Field<B> {
    name: &'static str,
    // TODO: should this be u8?
    bits: usize,

    value: B,
}
pub trait Fields<
    Bits: Clone
        + Copy
        + PartialEq
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + Not<Output = Self>
        + Sized
        + 'static,
>: Sized + 'static
{
    const FIELDS: &'static [Field<Self>];

    fn from_bits(bits: Bits) -> Self;
    fn to_bits(&self) -> Bits;
}
