use contract_interface::ToBytes;
use contract_standards::cs;
use near_sdk::json_types::U128;
use near_sdk::AccountId;

pub use crate::Contract;
pub use crate::ContractContract;

impl ContractContract {
    pub fn new_default_meta(
        &self,
        owner_id: AccountId,
        total_supply: U128,
    ) -> near_sdk::PendingContractTx {
        let args = crate::contract_behaviour::new_default_meta::Args::<Contract>::new(
            owner_id,
            total_supply,
        );
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "new_default_meta",
            args.to_bytes().unwrap(),
            is_view,
        )
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        &self,
        owner_id: AccountId,
        total_supply: U128,
        metadata: cs::ft::metadata::FungibleTokenMetadata,
    ) -> near_sdk::PendingContractTx {
        let args =
            crate::contract_behaviour::new::Args::<Contract>::new(owner_id, total_supply, metadata);
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "new",
            args.to_bytes().unwrap(),
            is_view,
        )
    }
}

impl ContractContract {
    pub fn storage_deposit(
        &self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> near_sdk::PendingContractTx {
        let args = cs::storage::storage_management::storage_deposit::Args::<Contract>::new(
            account_id,
            registration_only,
        );
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "storage_deposit",
            args.to_bytes().unwrap(),
            is_view,
        )
    }

    pub fn storage_withdraw(&self, amount: Option<U128>) -> near_sdk::PendingContractTx {
        let args = cs::storage::storage_management::storage_withdraw::Args::<Contract>::new(amount);
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "storage_withdraw",
            args.to_bytes().unwrap(),
            is_view,
        )
    }

    pub fn storage_unregister(&self, force: Option<bool>) -> near_sdk::PendingContractTx {
        let args =
            cs::storage::storage_management::storage_unregister::Args::<Contract>::new(force);
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "storage_unregister",
            args.to_bytes().unwrap(),
            is_view,
        )
    }

    pub fn storage_balance_bounds(&self) -> near_sdk::PendingContractTx {
        let args = cs::storage::storage_management::storage_balance_bounds::Args::<Contract>::new();
        let is_view = true;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "storage_balance_bounds",
            args.to_bytes().unwrap(),
            is_view,
        )
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> near_sdk::PendingContractTx {
        let args =
            cs::storage::storage_management::storage_balance_of::Args::<Contract>::new(account_id);
        let is_view = true;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "storage_balance_of",
            args.to_bytes().unwrap(),
            is_view,
        )
    }
}

impl ContractContract {
    pub fn ft_transfer(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> near_sdk::PendingContractTx {
        let args =
            cs::ft::core::fungible_token_core::ft_transfer::Args::<cs::ft::FungibleToken>::new(
                receiver_id,
                amount,
                memo,
            );
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "ft_transfer",
            args.to_bytes().unwrap(),
            is_view,
        )
    }

    pub fn ft_transfer_call(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> near_sdk::PendingContractTx {
        let args =
            cs::ft::core::fungible_token_core::ft_transfer_call::Args::<cs::ft::FungibleToken>::new(
                receiver_id,
                amount,
                memo,
                msg,
            );
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "ft_transfer_call",
            args.to_bytes().unwrap(),
            is_view,
        )
    }

    pub fn ft_total_supply(&self) -> near_sdk::PendingContractTx {
        let args =
            cs::ft::core::fungible_token_core::ft_total_supply::Args::<cs::ft::FungibleToken>::new(
            );
        let is_view = true;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "ft_total_supply",
            args.to_bytes().unwrap(),
            is_view,
        )
    }
    pub fn ft_balance_of(&self, account_id: AccountId) -> near_sdk::PendingContractTx {
        let args =
            cs::ft::core::fungible_token_core::ft_balance_of::Args::<cs::ft::FungibleToken>::new(
                account_id,
            );
        let is_view = true;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "ft_balance_of",
            args.to_bytes().unwrap(),
            is_view,
        )
    }
}
