use crate as interface;
use interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

pub mod api;

/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct2 {
    a: u8,
    b: u16,
    c: u32,
}

// #[near_bindgen]
// impl Trait for Struct2 {
//     fn method_a(&mut self, _my_string: String) {
//         unimplemented!()
//     }
//     fn method_b(&mut self, _my_string: String, _my_bool: bool) -> bool {
//         unimplemented!()
//     }
// }
