#![allow(unused_imports)]

use std::marker::PhantomData;

use crate as interface;

use interface::{contract, Serve};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

use near_sdk::serde;
