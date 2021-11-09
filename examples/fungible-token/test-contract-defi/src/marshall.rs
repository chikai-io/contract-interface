use contract_interface::ToBytes;
use near_sdk::AccountId;

pub use crate::{DeFi, DeFiContract};

impl DeFiContract {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(&self, fungible_token_account_id: AccountId) -> near_sdk::PendingContractTx {
        let args = crate::defi_behaviour::new::Args::<DeFi>::new(fungible_token_account_id);
        let is_view = false;
        near_sdk::PendingContractTx::new_from_bytes(
            self.account_id.clone(),
            "new",
            args.to_bytes().unwrap(),
            is_view,
        )
    }
}
