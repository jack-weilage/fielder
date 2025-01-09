use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token as T,
};

pub struct Bitfield {
    attrs: Vec<syn::Attribute>,

    visibility: syn::Visibility,
    name: syn::Ident,
    ty: syn::Type,

    fields: Punctuated<Field, T![;]>,
}
impl Parse for Bitfield {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Attribute cannot be automatically parsed, so explicitly call parse_outer.
        let attrs = syn::Attribute::parse_outer(input)?;

        let visibility = input.parse::<syn::Visibility>()?;
        input.parse::<T![struct]>()?;
        let name = input.parse::<syn::Ident>()?;
        input.parse::<T![:]>()?;
        let ty = input.parse::<syn::Type>()?;

        let content;
        syn::braced!(content in input);

        let fields = content.parse_terminated(Field::parse, T![;])?;

        Ok(Bitfield {
            attrs,
            visibility,
            name,
            ty,
            fields,
        })
    }
}

pub struct Field {
    attrs: Vec<syn::Attribute>,

    name: syn::Ident,
    start_bit: syn::LitInt,
    end_bit: syn::LitInt,
    value: syn::Expr,
}
impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Attribute cannot be automatically parsed, so explicitly call parse_outer.
        let attrs = syn::Attribute::parse_outer(input)?;

        let name = input.parse::<syn::Ident>()?;
        input.parse::<T![:]>()?;
        // TODO: Validate that the start bit is less than the end bit.
        // TODO: Validate that the start bit is smaller than the length of the underlying bits.
        let start_bit = input.parse::<syn::LitInt>()?;
        // If the next two tokens are "..", this field has a range of bits.
        // TODO: Validate that the end bit is smaller than the length of the underlying bits.
        let end_bit = if input.peek(T![.]) && input.peek2(T![.]) {
            input.parse::<T![.]>()?;
            input.parse::<T![.]>()?;

            input.parse::<syn::LitInt>()?
        } else {
            start_bit.clone()
        };

        let value = if input.peek(T![=]) {
            input.parse::<T![=]>()?;
            input.parse::<syn::Expr>()?
        } else {
            syn::parse_quote! { 1 }
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

pub fn to_tokens(bitfield: Bitfield) -> TokenStream {
    let Bitfield {
        attrs,
        visibility,
        name,
        ty,
        fields,
    } = bitfield;

    let const_fields = fields.iter().map(
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
    );

    quote! {
        #(#attrs)*
        #visibility struct #name(#ty);

        #[allow(non_upper_case_globals)]
        impl #name {
            #(#const_fields)*
        }
    }
    .into()
}
