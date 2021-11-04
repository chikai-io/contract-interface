use crate::fungible_token::core::{fungible_token_core, FungibleTokenCore};
use crate::pause::Pause;
use contract_interface::{contract, Final, Identity, Lens, NonLensing};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

// struct ForbidLens;
// impl<Ft> !Lens<Pause<Ft>, Ft> for ForbidLens {}

#[contract(mod = "pause_fungible_token", trait = "fungible_token_core")]
impl<Ft> FungibleTokenCore for Pause<Ft>
where
    Ft: FungibleTokenCore + Default + BorshSerialize + BorshDeserialize,
{
    fn ft_transfer(
        &mut self,
        receiver_id: near_sdk::AccountId,
        amount: near_sdk::json_types::U128,
        memo: Option<String>,
    ) {
        self.panic_if_paused();
        self.inner.ft_transfer(receiver_id, amount, memo)
    }

    fn ft_transfer_call(
        &mut self,
        receiver_id: near_sdk::AccountId,
        amount: near_sdk::json_types::U128,
        memo: Option<String>,
        msg: String,
    ) -> near_sdk::PromiseOrValue<near_sdk::json_types::U128> {
        self.panic_if_paused();
        self.inner.ft_transfer_call(receiver_id, amount, memo, msg)
    }

    fn ft_total_supply(&self) -> near_sdk::json_types::U128 {
        self.inner.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: near_sdk::AccountId) -> near_sdk::json_types::U128 {
        self.inner.ft_balance_of(account_id)
    }
}
