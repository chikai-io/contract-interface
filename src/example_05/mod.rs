#![allow(unused_parens)]
#![allow(unused_variables)]

use crate as interface;
use interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

/// (Original Struct5 documentation)
#[near_bindgen]
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
#[contract(mod = "impl_struct_5", trait = "trait_5")]
#[allow(unused_variables)]
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
