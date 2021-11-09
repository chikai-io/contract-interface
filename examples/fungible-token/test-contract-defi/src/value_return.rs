use crate::DeFi;
use contract_interface::contract;
use near_sdk::json_types::U128;
use near_sdk::{log, Balance, PromiseOrValue};

/// Defining cross-contract interface. This allows to create a new promise.
#[contract]
pub trait ValueReturnTrait {
    fn value_please(&self, amount_to_return: String) -> PromiseOrValue<U128>;
}

#[contract(
    //
    mod = "impl_value_return",
    trait = "value_return_trait"
)]
impl ValueReturnTrait for DeFi {
    fn value_please(&self, amount_to_return: String) -> PromiseOrValue<U128> {
        log!("in value_please, amount_to_return = {}", amount_to_return);
        let amount: Balance = amount_to_return.parse().expect("Not an integer");
        PromiseOrValue::Value(amount.into())
    }
}
