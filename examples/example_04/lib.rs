//! An example showing methods that use various kinds of argument types
//! such as `&T` and such.

#![allow(unused_parens)]
#![allow(unused_variables)]

use contract_interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault,
};

/// (Original Struct4 documentation)
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct Struct4 {
    a: u8,
    b: u16,
    c: u32,
}

/// (Trait4 Doc).
#[contract]
pub trait Trait4 {
    fn fn_array(my_array: [bool; 2]);
    // fn fn_bare_fn(my_fn: fn(bool) -> bool);
    // fn fn_impl_trait(my_impl: impl Clone);
    fn fn_paren(my_paren: (bool));
    fn fn_path(my_path: std::vec::Vec<bool>);
    fn fn_ptr(my_ptr: *const bool);
    fn fn_ptr_mut(my_ptr: *mut bool);
    fn fn_ref(my_ref: &bool);
    fn fn_ref_mut(my_ref: &mut bool);
    // fn fn_slice(my_slice: &[bool]);
    // fn fn_dyn(my_dyn: Box<dyn Clone>);
    fn fn_tuple(my_tuple: (bool, bool));
    // fn fn_tuple2((a, b): (bool, bool));

    // fn fn_box(box my_box: bool) {
    //     unimplemented!()
    // }
    // fn fn_ref(ref x: bool);
}

/// (Impl Trait4 for Struct4 Doc).
#[contract(mod = "impl_trait_4", trait = "trait_4")]
impl Trait4 for Struct4 {
    fn fn_array(my_array: [bool; 2]) {
        unimplemented!()
    }
    // fn fn_bare_fn(my_fn: fn(bool) -> bool) {
    //     unimplemented!()
    // }
    // fn fn_impl_trait(my_impl: impl Clone) {
    //     unimplemented!()
    // }
    fn fn_paren(my_paren: (bool)) {
        unimplemented!()
    }
    fn fn_path(my_path: std::vec::Vec<bool>) {
        unimplemented!()
    }
    fn fn_ptr(my_ptr: *const bool) {
        unimplemented!()
    }
    fn fn_ptr_mut(my_ptr: *mut bool) {
        unimplemented!()
    }

    fn fn_ref(my_ref: &bool) {
        unimplemented!()
    }
    fn fn_ref_mut(my_ref: &mut bool) {
        unimplemented!()
    }

    // fn fn_slice(my_slice: &[bool]) {
    //     unimplemented!()
    // }
    // fn fn_dyn(my_dyn: dyn Clone) {
    //     unimplemented!()
    // }
    fn fn_tuple(my_tuple: (bool, bool)) {
        unimplemented!()
    }
}

/// Note:  
/// Because of how `#[macro_use]` works, this module must be
/// at root and must come _after_ the referenced macros
/// are defined.  
/// Ie. This should be the last thing at the root of the project.
pub mod macros {
    pub use extern_impl_trait_4;
}
