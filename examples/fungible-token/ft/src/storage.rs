use crate::Contract;
use contract_interface::contract;
use contract_standards::cs;
use near_sdk::json_types::U128;
use near_sdk::{log, AccountId, Balance};

pub trait OnAccountClosed {
    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }
}

impl OnAccountClosed for Contract {}

#[contract(
    //
    mod = "impl_storage", 
    trait = "cs::storage::storage_management"
)]
impl cs::storage::StorageManagement for Contract {
    #[contract(payable)]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> cs::storage::StorageBalance {
        self.token.storage_deposit(account_id, registration_only)
    }

    #[contract(payable)]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> cs::storage::StorageBalance {
        self.token.storage_withdraw(amount)
    }

    #[contract(payable, private = false)]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        #[allow(unused_variables)]
        if let Some((account_id, balance)) = self.token.internal_storage_unregister(force) {
            self.on_account_closed(account_id, balance);
            true
        } else {
            false
        }
    }

    fn storage_balance_bounds(&self) -> cs::storage::StorageBalanceBounds {
        self.token.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<cs::storage::StorageBalance> {
        self.token.storage_balance_of(account_id)
    }
}
