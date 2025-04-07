use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

fn do_fields(fields: &Fields, ident: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unnamed(fields_unnamed) => {
            let vs = (0..fields_unnamed.unnamed.len()).map(|i| format_ident!("v{}", i));
            let mut result = {
                let vs = vs.clone();
                quote! {
                    f(#ident(#(#vs,)*))
                }
            };
            for (field, v) in fields_unnamed.unnamed.iter().zip(vs) {
                let ty = &field.ty;
                result = quote! {
                    #ty::iter_variants(|#v| #result)
                }
            }
            result
        }
        Fields::Named(fields_named) => {
            let idents: Vec<_> = fields_named
                .named
                .iter()
                .map(|field| field.ident.clone())
                .collect();
            let mut result = quote! {
                f(#ident {
                    #(#idents,)*
                })
            };
            for field in &fields_named.named {
                let ident = field.ident.clone();
                let ty = field.ty.clone();
                result = quote! {
                    #ty::iter_variants(|#ident| #result)
                }
            }
            result
        }
        Fields::Unit => quote! {
            f(#ident)
        },
    }
}

#[proc_macro_derive(IterVariants)]
pub fn iter_variants_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let output = match input.data {
        Data::Enum(data_enum) => {
            let arms = data_enum.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                do_fields(&variant.fields, quote! {Self::#variant_ident})
            });

            quote! {
                #(#arms;)*
            }
        }
        Data::Struct(data_struct) => {
            let ident = ident.clone();
            do_fields(&data_struct.fields, quote! { #ident })
        }
        _ => syn::Error::new_spanned(&ident, "`Name` can only be derived for enums or structs")
            .to_compile_error(),
    };

    let expanded = quote! {
        impl IterVariants for #ident {
            type T = Self;
            fn iter_variants<F: Fn(Self::T)>(f: F) {
                #output
            }
        }
    };

    TokenStream::from(expanded)
}
