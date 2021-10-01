//! Example of defining an contract to be called by consumer contracts.
//! (the consumer contracts still need to define their CallOut's)

use super::CalledIn;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

// #[CalledIn]
/// (Original Trait documentation)
pub trait Trait {
    /// (Original method_a documentation)
    fn method_a(&mut self, my_string: String);

    /// (Original method_b documentation)
    fn method_b(&mut self, my_string: String, my_bool: bool) -> bool;
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
        pub struct Args {
            pub my_string: String,
        }

        ///
        ///
        /// /// (Original method_a documentation)
        pub type Return = ();

        ///
        ///
        /// (Original method_a documentation)
        pub struct CalledIn<State> {
            _trait_param: (),
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
        pub struct Args {
            pub my_string: String,
            pub my_bool: bool,
        }

        ///
        ///
        /// (Original method_b documentation)
        pub type Return = bool;

        ///
        ///
        /// (Original method_b documentation)
        pub struct CalledIn<State> {
            _trait_param: (),
            _method_param: (),
            _state_param: PhantomData<State>,
        }
    }
}

// specific
/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct {
    a: u8,
    b: u16,
    c: u32,
}

// specific (where the CalledIn "derive" must happen)
// #[CalledIn]
impl Trait for Struct {
    fn method_a(&mut self, _my_string: String) {
        todo!()
    }
    fn method_b(&mut self, _my_string: String, _my_bool: bool) -> bool {
        todo!()
    }
}
// created by macro
impl CalledIn<crate::args::Json, crate::args::Json> for _trait::method_a::CalledIn<Struct> {
    type State = Struct;
    type Args = _trait::method_a::Args;
    type Return = _trait::method_a::Return;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
            let () = <Self::State as Trait>::method_a(state, args.my_string);
            None
        };
        Self::called_in(method_wrapper);
    }
}
// created by macro
impl CalledIn<crate::args::Json, crate::args::Json> for _trait::method_b::CalledIn<Struct> {
    type State = Struct;
    type Args = _trait::method_b::Args;
    type Return = _trait::method_b::Return;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
            let res = <Self::State as Trait>::method_b(state, args.my_string, args.my_bool);
            Some(res)
        };
        Self::called_in(method_wrapper);
    }
}

// must be created by macro (or by hand)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_a() {
    #[allow(unused_imports)]
    _trait::method_a::CalledIn::<Struct>::exposed_called_in()
}

// must be created by macro (or by hand)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_b() {
    #[allow(unused_imports)]
    _trait::method_b::CalledIn::<Struct>::exposed_called_in()
}
