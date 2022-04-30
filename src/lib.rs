extern crate syn;

use macro_test::proc_macro_attribute2;
use syn::{AttributeArgs, ItemTrait, parse2};
use syn::__private::TokenStream2;

use crate::stub_implementor::StubImplementor;

mod stub_implementor;

#[proc_macro_attribute2]
pub fn stub(_attributes: AttributeArgs, item: TokenStream2) -> TokenStream2 {
    if let Ok(item_trait) = parse2::<ItemTrait>(item) {
        return StubImplementor::new(item_trait).implement().into()
    }

    panic!("The 'stub' attribute is only allowed on traits")
}

#[cfg(test)]
mod tests {
    use macro_test::assert_attribute_implementation_as_expected;

    #[test]
    fn works() {
        assert_attribute_implementation_as_expected!(
            crate : stub,
            item: {
                #[stub]
                pub trait MyTrait {
                    fn get_ze_value(&self) -> usize {}
                }
            }
            expected: {
                pub trait MyTrait {
                    fn get_ze_value(&self) -> usize {}
                }

                pub struct MyTraitStub {
                    get_ze_value: Box<Fn() -> usize>,
                }

                impl MyTrait for MyTraitStub {
                    fn get_ze_value(&self) -> usize {
                        (self.get_ze_value)()
                    }
                }

                impl MyTraitStub {
                    pub fn new() -> Self {
                        MyTraitStub {
                            get_ze_value: Box::new(|| unimplemented!()),
                        }
                    }

                    pub fn get_ze_value_stub(&mut self, f: impl Fn() -> usize + 'static) {
                        self.get_ze_value = Box::new(f)
                    }
                }
            }
        )
    }
}
