extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let description = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                format!(
                    "a struct with these named fields: {}",
                    quote! {#(#idents), *}
                )
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let num_fields = unnamed.iter().count();
                format!("a struct with {} unnamed fields", num_fields)
            }
            syn::Fields::Unit => format!("a unit struct"),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("an enum with these variants: {}", quote! {#(#vs),*})
        }
        syn::Data::Union(DataUnion {
            fields: FieldsNamed { named, .. },
            ..
        }) => {
            let idents = named.iter().map(|f| &f.ident);
            format!("a union with these named fields: {}", quote! {#(#idents),*})
        }
    };

    let output = quote! {
    impl #ident {
        fn describe() {
            println!("{} is {}.", stringify!(#ident), #description);
        }
    }
    };

    output.into()
}

#[proc_macro_derive(DoubleF64)]
pub fn double_f64(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let mut func_str = String::new();

    let (func_name, fident) = if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fident = named[1].ident.clone().unwrap();
            (format_ident!("double_{}", fident), fident)
        } else {
            (format_ident!(""), format_ident!(""))
        }
    } else {
        (format_ident!(""), format_ident!(""))
    };

    let output = quote! {
        impl #ident {
            // func_str.parse.unwrap();
            // fn double_f64(&self) -> f64 {
            //     self.my_number * 2.
            // }
            fn #func_name(&self) -> f64 { self.#fident * 2. }
        }
    };

    output.into()
}
