use crate::Contract;
use contract_interface::contract;
use contract_standards::cs;
use near_sdk::json_types::U128;
use near_sdk::{log, AccountId, Balance};

pub trait OnTokensBurned {
    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

impl OnTokensBurned for Contract {}

#[contract(
    //
    mod = "impl_resolver",
    trait = "cs::ft::resolver::fungible_token_resolver"
)]
impl cs::ft::resolver::FungibleTokenResolver for Contract {
    #[contract(private)]
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        let (used_amount, burned_amount) =
            self.token
                .internal_ft_resolve_transfer(&sender_id, receiver_id, amount);
        if burned_amount > 0 {
            self.on_tokens_burned(sender_id, burned_amount);
        }
        used_amount.into()
    }
}
