use quote::quote;
use syn::{FnArg, Ident, ItemTrait, Signature, TraitItem};
use crate::stub_implementor::struct_definition_implementor::StructDefinitionImplementor;
use crate::stub_implementor::struct_implementor::StructImplementor;
use crate::stub_implementor::trait_implementor::TraitImplementor;

use crate::TokenStream2;

mod struct_definition_implementor;
mod struct_implementor;
mod trait_implementor;

pub struct StubImplementor {
    item_trait: ItemTrait,
}

impl StubImplementor {
    pub fn new(item_trait: ItemTrait) -> Self {
        StubImplementor {
            item_trait
        }
    }

    pub fn implement(&self) -> TokenStream2 {
        let item_trait = &self.item_trait;
        let stub_ident = self.create_stub_ident();
        let signatures = self.get_signatures_from_trait();
        let struct_definition_implementation = StructDefinitionImplementor::new(&stub_ident, &signatures).implement();
        let trait_implementation = TraitImplementor::new(&stub_ident, &self.item_trait, &signatures).implement();
        let struct_implementation = StructImplementor::new(&stub_ident, &signatures).implement();

        quote! {
            #item_trait

            #struct_definition_implementation

            #trait_implementation

            #struct_implementation
        }
    }

    fn create_stub_ident(&self) -> Ident {
        let trait_ident = &self.item_trait.ident;
        let stub_name = format!("{}Stub", trait_ident.to_string());
        Ident::new(&stub_name, trait_ident.span())
    }

    fn get_signatures_from_trait(&self) -> Vec<&Signature> {
        self.item_trait.items.iter()
            .filter_map(|i| match i {
                TraitItem::Method(m) => Some(m),
                _ => None
            })
            .map(|m| &m.sig)
            .collect()
    }
}

pub fn method_is_static(signature: &Signature) -> bool {
    if signature.inputs.is_empty() {
        return true
    }

    match signature.inputs[0] {
        FnArg::Receiver(_) => false,
        _ => false
    }
}