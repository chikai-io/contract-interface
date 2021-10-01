//! Example of defining an contract to be called by consumer contracts,
//! (making use of generics for demonstration purposes).
//! That is, the usage of generics in this example are pointless.
//!
//! (the consumer contracts still need to define their CallOut's)

use super::CalledIn;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract, // PanicOnDefault,
    near_bindgen,
};

pub mod arbitrary_mod {
    // #[CalledIn]
    /// (Original Trait documentation)
    pub trait Trait<'a, M: std::fmt::Debug> {
        type MyTypeA: Clone;
        type MyTypeB;

        /// (Original method_a documentation)
        fn method_a<'b, Y, Z>(
            &mut self,
            _my_string: String,
            _my_m: M,
            (_my_y, _my_bool): (Y, bool),
            _my_y2: &'b Y,
            _my_type_a: Self::MyTypeA,
        ) -> Z
        where
            M: 'a,
            Y: Default,
        {
            todo!()
        }
    }

    // created by macro
    ///
    ///
    /// (Original Trait documentation)
    pub mod _trait {

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
            pub struct Args<'b, M, StateMyTypeA, Y>
            where
                &'b Y: near_sdk::serde::de::DeserializeOwned,
            {
                pub my_string: String,
                pub my_m: M,
                pub my_y: Y,
                pub my_bool: bool,
                pub my_y2: &'b Y,
                pub my_type_a: StateMyTypeA,
            }

            ///
            ///
            /// (Original method_a documentation)
            pub type Return<Z> = Z;

            ///
            ///
            /// (Original method_a documentation)
            pub struct CalledIn<'b, M, MyTypeA, MyTypeB, Y, Z, State> {
                _trait_lifetimes: PhantomData<&'b ()>,
                _trait_param: PhantomData<M>,
                _types_param: (PhantomData<MyTypeA>, PhantomData<MyTypeB>),
                _method_param: (PhantomData<Y>, PhantomData<Z>),
                _state_param: PhantomData<State>,
            }
        }
    }
}

// specific
/// (Original Struct documentation)
#[near_bindgen]
// TODO: PanicOnDefault doesn't work with generics
// #[derive(PanicOnDefault)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Struct<X> {
    a: u8,
    b: u16,
    x: X,
}

// TODO: PanicOnDefault doesn't work with generics
impl<X> Default for Struct<X> {
    fn default() -> Self {
        near_sdk::env::panic_str("The contract is not initialized");
    }
}

pub type TypeA = u32;
pub type TypeB = u64;

// specific (where the CalledIn "derive" must happen)
// #[CalledIn]
impl<'a, X, M: std::fmt::Debug> arbitrary_mod::Trait<'a, M> for Struct<X> {
    type MyTypeA = TypeA;
    type MyTypeB = TypeB;

    fn method_a<'b, Y, Z>(
        &mut self,
        _my_string: String,
        _my_m: M,
        (_my_y, _my_bool): (Y, bool),
        _my_y2: &'b Y,
        _my_type_a: Self::MyTypeA,
    ) -> Z
    where
        M: 'a,
    {
        todo!()
    }
}
// created by macro
pub mod trait_method_a_impl {
    use super::*;
    impl<'b, X, M, Y, Z> crate::interface::CalledIn<crate::args::Json, crate::args::Json>
        for arbitrary_mod::_trait::method_a::CalledIn<'b, M, TypeA, TypeB, Y, Z, Struct<X>>
    where
        // state
        X: near_sdk::borsh::BorshSerialize + near_sdk::borsh::BorshDeserialize + Default,
        // arg
        M: near_sdk::serde::de::DeserializeOwned
            // extra from the bound on trait definition
            + std::fmt::Debug,
        // arg
        Y: near_sdk::serde::de::DeserializeOwned
            +
            // extra from the bound on method definition
            Default
            // extra because of `&'b Y`
            + 'b,
        &'b Y: near_sdk::serde::de::DeserializeOwned,
        // return
        Z: near_sdk::serde::Serialize,
    {
        type State = Struct<X>;
        type Args = arbitrary_mod::_trait::method_a::Args<'b, M, TypeA, Y>;
        type Return = arbitrary_mod::_trait::method_a::Return<Z>;
        type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

        fn exposed_called_in() {
            let method_wrapper = |state: &mut Self::State, args: Self::Args| {
                let res = <Self::State as arbitrary_mod::Trait<M>>::method_a::<Y, Z>(
                    state,
                    args.my_string,
                    args.my_m,
                    (args.my_y, args.my_bool),
                    args.my_y2,
                    args.my_type_a,
                );
                Some(res)
            };
            Self::called_in(method_wrapper);
        }
    }
}

#[derive(Default)]
pub struct MyU8(pub u8);
impl<'de> near_sdk::serde::de::Deserialize<'de> for MyU8 {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: near_sdk::serde::Deserializer<'de>,
    {
        todo!()
    }
}
impl<'x, 'de> near_sdk::serde::de::Deserialize<'de> for &'x MyU8 {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: near_sdk::serde::Deserializer<'de>,
    {
        todo!()
    }
}

// must be created by hand (struct and trait must be specialized)
pub type A<'b> =
    arbitrary_mod::_trait::method_a::CalledIn<'b, (), TypeA, TypeB, MyU8, bool, Struct<u16>>;
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn my_method_b_u16() {
    #[allow(unused_imports)]
    A::exposed_called_in()
}
// expose!(my_method_b_u16, SpecializedMethodB)

// to get that specialized method's arguments/return, you can do:
// pub type BArgs = <A as CalledIn<crate::args::Json, crate::args::Json>>::Args;
// pub type BReturn = <A as CalledIn<crate::args::Json, crate::args::Json>>::Return;
// TODO: ^check if those informations are in fact "usable"
// eg. if it's possible to check it against a hand-rolled concrete type
