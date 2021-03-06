extern crate proc_macro2;
use proc_macro2::TokenStream as TokenStream2;
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field, FieldValue, FieldsNamed, Type};

extern crate quote;
extern crate syn;

// could make it so that presence or absence of `orphaned` is determines whether setters are created

/// macro for creating appropriate setters and getters for pyo3 struct attributes
#[proc_macro_attribute]
pub fn impl_pyo3_get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.clone();
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut impl_block = TokenStream2::default();
    impl_block.extend::<TokenStream2>(attr.into());

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);

            let mut field_names = Vec::new();
            for field in fields.clone().into_iter() {
                field_names.push(field.clone().unwrap().to_string());
            }

            let orphaned_present = field_names.iter().any(|f| f == "orphaned");
            println!("value of orphaned: {}", orphaned_present);

            let ftypes = named.iter().map(|f| &f.ty);

            for (field, ftype) in fields.into_iter().zip(ftypes.into_iter()) {
                if let Type::Path(type_path) = ftype {
                    if type_path.clone().into_token_stream().to_string() == "si :: Power" {
                        let fname = format_ident!("get_{}_watts", field.clone().unwrap());
                        impl_block.extend::<TokenStream2>(quote! {
                            #[getter]
                            fn #fname(&self) -> f64 { self.#field.get::<si::watt>() }
                        });
                        if orphaned_present {
                            println!("got inside orphaned present");

                            let set_name = format_ident!("set_{}_watts", field.clone().unwrap());
                            impl_block.extend::<TokenStream2>(quote! {
                                #[setter]
                                fn #set_name(&mut self, value:f64) -> PyResult<()> {
                                    if !self.orphaned {
                                        self.#field = value * si::W;
                                        Ok(())
                                    } else {
                                        Err(PyAttributeError::new_err(
                                            "Setting field value on nested struct not allowed.",
                                        ))
                                    }
                                }
                            })
                        }
                    } else if type_path.clone().into_token_stream().to_string() == "si :: Ratio" {
                        let fname = format_ident!("get_{}", field.clone().unwrap());
                        impl_block.extend::<TokenStream2>(quote! {
                            #[getter]
                            fn #fname(&self) -> f64 { self.#field.get::<si::ratio>()
                            }
                        });
                    } else if type_path.clone().into_token_stream().to_string() == "si :: Energy" {
                        let fname = format_ident!("get_{}_joules", field.clone().unwrap());
                        impl_block.extend::<TokenStream2>(quote! {
                            #[getter]
                            fn #fname(&self) -> f64 { self.#field.get::<si::joule>()
                            }
                        });
                    }
                }
            }
        }
    };

    let impl_block = quote! {
        #[pymethods]
        impl #ident {
            #impl_block
        }
    };

    let mut output: TokenStream2 = item.into();
    output.extend(impl_block);
    output.into()
}
