//! An example showing methods that use various kinds of `self`.

#![allow(unused_variables)]

use crate as interface;
use interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault,
};

pub mod api;
pub mod api_manual;
pub mod client;

/// (Struct1 Doc).
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct Struct1 {
    a: u8,
    b: u16,
    c: u32,
}

/// (Trait1 Doc).
#[contract]
#[allow(unused_variables)]
pub trait Trait1 {
    /// (method_ref_mut Doc).
    fn method_ref_mut(&mut self, my_bool: bool) {
        unimplemented!()
    }

    /// (method_ref Doc).
    fn method_ref(&self, my_bool: bool) {
        unimplemented!()
    }

    /// (method_owned Doc).
    ///
    /// Owned method must return `Self`,
    /// which will be stored as the state at the end.
    fn method_owned(self, my_bool: bool) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    /// (method_stateless Doc).
    fn method_stateless(my_bool: bool) {
        unimplemented!()
    }

    /// (method_state_only Doc).
    ///
    /// Owned method must return `Self`,
    /// which will be stored as the state at the end.
    fn method_state_only(self) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    /// (method_no_arg Doc).
    fn method_no_arg() {
        unimplemented!()
    }
}

/// (Impl Trait1 for Struct1 Doc).
#[contract(mod = "impl_trait_1", trait = "trait_1")]
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
    ///
    /// Owned method must return `Self`,
    /// which will be stored as the state at the end.
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

    /// (Impl method_state_only Doc).
    ///
    /// Owned method must return `Self`,
    /// which will be stored as the state at the end.
    fn method_state_only(self) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    /// (Impl method_no_arg Doc).
    fn method_no_arg() {
        unimplemented!()
    }
}
