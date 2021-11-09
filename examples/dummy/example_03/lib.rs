//! A dummy example showing a struct that _contains_ a [`FungibleToken`](contract_standards::ft::FungibleToken).
//!
//! Please check [`api`] to see how to create `extern "C"`
//! functions out of methods from that [`Struct3::token`] field.

#![allow(unused_variables)]

use contract_standards::cs;
use cs::ft::FungibleToken;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault,
};

pub mod api;
pub mod api_manual;
// pub mod client;

/// (Struct1 Doc).
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct3 {
    token: FungibleToken,
}
