use proc_macro::TokenStream;
use syn::parse_macro_input;

extern crate proc_macro;

mod bitfield;

#[proc_macro]
pub fn bitfield(input: TokenStream) -> TokenStream {
    bitfield::to_tokens(parse_macro_input!(input as bitfield::Bitfield))
}
