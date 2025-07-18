use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitInt};


#[proc_macro_derive(Discriminant, attributes(discriminant))]
pub fn derive_discriminant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let Data::Enum(data) = input.data else { panic!("Discriminant can only be derived for enums") };

    let mut next_value: usize = 0;
    let match_arms = data.variants.iter().map(|v| {
        // Look for #[descriminant(N)]
        let value_override = v.attrs.iter()
            .find(|e|e.path().is_ident("discriminant"))
            .map(|e|{
                let lit: LitInt = e.parse_args().expect("Expected a number for the discriminant");
                let n = lit.base10_parse().expect("Invalid unsigned integer");
                n
            });
        if let Some(value_override) = value_override {
            next_value = value_override;
        }
        // Make the match arm
        let variant_ident = &v.ident;
        let pat = match v.fields {
            Fields::Unnamed(_) => quote! { #name::#variant_ident (..) },
            Fields::Named(_) => quote! { #name::#variant_ident { .. } },
            Fields::Unit => quote! { #name::#variant_ident },
        };
        // Spit out the words
        let arm = quote! {
            #pat => #next_value as u8,
        };
        next_value += 1;
        arm
    });

    let x = quote! {
        impl ::derive_discriminant::HasDiscriminant for #name {
            fn discriminant(&self) -> u8 {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    x.into()
}
