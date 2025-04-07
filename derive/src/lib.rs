// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Data, DeriveInput, Fields};


// #[proc_macro_derive(IterVariants)]
// pub fn my_trait_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let ident = input.ident;

//     let output = match input.data {
//         Data::Enum(data_enum) => {
//             let arms = data_enum.variants.iter().map(|variant| {
//                 let variant_ident = &variant.ident;

//                 let explicit_name = variant
//                     .attrs
//                     .iter()
//                     .find(|attr| attr.path().is_ident("name"))
//                     .and_then(|attr| attr.parse_args::<LitStr>().ok())
//                     .map(|lit| lit.value());

//                 match (&variant.fields, &explicit_name) {
//                     (Fields::Unnamed(fields), _) => {
//                         let field_count = fields.unnamed.len();
//                         let vars: Vec<_> =
//                             (0..field_count).map(|i| format_ident!("f{}", i)).collect();
//                         quote! {
//                             Self::#variant_ident(#(#vars),*) => {
//                                 let mut result = String::new();
//                                 #(result += &#vars.name();)*
//                                 result
//                             }
//                         }
//                     }
//                     (Fields::Named(_), _) => quote! {
//                         Self::#variant_ident { inner } => inner.name()
//                     },
//                     (Fields::Unit, Some(name)) => quote! {
//                         Self::#variant_ident => #name.to_string()
//                     },
//                     (Fields::Unit, None) => syn::Error::new_spanned(
//                         variant_ident,
//                         "Unit variant must have #[name(\"...\")]",
//                     )
//                     .to_compile_error(),
//                 }
//             });

//             quote! {
//                 impl Name for #ident {
//                     fn name(&self) -> String {
//                         match self {
//                             #(#arms),*
//                         }
//                     }
//                 }
//             }
//         }

//         Data::Struct(data_struct) => match data_struct.fields {
//             Fields::Unit => {
//                 let name_attr = input
//                     .attrs
//                     .iter()
//                     .find(|attr| attr.path().is_ident("name"))
//                     .and_then(|attr| attr.parse_args::<LitStr>().ok());

//                 if let Some(name) = name_attr {
//                     quote! {
//                         impl Name for #ident {
//                             fn name(&self) -> String {
//                                 #name.to_string()
//                             }
//                         }
//                     }
//                 } else {
//                     syn::Error::new_spanned(ident, "Unit structs must have #[name(\"...\")]")
//                         .to_compile_error()
//                 }
//             }

//             Fields::Unnamed(fields) => {
//                 let field_count = fields.unnamed.len();
//                 let vars: Vec<_> = (0..field_count).collect();

//                 if field_count == 1 {
//                     quote! {
//                         impl Name for #ident {
//                             fn name(&self) -> String {
//                                 self.0.name()
//                             }
//                         }
//                     }
//                 } else {
//                     quote! {
//                         impl Name for #ident {
//                             fn name(&self) -> String {
//                                 let mut result = String::new();
//                                 #(result += self.#vars.name();)*
//                                 result
//                             }
//                         }
//                     }
//                 }
//             }

//             Fields::Named(_) => {
//                 syn::Error::new_spanned(ident, "Named-field structs are not supported")
//                     .to_compile_error()
//             }
//         },

//         _ => syn::Error::new_spanned(ident, "`Name` can only be derived for enums or structs")
//             .to_compile_error(),
//     };

//     let expanded = quote! {
//         impl IterVariants for #ident {
//             fn iter_variants(&self, f: fn(Self)) {
                
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }
