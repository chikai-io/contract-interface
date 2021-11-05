#![allow(unused_variables)]

pub mod api;
pub mod client;

use ci::{contract, Lens};
use contract_interface::ci;
use contract_standards::cs;
use cs::fungible_token::FungibleToken;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault,
};

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct3A {
    token: FungibleToken,
}

impl Lens<FungibleToken> for Struct3A {
    fn lens(&self) -> &FungibleToken {
        &self.token
    }

    fn lens_mut(&mut self) -> &mut FungibleToken {
        &mut self.token
    }
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct3B {
    a: Struct3A,
}

impl Lens<Struct3A> for Struct3B {
    fn lens(&self) -> &Struct3A {
        &self.a
    }

    fn lens_mut(&mut self) -> &mut Struct3A {
        &mut self.a
    }
}

// impl Lens<Struct3, FungibleToken> for L {
//     fn with_ref<V, F>(&self, data: &Struct3, f: F) -> V
//     where
//         F: FnOnce(&FungibleToken) -> V,
//     {
//         f(&data.token)
//     }

//     fn with_mut<V, F>(&self, data: &mut Struct3, f: F) -> V
//     where
//         F: FnOnce(&mut FungibleToken) -> V,
//     {
//         f(&mut data.token)
//     }
// }
