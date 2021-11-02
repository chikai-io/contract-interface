//! A dummy example which just tries to declare all sort of generic 
//! parameters.
//! 
//! Please check [`api`] to see how to create `extern "C"`
//! functions out of a generic trait impls.

pub mod api;

use contract_interface::contract;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault,
};

/// (Original Struct documentation)
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct Struct2 {
    a: u8,
    b: u16,
    c: u32,
}

/// (Trait2 Doc).
#[contract]
pub trait Trait2< //
        'trait_lt,
        TraitType,
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

/// (Impl Trait2 for Struct Doc).
#[contract(mod = "impl_trait_2", trait = "trait_2")]
impl<
        //
        'trait_lt,
        TraitType,
        const TRAIT_CONST: bool,
    >
    Trait2<
        //
        'trait_lt,
        TraitType,
        TRAIT_CONST,
    > for Struct2 //

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
        Self: Trait2<'trait_lt, TraitType, TRAIT_CONST>,
        TraitType: 'trait_lt,
        MethodTypeA: Default,
        MethodTypeB: Default + near_sdk::serde::Serialize,
    {
        unimplemented!()
    }
}


/// Note:  
/// Because of how `#[macro_use]` works, this module must be
/// at root and must come _after_ the referenced macros
/// are defined.  
/// Ie. This should be the last thing at the root of the project.
pub mod macros {
    pub use extern_impl_trait_2;
}