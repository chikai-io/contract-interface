use crate::fungible_token::core::{
    fungible_token_core, fungible_token_core_lensed, FungibleTokenCore, FungibleTokenCoreLensed,
};
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

use crate::fungible_token::resolver::FungibleTokenResolver;
use crate::fungible_token::FungibleToken;
use near_sdk::borsh::{self};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    assert_one_yocto, env, log, require, AccountId, Balance, Gas, IntoStorageKey, PanicOnDefault,
    PromiseOrValue, PromiseResult, StorageUsage,
};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);

const NO_DEPOSIT: Balance = 0;

/*
#[contract(
    mod = "pause_fungible_token_lensed",
    trait = "fungible_token_core_lensed"
)]
impl<_State, _LensTarget> FungibleTokenCoreLensed<_LensTarget> for _State
where
    _LensTarget: FungibleTokenCore + Default + BorshSerialize + BorshDeserialize,
    _State: Lens<Pause<_LensTarget>>,
{
    fn ft_transfer(
        &mut self,
        receiver_id: near_sdk::AccountId,
        amount: near_sdk::json_types::U128,
        memo: Option<String>,
    ) {
        todo!()
    }

    fn ft_transfer_call(
        &mut self,
        receiver_id: near_sdk::AccountId,
        amount: near_sdk::json_types::U128,
        memo: Option<String>,
        msg: String,
    ) -> near_sdk::PromiseOrValue<near_sdk::json_types::U128> {
        todo!()
    }

    fn ft_total_supply(&self) -> near_sdk::json_types::U128 {
        todo!()
    }

    fn ft_balance_of(&self, account_id: near_sdk::AccountId) -> near_sdk::json_types::U128 {
        todo!()
    }
}
*/

/*
#[contract(
    mod = "impl_pause_inheritance",
    trait = "crate::fungible_token::core::fungible_token_core"
)]
impl<T> FungibleTokenCore for T
where
    T: Lens<Pause<FungibleToken>> + Default + BorshSerialize + BorshDeserialize,
{
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        FungibleToken::ft_transfer(Lens::lens_mut(self), receiver_id, amount, memo)
    }

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        FungibleToken::ft_transfer_call(Lens::lens_mut(self), receiver_id, amount, memo, msg)
    }

    fn ft_total_supply(&self) -> U128 {
        FungibleToken::ft_total_supply(Lens::lens(self))
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        FungibleToken::ft_balance_of(Lens::lens(self), account_id)
    }
}
*/
