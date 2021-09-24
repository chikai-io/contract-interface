use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas,
    PanicOnDefault, PromiseOrValue,
};

use near_sdk::serde::{de::DeserializeOwned, Deserialize, Serialize};
// use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

pub mod args;
mod ex;
pub mod interface;
pub mod stand;

pub use args::ArgsType;
pub use interface::Call;
