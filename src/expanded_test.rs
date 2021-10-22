use std::marker::PhantomData;

use crate as interface;

use interface::{contract, CalledIn};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};
