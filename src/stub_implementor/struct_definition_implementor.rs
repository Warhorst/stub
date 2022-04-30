use quote::quote;
use syn::{Ident, Signature};
use crate::stub_implementor::method_is_static;
use crate::TokenStream2;

pub struct StructDefinitionImplementor<'a> {
    stub_ident: &'a Ident,
    signatures: &'a Vec<&'a Signature>
}

impl<'a> StructDefinitionImplementor<'a> {
    pub fn new(stub_ident: &'a Ident, signatures: &'a Vec<&'a Signature>) -> Self {
        Self { stub_ident, signatures }
    }

    /// Generate the definition of the stub struct.
    pub fn implement(&self) -> TokenStream2 {
        let stub_ident = &self.stub_ident;
        let fields = self.implement_fields();

        quote! {
            pub struct #stub_ident {
                #(#fields,)*
            }
        }
    }

    fn implement_fields(&self) -> Vec<TokenStream2> {
        self.signatures.into_iter()
            .filter(|sig| !method_is_static(sig))
            .map(|sig| self.implement_field_from_signature(sig))
            .collect()
    }

    fn implement_field_from_signature(&self, signature: &Signature) -> TokenStream2 {
        let ident = &signature.ident;
        let return_type = &signature.output;

        quote!{
            #ident : Box<Fn() #return_type>
        }
    }
}