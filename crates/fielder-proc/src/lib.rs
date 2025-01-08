use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Attribute, Expr, Ident, LitInt, Token as T, Type, Visibility,
};

extern crate proc_macro;

struct Bitfield {
    visibility: Visibility,
    name: Ident,
    ty: Type,

    fields: Punctuated<Field, T![;]>,
}
struct Field {
    attrs: Vec<Attribute>,
    name: Ident,
    start_bit: LitInt,
    end_bit: LitInt,
    value: Expr,
}
impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Attribute cannot be automatically parsed, so explicitly call parse_outer.
        let attrs = Attribute::parse_outer(input)?;

        let name = input.parse::<Ident>()?;
        input.parse::<T![:]>()?;
        // TODO: Validate that the start bit is less than the end bit.
        // TODO: Validate that the start bit is smaller than the length of the underlying bits.
        let start_bit = input.parse::<LitInt>()?;
        // If the next two tokens are "..", this field has a range of bits.
        // TODO: Validate that the end bit is smaller than the length of the underlying bits.
        let end_bit = if input.peek(T![.]) && input.peek2(T![.]) {
            input.parse::<T![.]>()?;
            input.parse::<T![.]>()?;

            input.parse::<LitInt>()?
        } else {
            start_bit.clone()
        };

        let value = if input.peek(T![=]) {
            input.parse::<T![=]>()?;
            input.parse::<Expr>()?
        } else {
            parse_quote! { 1 }
        };

        Ok(Self {
            attrs,
            name,
            start_bit,
            end_bit,
            value,
        })
    }
}

impl Parse for Bitfield {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<Visibility>()?;
        input.parse::<T![struct]>()?;
        let name = input.parse::<Ident>()?;
        input.parse::<T![:]>()?;
        let ty = input.parse::<Type>()?;

        let content;
        braced!(content in input);

        let fields = content.parse_terminated(Field::parse, T![;])?;

        Ok(Bitfield {
            visibility,
            name,
            ty,
            fields,
        })
    }
}

#[proc_macro]
pub fn bitfield(input: TokenStream) -> TokenStream {
    let Bitfield {
        visibility,
        name,
        ty,
        fields,
    } = parse_macro_input!(input as Bitfield);

    let const_fields = fields
        .iter()
        .map(
            |Field {
                 attrs,
                 name,
                 value,
                 start_bit,
                 ..
             }| {
                quote! {
                    #(#attrs)*
                    const #name: Self = Self(#value << #start_bit);
                }
            },
        )
        .collect::<Vec<_>>();

    quote! {
        #visibility struct #name(#ty);

        #[allow(non_upper_case_globals)]
        impl #name {
            #(#const_fields)*
        }
    }
    .into()
}
