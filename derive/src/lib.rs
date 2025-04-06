use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(IterVariants)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let expanded = quote! {
        impl IterVariants for #ident {
            fn iter_variants(&self, f: fn(Self)) {
                
            }
        }
    };

    TokenStream::from(expanded)
}
