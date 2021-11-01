use crate as interface;
use interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

pub mod api;
pub mod api_manual;
pub mod client;

/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct Struct1 {
    a: u8,
    b: u16,
    c: u32,
}

/// (Trait1 Doc).
#[contract(mod = "trait1")]
#[allow(unused_variables)]
pub trait Trait1 {
    fn method_ref_mut(&mut self, my_bool: bool) {
        unimplemented!()
    }
    fn method_ref(&self, my_bool: bool) {
        unimplemented!()
    }
    fn method_owned(self, my_bool: bool) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn method_stateless(my_bool: bool) {
        unimplemented!()
    }
    fn method_state_only(self) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn method_no_arg() {
        unimplemented!()
    }
}

/// (Impl Trait1 for Struct Doc).
#[contract(mod = "struct_", trait = "trait1")]
#[allow(unused_variables)]
impl Trait1 for Struct1 {
    /// (Impl method_ref_mut Doc).
    fn method_ref_mut(&mut self, my_bool: bool) {
        unimplemented!()
    }
    /// (Impl method_ref Doc).
    fn method_ref(&self, my_bool: bool) {
        unimplemented!()
    }
    /// (Impl method_owned Doc).
    fn method_owned(self, my_bool: bool) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    /// (Impl method_stateless Doc).
    fn method_stateless(my_bool: bool) {
        unimplemented!()
    }
    // / (Impl method_no_arg Doc).
    fn method_state_only(self) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
    // / (Impl method_no_arg Doc).
    fn method_no_arg() {
        unimplemented!()
    }
}
