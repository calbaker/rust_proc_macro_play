extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
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
    // TODO: make it so that this gets the struct as input
    // then finds the field named my_number based on the fact that it's f64
    // then creates a method that multiplies it by 2 and returns it

    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let mut func_str = String::new();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);
            let ftypes = named.iter().map(|f| &f.ty);
            for (field, ftype) in fields.into_iter().zip(ftypes) {
                if stringify!(#ftype) == "f64" {
                    func_str.push_str("fn double_my_number(&self) -> f64 { self.my_number * 2. }")
                }
            }
        }
    }

    let output = quote! {
        impl #ident {
            // func_str.parse.unwrap();
            // fn double_f64(&self) -> f64 {
            //     self.my_number * 2.
            // }
            fn double_f64(&self) -> f64 { self.my_number * 2. }
        }
    };

    output.into()
}
