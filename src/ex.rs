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

#[ext_contract]
pub trait Message {
    #[result_serializer(borsh)]
    fn method_a(&mut self, #[serializer(borsh)] message: String);
}

#[ext_contract]
pub trait ExtStatusMessage {
    fn set_status1(&mut self, message: String);
    fn set_status2(&mut self, message1: String, message2: String);

    fn get_status(&self, account_id: AccountId) -> Option<String>;
}

use crate::Call;

pub fn ex() {
    const SINGLE_CALL_GAS: u64 = 200000000000000;
    let account_id: AccountId = "testing.acc".parse().unwrap();
    let message: String = "ma-oee".to_string();

    use crate::args::Json1;

    // ext_status_message::set_status()

    // let args = JsonArgs(Json1(message.clone()));

    Call::contract("my.contract".parse().unwrap())
        .method(String::from("set_status"))
        .args(Json1(String::from("my_value")))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call();

    // ordered_contract_call(
    //     Json1(message.clone()),
    //     "set_status",
    //     account_id.clone(),
    //     0,
    //     Gas::from(SINGLE_CALL_GAS),
    // );

    // ordered_contract_call(
    //     JsonArgs(Json1(message.clone())),
    //     "set_status",
    //     account_id.clone(),
    //     0,
    //     Gas::from(SINGLE_CALL_GAS),
    // );

    // // contract_call();

    // ext_status_message::set_status1(
    //     message.clone(),
    //     account_id.clone(),
    //     0,
    //     Gas::from(SINGLE_CALL_GAS),
    // );

    // ext_status_message::set_status1(message, account_id.clone(), 0, Gas::from(SINGLE_CALL_GAS))
    //     .then(ext_status_message::get_status(
    //         env::signer_account_id(),
    //         account_id,
    //         0,
    //         Gas::from(SINGLE_CALL_GAS),
    //     ));
}
