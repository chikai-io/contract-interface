use crate::DeFi;
use contract_interface::contract;
use contract_standards::cs;
use near_sdk::json_types::U128;
use near_sdk::{env, log, require, AccountId, Balance, Gas, PromiseOrValue};

const NO_DEPOSIT: Balance = 0;
const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = Gas(BASE_GAS + PROMISE_CALL);

#[contract(
    //
    mod = "impl_receiver", 
    trait = "cs::ft::receiver::fungible_token_receiver"
)]
impl cs::ft::receiver::FungibleTokenReceiver for DeFi {
    /// If given `msg: "take-my-money", immediately returns U128::From(0)
    /// Otherwise, makes a cross-contract call to own `value_please` function, passing `msg`
    /// value_please will attempt to parse `msg` as an integer and return a U128 version of it
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Verifying that we were called by fungible token contract that we expect.
        require!(
            env::predecessor_account_id() == self.fungible_token_account_id,
            "Only supports the one fungible token contract"
        );
        log!(
            "in {} tokens from @{} ft_on_transfer, msg = {}",
            amount.0,
            sender_id.as_ref(),
            msg
        );

        type SelfRequest = crate::value_return::value_return_trait::value_please::Request<DeFi>;
        match msg.as_str() {
            "take-my-money" => PromiseOrValue::Value(U128::from(0)),
            _ => SelfRequest::contract(env::current_account_id())
                .args(msg)
                .send_amount(NO_DEPOSIT)
                .prepaid_gas(env::prepaid_gas() - GAS_FOR_FT_ON_TRANSFER)
                .request()
                .into(),
        }
    }
}
