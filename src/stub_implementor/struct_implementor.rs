use quote::quote;
use syn::{Ident, Signature};
use crate::stub_implementor::method_is_static;
use crate::TokenStream2;

pub struct StructImplementor<'a> {
    stub_ident: &'a Ident,
    signatures: &'a Vec<&'a Signature>
}

impl<'a> StructImplementor<'a> {
    pub fn new(stub_ident: &'a Ident, signatures: &'a Vec<&'a Signature>) -> Self {
        Self { stub_ident, signatures }
    }

    /// Generate the implementation block of the stub struct.
    pub fn implement(&self) -> TokenStream2 {
        let ident = &self.stub_ident;
        let constructor = self.implement_constructor();
        let stub_methods = self.implement_stub_methods();

        quote! {
            impl #ident {
                #constructor

                #(#stub_methods)*
            }
        }
    }

    fn implement_constructor(&self) -> TokenStream2 {
        let ident = &self.stub_ident;
        let initial_field_settings = self.implement_initial_field_settings();

        quote! {
            pub fn new() -> Self {
                #ident {
                    #(#initial_field_settings,)*
                }
            }
        }
    }

    fn implement_initial_field_settings(&self) -> Vec<TokenStream2> {
        self.signatures
            .iter()
            .filter(|sig| !method_is_static(sig))
            .map(|sig| self.implement_initial_field_setting(sig))
            .collect()
    }

    /// Create the initial setting for a stub field.
    ///
    /// By default, every stubable method is unimplemented.
    fn implement_initial_field_setting(&self, sig: &Signature) -> TokenStream2 {
        let ident = &sig.ident;
        quote! {
            #ident : Box::new(|| unimplemented!())
        }
    }

    fn implement_stub_methods(&self) -> Vec<TokenStream2> {
        self.signatures
            .iter()
            .filter(|sig| !method_is_static(sig))
            .map(|sig| self.implement_stub_method(sig))
            .collect()
    }

    fn implement_stub_method(&self, sig: &Signature) -> TokenStream2 {
        let ident = &sig.ident;
        let return_type = &sig.output;
        let stubable_ident = Ident::new(&format!("{}_stub", ident), ident.span());

        quote! {
            pub fn #stubable_ident (&mut self, f: impl Fn() #return_type + 'static) {
                self.#ident = Box::new(f)
            }
        }
    }
}