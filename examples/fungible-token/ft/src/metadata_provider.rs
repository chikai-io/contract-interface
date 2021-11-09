use crate::Contract;
use contract_interface::contract;
use contract_standards::cs;

#[contract(
    //
    mod = "impl_ft_metadata_provider",
    trait = "cs::ft::metadata::fungible_token_metadata_provider"
)]
impl cs::ft::metadata::FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> cs::ft::metadata::FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}
