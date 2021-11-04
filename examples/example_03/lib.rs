#![allow(unused_variables)]

pub mod api;
pub mod client;

use contract_interface::{contract, Lens, Lens2};
use contract_standards::fungible_token::FungibleToken;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault,
};

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct3A {
    token: FungibleToken,
}

#[derive(Default)]
pub struct L3AFt;
impl Lens<Struct3A, FungibleToken> for L3AFt {
    fn with_ref<V, F>(data: &Struct3A, f: F) -> V
    where
        F: FnOnce(&FungibleToken) -> V,
    {
        f(&data.token)
    }

    fn with_mut<V, F>(data: &mut Struct3A, f: F) -> V
    where
        F: FnOnce(&mut FungibleToken) -> V,
    {
        f(&mut data.token)
    }
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct3B {
    a: Struct3A,
}

#[derive(Default)]
pub struct L3B3A;
impl Lens<Struct3B, Struct3A> for L3B3A {
    fn with_ref<V, F>(data: &Struct3B, f: F) -> V
    where
        F: FnOnce(&Struct3A) -> V,
    {
        f(&data.a)
    }

    fn with_mut<V, F>(data: &mut Struct3B, f: F) -> V
    where
        F: FnOnce(&mut Struct3A) -> V,
    {
        f(&mut data.a)
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
