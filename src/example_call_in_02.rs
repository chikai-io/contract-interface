//! Example of defining an contract to be called by consumer contracts,
//! (making use of generics for demonstration purposes).
//! That is, the usage of generics in this example are pointless.
//!
//! (the consumer contracts still need to define their CallOut's)

use super::CalledIn;
use crate::args::{Json1, Json2};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, // PanicOnDefault,
};

// #[CalledIn]
/// (Original Message documentation)
pub trait Message<M> {
    /// (Original method_a documentation)
    fn method_a(&mut self, my_string: String) -> bool;

    /// (Original method_b documentation)
    fn method_b<Y, Z>(&mut self, my_string: String, my_y: Y) -> Z;
}
// created by macro
pub mod message_concrete {
    use std::marker::PhantomData;

    ///
    ///
    /// (Original method_a documentation)
    #[allow(dead_code)]
    pub struct CalledInMethodA<M, State> {
        trait_param: PhantomData<M>,
        state_param: PhantomData<State>,
    }

    ///
    ///
    /// (Original method_b documentation)
    #[allow(dead_code)]
    pub struct CalledInMethodB<M, Y, Z, State> {
        trait_param: PhantomData<M>,
        method_param: (PhantomData<Y>, PhantomData<Z>),
        state_param: PhantomData<State>,
    }
}

// specific
/// (Original Abc documentation)
#[near_bindgen]
// TODO: PanicOnDefault doesn't work with generics
// #[derive(PanicOnDefault)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Abx<X> {
    a: u8,
    b: u16,
    c: X,
}

// TODO: PanicOnDefault doesn't work with generics
impl<X> Default for Abx<X> {
    fn default() -> Self {
        near_sdk::env::panic_str("The contract is not initialized");
    }
}

// specific (where the CalledIn "derive" must happen)
// #[CalledIn]
impl<X, M> Message<M> for Abx<X> {
    fn method_a(&mut self, _my_string: String) -> bool {
        todo!()
    }
    fn method_b<Y, Z>(&mut self, _my_string: String, _my_y: Y) -> Z {
        todo!()
    }
}
// created by macro
impl<X, M> CalledIn for message_concrete::CalledInMethodA<M, Abx<X>>
where
    X: crate::args::borsh::BorshSerDe + Default,
{
    type State = Abx<X>;
    type Args = Json1<String>;
    type Return = Json1<bool>;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut <Self as CalledIn>::State,
                              args: <Self as CalledIn>::Args| {
            let res = <<Self as CalledIn>::State as Message<M>>::method_a(state, args.0);
            Some(Json1(res))
        };
        <Self as CalledIn>::called_in(method_wrapper);
    }
}
// created by macro
impl<X, M, Y, Z> CalledIn for message_concrete::CalledInMethodB<M, Y, Z, Abx<X>>
where
    X: crate::args::borsh::BorshSerDe + Default,
    Y: crate::args::json::SerDe,
    Z: crate::args::json::SerDe,
{
    type State = Abx<X>;
    type Args = Json2<String, Y>;
    type Return = Json1<Z>;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut <Self as CalledIn>::State,
                              args: <Self as CalledIn>::Args| {
            let res =
                <<Self as CalledIn>::State as Message<M>>::method_b::<Y, Z>(state, args.0, args.1);
            Some(Json1(res))
        };
        <Self as CalledIn>::called_in(method_wrapper);
    }
}

// must be created by hand (struct and trait must be specialized)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_a_u32() {
    pub type SpecializedMethodA = message_concrete::CalledInMethodA<(), Abx<u32>>;
    #[allow(unused_imports)]
    SpecializedMethodA::exposed_called_in()
}

// must be created by hand (struct and trait must be specialized)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_b_u16() {
    pub type SpecializedMethodB = message_concrete::CalledInMethodB<(), u8, bool, Abx<u16>>;
    #[allow(unused_imports)]
    SpecializedMethodB::exposed_called_in()
}
