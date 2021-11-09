/*!
Some hypothetical DeFi contract that will do smart things with the transferred tokens
*/
use contract_interface::contract;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};

#[macro_use]
pub mod value_return;
#[macro_use]
pub mod receiver;

#[cfg(feature = "serve")]
pub mod api;

#[cfg(not(target_arch = "wasm32"))]
pub mod marshall;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DeFi {
    fungible_token_account_id: AccountId,
}

#[contract]
pub trait DefiBehaviour {
    #[contract(init())]
    fn new(fungible_token_account_id: AccountId) -> Self;
}

#[contract(
    //
    mod = "impl_defi", 
    trait = "defi_behaviour"
)]
impl DefiBehaviour for DeFi {
    #[contract(init())]
    fn new(fungible_token_account_id: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");
        Self {
            fungible_token_account_id,
        }
    }
}

pub mod macros {
    pub use extern_impl_defi;
    pub use extern_impl_receiver;
    pub use extern_impl_value_return;
}
