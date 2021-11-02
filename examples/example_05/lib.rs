//! An example showing methods that use various kinds of attributes
//! such as `init` and such.

#![allow(unused_parens)]
#![allow(unused_variables)]

use contract_interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault,
};

/// (Original Struct5 documentation)
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct Struct5 {
    a: u8,
}

/// (Trait5 Doc).
#[contract]
pub trait Trait5 {
    #[contract(init())]
    fn method_init() -> Self;
    fn method_payable(&mut self);
    fn method_private();
}

/// (Impl Trait5 for Struct5 Doc).
#[contract(mod = "impl_trait_5", trait = "trait_5")]
impl Trait5 for Struct5 {
    #[contract(init())]
    fn method_init() -> Self {
        unimplemented!()
    }

    #[contract(payable)]
    fn method_payable(&mut self) {
        unimplemented!()
    }

    #[contract(private)]
    fn method_private() {
        unimplemented!();
    }
}

/// Note:  
/// Because of how `#[macro_use]` works, this module must be
/// at root and must come _after_ the referenced macros
/// are defined.  
/// Ie. This should be the last thing at the root of the project.
pub mod macros {
    pub use extern_impl_trait_5;
}
