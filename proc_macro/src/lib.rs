extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(DoubleF64)]
pub fn double_f64(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let (func_name, fident) = if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let f = named[1].ident.clone().unwrap();
            (format_ident!("double_{}", f), f)
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
