#[derive(Debug, Clone, Copy)]
pub struct Field<Bits> {
    pub name: &'static str,

    pub start_bit: Bits,
    pub end_bit: Bits,

    pub mask: Bits,
    pub value: Bits,
}
