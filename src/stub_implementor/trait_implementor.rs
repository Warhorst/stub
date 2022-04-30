use quote::quote;
use syn::{Ident, ItemTrait, Signature};
use crate::stub_implementor::method_is_static;
use crate::TokenStream2;

pub struct TraitImplementor<'a> {
    stub_ident: &'a Ident,
    item_trait: &'a ItemTrait,
    signatures: &'a Vec<&'a Signature>
}

impl<'a> TraitImplementor<'a> {
    pub fn new(stub_ident: &'a Ident, item_trait: &'a ItemTrait, signatures: &'a Vec<&'a Signature>) -> Self {
        Self { stub_ident, item_trait, signatures }
    }

    /// Generate the trait implementation of the stub struct.
    pub fn implement(&self) -> TokenStream2 {
        let ident = &self.stub_ident;
        let trait_ident = &self.item_trait.ident;
        let methods = self.implement_methods();

        quote! {
            impl #trait_ident for #ident {
                #(#methods)*
            }
        }
    }

    fn implement_methods(&self) -> Vec<TokenStream2> {
        self.signatures.into_iter()
            .map(|sig| self.implement_methods_for_signature(sig))
            .collect()
    }

    fn implement_methods_for_signature(&self, sig: &Signature) -> TokenStream2 {
        match method_is_static(sig) {
            true => self.implement_static_method(sig),
            false => self.implement_non_static_method(sig)
        }
    }

    /// Currently it is not possible to stub static methods. So we just panic.
    fn implement_static_method(&self, sig: &Signature) -> TokenStream2 {
        let ident = sig.ident.to_string();
        quote! {
            #sig {
                panic!("Called unstubbed method {}.", #ident)
            }
        }
    }

    fn implement_non_static_method(&self, sig: &Signature) -> TokenStream2 {
        let ident = &sig.ident;

        quote! {
            #sig {
                (self.#ident)()
            }
        }
    }
}