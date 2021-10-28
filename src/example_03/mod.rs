use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

pub mod api;

/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct3 {
    a: u8,
    b: u16,
    c: u32,
}

pub trait Trait3 {
    fn method_ex3_a(&mut self, _my_string: String);
    fn method_ex3_b(&mut self, _my_string: String, _my_bool: &bool) -> bool;
}

#[near_bindgen]
impl Trait3 for Struct3 {
    fn method_ex3_a(&mut self, _my_string: String) {
        unimplemented!()
    }
    fn method_ex3_b(&mut self, _my_string: String, _my_bool: &bool) -> bool {
        unimplemented!()
    }
}
