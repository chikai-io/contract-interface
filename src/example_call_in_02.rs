//! Example of defining an contract to be called by consumer contracts,
//! (making use of generics for demonstration purposes).
//! That is, the usage of generics in this example are pointless.
//!
//! (the consumer contracts still need to define their CallOut's)

use super::CalledIn;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, // PanicOnDefault,
};

// #[CalledIn]
/// (Original Message documentation)
pub trait Message<M> {
    /// (Original method_a documentation)
    fn method_a(&mut self, my_string: String);

    /// (Original method_b documentation)
    fn method_b<Y, Z>(&mut self, my_string: String, my_y: Y) -> Z;
}
// created by macro
///
///
/// (Original Message documentation)
pub mod message_concrete {

    ///
    ///
    /// (Original method_a documentation)
    pub mod method_a {
        use near_sdk::serde::Deserialize;
        use std::marker::PhantomData;

        ///
        ///
        /// (Original method_a documentation)
        #[derive(Deserialize)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Args {
            pub my_string: String,
        }

        ///
        ///
        /// (Original method_a documentation)
        pub type Return = ();

        ///
        ///
        /// (Original method_a documentation)
        #[allow(dead_code)]
        pub struct CalledIn<M, State> {
            _trait_param: PhantomData<M>,
            _method_param: (),
            _state_param: PhantomData<State>,
        }
    }

    ///
    ///
    /// (Original method_b documentation)
    pub mod method_b {
        use near_sdk::serde::Deserialize;
        use std::marker::PhantomData;

        ///
        ///
        /// (Original method_b documentation)
        #[derive(Deserialize)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Args<Y> {
            pub my_string: String,
            pub my_y: Y,
        }

        ///
        ///
        /// (Original method_b documentation)
        pub type Return<Z> = Z;

        ///
        ///
        /// (Original method_b documentation)
        pub struct CalledIn<M, Y, Z, State> {
            _trait_param: PhantomData<M>,
            _method_param: (PhantomData<Y>, PhantomData<Z>),
            _state_param: PhantomData<State>,
        }
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
    x: X,
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
    fn method_a(&mut self, _my_string: String) {
        todo!()
    }
    fn method_b<Y, Z>(&mut self, _my_string: String, _my_y: Y) -> Z {
        todo!()
    }
}
// created by macro
impl<X, M> CalledIn<crate::args::Json, crate::args::Json>
    for message_concrete::method_a::CalledIn<M, Abx<X>>
where
    X: near_sdk::borsh::BorshSerialize + near_sdk::borsh::BorshDeserialize + Default,
{
    type State = Abx<X>;
    type Args = message_concrete::method_a::Args;
    type Return = message_concrete::method_a::Return;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
            let () = <Self::State as Message<M>>::method_a(state, args.my_string);
            None
        };
        Self::called_in(method_wrapper);
    }
}
// created by macro
impl<X, M, Y, Z> CalledIn<crate::args::Json, crate::args::Json>
    for message_concrete::method_b::CalledIn<M, Y, Z, Abx<X>>
where
    X: near_sdk::borsh::BorshSerialize + near_sdk::borsh::BorshDeserialize + Default,
    Y: near_sdk::serde::de::DeserializeOwned,
    Z: near_sdk::serde::Serialize,
{
    type State = Abx<X>;
    type Args = message_concrete::method_b::Args<Y>;
    type Return = message_concrete::method_b::Return<Z>;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
            let res =
                <Self::State as Message<M>>::method_b::<Y, Z>(state, args.my_string, args.my_y);
            Some(res)
        };
        Self::called_in(method_wrapper);
    }
}

// must be created by hand (struct and trait must be specialized)
pub type SpecializedA = message_concrete::method_a::CalledIn<(), Abx<u32>>;
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn my_method_a_u32() {
    #[allow(unused_imports)]
    SpecializedA::exposed_called_in()
}
// or by some expose!(my_method_a_u32, SpecializedA)

// to get that specialized method's arguments/return, you can do:
pub type SpecializedAArgs = <SpecializedA as CalledIn<crate::args::Json, crate::args::Json>>::Args;
pub type SpecializedAReturn =
    <SpecializedA as CalledIn<crate::args::Json, crate::args::Json>>::Return;
// TODO: ^check if those informations are in fact "usable"
// eg. if it's possible to check it against a hand-rolled concrete type

pub type SpecializedB = message_concrete::method_b::CalledIn<(), u8, bool, Abx<u16>>;
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn my_method_b_u16() {
    #[allow(unused_imports)]
    SpecializedB::exposed_called_in()
}
// expose!(my_method_b_u16, SpecializedMethodB)

pub type SpecializedBArgs = <SpecializedB as CalledIn<crate::args::Json, crate::args::Json>>::Args;
pub type SpecializedBReturn =
    <SpecializedB as CalledIn<crate::args::Json, crate::args::Json>>::Return;
