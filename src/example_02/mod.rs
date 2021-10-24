//! Example of defining an contract to be called by consumer contracts.
//! (the consumer contracts still need to define their CallOut's)
//! 

pub mod api;

use crate as interface;
use interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct Struct {
    a: u8,
    b: u16,
    c: u32,
}

/// (Trait3 Doc).
#[contract]
pub trait Trait3< //
        'trait_lt,
        TraitType: std::fmt::Debug,
        const TRAIT_CONST: bool
>: Clone
where 
TraitType: near_sdk::serde::Serialize + near_sdk::serde::de::DeserializeOwned + Sized + Default
{
    /// (TRAIT_INTERNAL_CONST Doc)
    const TRAIT_INTERNAL_CONST: bool;

    /// (TraitInternalTypeA Doc)
    type TraitInternalTypeA: Clone + near_sdk::serde::de::DeserializeOwned;

    /// (TraitInternalTypeB Doc)
    type TraitInternalTypeB;

    /// (method_a Doc)
    fn method_a< //
        'method_lt,
        MethodTypeA,
        MethodTypeB
    >(
        &mut self,
        _my_string: String,
        #[contract(attr(serde(bound = "")))]
        _my_m: TraitType,
        // (_my_y, _my_bool): (MethodTypeA, bool),
        // _my_y2: &'method_lt MethodTypeA,

        // TODO: attribute to implicitly consider
        // Self (aka. _State) as implementing the trait itself
        _my_type_a: Self::TraitInternalTypeA,
    ) -> MethodTypeB
    where
        TraitType: 'trait_lt,
        MethodTypeA: Default,
        MethodTypeB: Default + near_sdk::serde::Serialize,
    {
        unimplemented!()
    }
}

/// (Impl Trait3 for Struct Doc).
#[contract(mod = "struct_2", trait = "trait_3")]
impl<
        //
        'trait_lt,
        TraitType: std::fmt::Debug,
        // TraitType: std::fmt::Debug + near_sdk::serde::de::DeserializeOwned,
        const TRAIT_CONST: bool,
    >
    Trait3<
        //
        'trait_lt,
        TraitType,
        TRAIT_CONST,
    > for Struct //

    where TraitType: near_sdk::serde::Serialize + near_sdk::serde::de::DeserializeOwned + Default,
    
{
    /// (TRAIT_INTERNAL_CONST Doc)
    const TRAIT_INTERNAL_CONST: bool = true;

    /// (TraitInternalTypeA Doc)
    type TraitInternalTypeA = ();

    /// (TraitInternalTypeB Doc)
    type TraitInternalTypeB = ();
    /// (Impl method_a Doc).
    fn method_a<
        //
        'method_lt,
        MethodTypeA,
        MethodTypeB,
    >(
        &mut self,
        _my_string: String,
        _my_m: TraitType,
        _my_type_a: Self::TraitInternalTypeA,
    ) -> MethodTypeB
    where
        Self: Trait3<'trait_lt, TraitType, TRAIT_CONST>,
        TraitType: 'trait_lt,
        MethodTypeA: Default,
        MethodTypeB: Default + near_sdk::serde::Serialize,
    {
        unimplemented!()
    }
}




