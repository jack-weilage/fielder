use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token as T,
};

macro_rules! syn_error {
    ($span:expr, $comment:expr) => {
        return syn::Error::new_spanned($span, $comment)
            .to_compile_error()
            .into();
    };
}

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

    is_counter: bool,
}
impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Attribute cannot be automatically parsed, so explicitly call parse_outer.
        let attrs = syn::Attribute::parse_outer(input)?;

        let name = input.parse::<syn::Ident>()?;
        input.parse::<T![:]>()?;
        let start_bit = input.parse::<syn::LitInt>()?;
        // If the next two tokens are "..", this field has a range of bits.
        let end_bit = if input.peek(T![.]) && input.peek2(T![.]) {
            input.parse::<T![.]>()?;
            input.parse::<T![.]>()?;

            input.parse::<syn::LitInt>()?
        } else {
            // If there's no range, just use the start bit again.
            start_bit.clone()
        };

        let value = if input.peek(T![=]) {
            input.parse::<T![=]>()?;
            input.parse::<syn::Expr>()?
        } else if start_bit.to_string() == end_bit.to_string() {
            // If the field only spans a single bit, the value should be 1 by default to make it
            // like a bitflag.
            syn::parse_quote! { 1 }
        } else {
            // If the field spans multiple bits, the value should be 0 to allow for more simple
            // definition of counter bitfields.
            syn::parse_quote! { 0 }
        };

        Ok(Self {
            attrs,
            name,
            start_bit,
            end_bit,
            value: value.clone(),
            is_counter: value == syn::parse_quote! { !0 },
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

    // Ensure that the type is an unsigned integer.
    let bits = match ty.to_token_stream().to_string().as_str() {
        "u8" => 8,
        "u16" => 16,
        "u32" => 32,
        "u64" => 64,
        "u128" => 128,
        _ => {
            syn_error!(ty, "Type must be one of u8, u16, u32, u64, or u128");
        }
    };

    let mut const_fields = Vec::with_capacity(fields.len());
    let mut impl_const_fields = Vec::with_capacity(fields.len());
    // TODO: ensure that multiple fields cannot have both the same start + end bits and value.
    // TODO: ensure that some kind of warning is given when a range overlaps another.
    for field in fields {
        let Field {
            attrs,
            name,
            start_bit,
            end_bit,
            value,
            is_counter,
        } = field;

        let start: i32 = start_bit.base10_parse().unwrap();
        let end: i32 = end_bit.base10_parse().unwrap();

        if start > end {
            syn_error!(start_bit, "Start bit must not be greater than end bit");
        }

        if start >= bits {
            syn_error!(
                start_bit,
                format!("Start bit must be less than underlying size ({bits})")
            );
        }
        if end >= bits {
            syn_error!(
                end_bit,
                format!("End bit must be less than underlying size ({bits})")
            );
        }

        if start < 0 {
            syn_error!(start_bit, "Start bit must be greater than zero");
        }
        if end < 0 {
            syn_error!(end_bit, "End bit must be greater than zero");
        }

        const_fields.push(quote! {
            #(#attrs)*
            const #name: ::fielder::Field<#ty> = {
                #[allow(clippy::identity_op, clippy::eq_op)]
                let mask = ((1 << (#end_bit - #start_bit + 1)) - 1) << #start_bit;

                ::fielder::Field {
                    name: stringify!(#name),
                    start_bit: #start_bit,
                    end_bit: #end_bit,
                    mask,
                    value: (#value << #start_bit) & mask,
                    is_counter: #is_counter,
                }
            };
        });
        impl_const_fields.push(quote! { Self::#name });
    }

    let defmt = if cfg!(feature = "defmt") {
        quote! {
            impl ::defmt::Format for #name {
                fn format(&self, fmt: ::defmt::Formatter) {
                    let mut first = true;

                    ::defmt::write!(fmt, "{}(", stringify!(#name));
                    for field in Self::FIELDS.iter().filter(|&&f| self.contains(f)) {
                        if !first {
                            ::defmt::write!(fmt, " | ");
                        }
                        first = false;

                        ::defmt::write!(fmt, "{}", field.name);
                        if field.is_counter {
                            ::defmt::write!(fmt, "({})", self.get_literal(*field));
                        }
                    }
                    ::defmt::write!(fmt, ")");
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #(#attrs)*
        #visibility struct #name(#ty);

        #[allow(non_upper_case_globals)]
        impl #name {
            #(#const_fields)*

            const FIELDS: &'static [::fielder::Field<#ty>] = &[
                #(#impl_const_fields),*
            ];

            /// Convert an integer into a bitfield.
            #[inline]
            pub const fn from_bits(bits: #ty) -> Self {
                Self(bits)
            }
            /// Convert the bitfield to its underlying representation.
            #[inline]
            pub const fn to_bits(&self) -> #ty {
                self.0
            }

            /// Check if the bitfield contains a specific flag.
            #[inline]
            pub const fn contains(&self, field: ::fielder::Field<#ty>) -> bool {
                field.is_counter || (self.to_bits() & field.mask) == field.value
            }

            /// Set the bit/s related to the field to the field's value value.
            #[inline]
            pub const fn set(&mut self, field: ::fielder::Field<#ty>) -> Self {
                self.0 = (self.to_bits() & !field.mask) | field.value;

                *self
            }

            /// Unset the bit/s related to the field. Note that if there is a field with the value `0` over
            /// the bit/s, that field will become active.
            #[inline]
            pub const fn unset(&mut self, field: ::fielder::Field<#ty>) -> Self {
                self.0 = self.to_bits() & !field.mask;

                *self
            }

            /// Get the literal value of the bit/s related to the field.
            #[inline]
            pub const fn get_literal(&self, field: ::fielder::Field<#ty>) -> #ty {
                (self.to_bits() & field.mask) >> field.start_bit
            }

            /// Sets the literal value of the bit/s related to the field.
            #[inline]
            pub const fn set_literal(&mut self, field: ::fielder::Field<#ty>, value: #ty) -> Self {
                self.0 = (self.to_bits() & !field.mask) | ((value << field.start_bit) & field.mask);

                *self
            }
        }
        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(self, f)
            }
        }
        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut first = true;

                f.write_fmt(format_args!("{}(", stringify!(#name)))?;
                for field in Self::FIELDS.iter().filter(|&&f| self.contains(f)) {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;

                    f.write_fmt(format_args!("{}", field.name))?;
                    if field.is_counter {
                        f.write_fmt(format_args!("({})", self.get_literal(*field)))?;
                    }
                }
                f.write_str(")")?;

                Ok(())
            }
        }

        #defmt
    }
    .into()
}
// TODO: use !0 to denote the field being a counter. This should still mask the bits and set a
// "is_counter" mask.
