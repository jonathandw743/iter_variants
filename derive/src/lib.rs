use proc_macro as pm;
use proc_macro2::{self as pm2, Span};
use quote::{format_ident, quote};
use syn::{parse_macro_input, token::PathSep, Data, DeriveInput, Fields, PathArguments, Type};

fn do_fields(fields: &Fields, ident: pm2::TokenStream) -> pm2::TokenStream {
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
                let mut ty = field.ty.clone();
                // Option<T> -> Option::<T>
                if let Type::Path(type_path) = &mut ty {
                    if let Some(segment) = type_path.path.segments.last_mut() {
                        if let PathArguments::AngleBracketed(generic_arguments) = &mut segment.arguments {
                            generic_arguments.colon2_token = Some(PathSep { spans: [Span::call_site(); 2] })
                        }
                    }
                }
                result = quote! {
                    #ty::iter_variants(|#v| #result)
                };
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
                let mut ty = field.ty.clone();
                // Option<T> -> Option::<T>
                if let Type::Path(type_path) = &mut ty {
                    if let Some(segment) = type_path.path.segments.last_mut() {
                        if let PathArguments::AngleBracketed(generic_arguments) = &mut segment.arguments {
                            generic_arguments.colon2_token = Some(PathSep { spans: [Span::call_site(); 2] })
                        }
                    }
                }
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
pub fn iter_variants_derive(input: pm::TokenStream) -> pm::TokenStream {
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
            type IterVariantsInput = Self;
            fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
                #output
            }
        }
    };

    pm::TokenStream::from(expanded)
}
