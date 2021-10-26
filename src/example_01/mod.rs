// pub mod api;
// pub mod client;

use crate as interface;
use interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct Struct {
    a: u8,
    b: u16,
    c: u32,
}

/// (Trait4 Doc).
#[contract(mod = "trait4")]
#[allow(unused_variables)]
pub trait Trait4 {
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

/// (Impl Trait4 for Struct Doc).
#[contract(mod = "struct_", trait = "trait4")]
#[allow(unused_variables)]
impl Trait4 for Struct {
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
