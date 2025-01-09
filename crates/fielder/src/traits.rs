use core::ops::{BitAnd, BitOr, BitXor, Not};

pub struct Field<B> {
    name: &'static str,
    // TODO: should this be u8?
    start_bit: B,
    end_bit: B,

    value: usize,
}
impl<B: BackingBits> Field<B> {
    pub const fn new_field(name: &'static str, start_bit: B, end_bit: B, value: usize) -> Self {
        Self {
            name,
            start_bit,
            end_bit,
            value,
        }
    }
}
pub trait Fields: Sized + 'static {
    type Bits: BackingBits;

    const FIELDS: &'static [Field<Self::Bits>];

    fn from_bits(bits: Self::Bits) -> Self;
    fn to_bits(&self) -> Self::Bits;
}

pub trait BackingBits:
    Clone
    + Copy
    + PartialEq
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Sized
    + 'static
{
}

impl BackingBits for u8 {}
impl BackingBits for usize {}
