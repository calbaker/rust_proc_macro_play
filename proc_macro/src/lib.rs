extern crate proc_macro2;
use proc_macro2::TokenStream as TokenStream2;
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};


extern crate quote;
extern crate syn;

#[proc_macro_derive(DoubleF64)]
pub fn double_f64(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut func_stream = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);
            let ftypes = named.iter().map(|f| &f.ty);

            for (i, (field, ftype)) in fields.into_iter().zip(ftypes).enumerate() {
                // if stringify!(#ftype) == "f64" {
                    if i == 2 || i == 1 {
                        let fname = format_ident!("double_{}", field.clone().unwrap());
                    func_stream.extend::<TokenStream2>(quote! { fn #fname(&self) -> f64 { self.#field * 2.0 } });
                }
            }
        }
    };


    let output = quote! {
        impl #ident {
            #func_stream
        }
    };

    output.into()
}

