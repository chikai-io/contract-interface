//! Example of defining an contract to be called by consumer contracts.
//! (the consumer contracts still need to define their CallOut's)

use super::CalledIn;
use crate::args::{Json1, Json2};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

// #[CalledIn]
/// (Original Message documentation)
pub trait Message {
    /// (Original method_a documentation)
    fn method_a(&mut self, my_string: String) -> bool;

    /// (Original method_b documentation)
    fn method_b(&mut self, my_string: String, my_bool: bool) -> bool;
}
// created by macro
pub mod message_concrete {
    use std::marker::PhantomData;

    ///
    ///
    /// (Original method_a documentation)
    pub struct CalledInMethodA<State>(PhantomData<State>);

    ///
    ///
    /// (Original method_b documentation)
    pub struct CalledInMethodB<State>(PhantomData<State>);
}

// specific
/// (Original Abc documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Abc {
    a: u8,
    b: u16,
    c: u32,
}

// specific (where the CalledIn "derive" must happen)
// #[CalledIn]
impl Message for Abc {
    fn method_a(&mut self, _my_string: String) -> bool {
        todo!()
    }
    fn method_b(&mut self, _my_string: String, _my_bool: bool) -> bool {
        todo!()
    }
}
// created by macro
impl CalledIn for message_concrete::CalledInMethodA<Abc> {
    type State = Abc;
    type Args = Json1<String>;
    type Return = Json1<bool>;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut <Self as CalledIn>::State,
                              args: <Self as CalledIn>::Args| {
            let res = <<Self as CalledIn>::State as Message>::method_a(state, args.0);
            Some(Json1(res))
        };
        <Self as CalledIn>::called_in(method_wrapper);
    }
}
// created by macro
impl CalledIn for message_concrete::CalledInMethodB<Abc> {
    type State = Abc;
    type Args = Json2<String, bool>;
    type Return = Json1<bool>;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut <Self as CalledIn>::State,
                              args: <Self as CalledIn>::Args| {
            let res = <<Self as CalledIn>::State as Message>::method_b(state, args.0, args.1);
            Some(Json1(res))
        };
        <Self as CalledIn>::called_in(method_wrapper);
    }
}

// must be created by macro (or by hand)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_a() {
    #[allow(unused_imports)]
    message_concrete::CalledInMethodA::<Abc>::exposed_called_in()
}

// must be created by macro (or by hand)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_b() {
    #[allow(unused_imports)]
    message_concrete::CalledInMethodB::<Abc>::exposed_called_in()
}
