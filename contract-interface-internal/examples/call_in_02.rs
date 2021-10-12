//! Example of defining an contract to be called by consumer contracts,
//! (making use of generics for demonstration purposes).
//! That is, the usage of generics in this example are pointless.
//!
//! (the consumer contracts still need to define their CallOut's)

use contract_interface_internal as interface;
use interface::CalledIn;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract, // PanicOnDefault,
    near_bindgen,
};

pub mod arbitrary_mod {
    // #[CalledIn]
    /// (Original Trait documentation)
    pub trait Trait< // 
        'trait_lt, 
        TraitType: std::fmt::Debug, 
        const TRAIT_CONST: bool
    >: Clone {
        const TRAIT_INTERNAL_CONST: bool;
        type TraitInternalTypeA: Clone;
        type TraitInternalTypeB;

        /// (Original method_a documentation)
        fn method_a< //
            'method_lt, 
            MethodTypeA, 
            MethodTypeB
        >(
            &mut self,
            _my_string: String,
            _my_m: TraitType,
            (_my_y, _my_bool): (MethodTypeA, bool),
            _my_y2: &'method_lt MethodTypeA,
            _my_type_a: Self::TraitInternalTypeA,
        ) -> MethodTypeB
        where
            TraitType: 'trait_lt,
            MethodTypeA: Default,
        {
            unimplemented!()
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
        #[allow(non_camel_case_types)]
        pub mod method_a {
            use std::marker::PhantomData;

            ///
            ///
            /// (Original method_a documentation)
            #[derive(near_sdk::serde::Deserialize)]
            #[serde(crate = "near_sdk::serde")]
            pub struct 
            Args< //
                'trait_lt,
                'method_lt, 
                _State,
                TraitType, 
                Self_TraitInternalTypeA, 
                MethodTypeA
            >
            where
                &'method_lt MethodTypeA: near_sdk::serde::de::DeserializeOwned,
                TraitType: 'trait_lt,
                MethodTypeA: Default,
            {
                pub my_string: String,
                pub my_m: TraitType,
                pub my_y: MethodTypeA,
                pub my_bool: bool,
                pub my_y2: &'method_lt MethodTypeA,
                pub my_type_a: Self_TraitInternalTypeA,
                #[serde(skip)]
                pub _state: PhantomData<_State>,
                #[serde(skip)]
                pub _phantom: ArgPhantom<'trait_lt>,
            }

            #[derive(Default)]
            pub struct ArgPhantom<'trait_lt> {
                _trait_lifetimes: (std::marker::PhantomData<&'trait_lt ()>,),
            }

            ///
            ///
            /// (Original method_a documentation)
            pub type Return<Z> = Z;

            ///
            ///
            /// (Original method_a documentation)
            pub struct CalledIn< //
                'trait_lt,
                'method_lt,
                TraitType,
                Self_TraitInternalTypeA,
                Self_TraitInternalTypeB,
                MethodTypeA,
                MethodTypeB,
                State,
                const TRAIT_CONST: bool,
            > {
                _state_param: (std::marker::PhantomData<State>,),
                _other_param: StatelessCalledIn< // 
                    'trait_lt,
                    'method_lt,
                    TraitType,
                    Self_TraitInternalTypeA,
                    Self_TraitInternalTypeB,
                    MethodTypeA,
                    MethodTypeB,
                    TRAIT_CONST
                >,
            }

            #[derive(Default)]
            pub struct StatelessCalledIn< //
                'trait_lt,
                'method_lt,
                TraitType,
                Self_TraitInternalTypeA,
                Self_TraitInternalTypeB,
                MethodTypeA,
                MethodTypeB,
                const TRAIT_CONST: bool,
            > {
                _trait_lifetimes: (
                    std::marker::PhantomData<&'trait_lt ()>,
                    std::marker::PhantomData<&'method_lt ()>,
                ),
                _trait_param: (std::marker::PhantomData<TraitType>,),
                _trait_types: (
                    std::marker::PhantomData<Self_TraitInternalTypeA>,
                    std::marker::PhantomData<Self_TraitInternalTypeB>,
                ),
                _method_param: (std::marker::PhantomData<MethodTypeA>, std::marker::PhantomData<MethodTypeB>),
            }
        }
    }
}

// specific
/// (Original Struct documentation)
#[near_bindgen]
// TODO: PanicOnDefault doesn't work with generics
// #[derive(PanicOnDefault)]
#[derive(BorshDeserialize, BorshSerialize, Clone)]
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
impl< //
    'trait_lt, 
    X, 
    TraitType: std::fmt::Debug, 
    const TRAIT_CONST: bool
>
    arbitrary_mod::Trait<'trait_lt, TraitType, TRAIT_CONST> //
    for Struct<X>
where
    Struct<X>: Clone,
{
    const TRAIT_INTERNAL_CONST: bool = true;
    type TraitInternalTypeA = TypeA;
    type TraitInternalTypeB = TypeB;

    fn method_a< //
        'method_lt, 
        MethodTypeA, 
        MethodTypeB
    >(
        &mut self,
        _my_string: String,
        _my_m: TraitType,
        (_my_y, _my_bool): (MethodTypeA, bool),
        _my_y2: &'method_lt MethodTypeA,
        _my_type_a: Self::TraitInternalTypeA,
    ) -> MethodTypeB
    where
        TraitType: 'trait_lt,
    {
        unimplemented!()
    }
}
// created by macro
pub mod trait_method_a_impl {
    use super::*;
    impl< //
        'trait_lt,
        'method_lt, 
        X, 
        TraitType, 
        MethodTypeA, 
        MethodTypeB, 
        const TRAIT_CONST: bool
    >
        crate::CalledIn<interface::Json, interface::Json>
        for arbitrary_mod::_trait::method_a::CalledIn< //
            'trait_lt,
            'method_lt,
            TraitType,
            TypeA,
            TypeB,
            MethodTypeA,
            MethodTypeB,
            Struct<X>,
            TRAIT_CONST,
        >
    where
        // state
        X: near_sdk::borsh::BorshSerialize + near_sdk::borsh::BorshDeserialize + Default,
        // Self bonds from trait
        Struct<X>: Clone,
        // arg
        TraitType: near_sdk::serde::de::DeserializeOwned
            // extra from the bound on trait definition
            + std::fmt::Debug
            // bound form method
            + 'trait_lt
            ,
        // arg
        MethodTypeA: near_sdk::serde::de::DeserializeOwned
            +
            // extra from the bound on method definition
            Default
            // extra because of `&'method_lt MethodTypeA`
            + 'method_lt,
        &'method_lt MethodTypeA: near_sdk::serde::de::DeserializeOwned,
        // return
        MethodTypeB: near_sdk::serde::Serialize,
    {
        type State = Struct<X>;
        type Args =
            arbitrary_mod::_trait::method_a::Args<'trait_lt, 'method_lt, Self::State, TraitType, TypeA, MethodTypeA>;
        type Return = arbitrary_mod::_trait::method_a::Return<MethodTypeB>;
        type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

        fn exposed_called_in() {
            let method_wrapper = |state: &mut Self::State, args: Self::Args| {
                let res = <Self::State as arbitrary_mod::Trait<TraitType, TRAIT_CONST>>::method_a::<
                    MethodTypeA,
                    MethodTypeB,
                >(
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
        unimplemented!()
    }
}
impl<'x, 'de> near_sdk::serde::de::Deserialize<'de> for &'x MyU8 {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: near_sdk::serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}

// must be created by hand (struct and trait must be specialized)
pub type A<'trait_lt, 'method_lt> = arbitrary_mod::_trait::method_a::CalledIn< //
    'trait_lt,
    'method_lt,
    (),
    TypeA,
    TypeB,
    MyU8,
    bool,
    Struct<u16>,
    true,
>;
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
