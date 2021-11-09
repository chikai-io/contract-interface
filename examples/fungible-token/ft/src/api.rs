use crate::Contract;
use contract_standards::cs;

crate::macros::extern_impl_contract!(
    //
    stored_type = Contract,
    impl_mod = crate::impl_contract
);

cs::macros::ft::extern_impl_fungible_token!(
    //
    stored_type = Contract,
    state_access = state.token,
    impl_mod = cs::ft::core_impl::impl_fungible_token
);

crate::macros::extern_impl_ft_metadata_provider!(
    //
    stored_type = Contract,
    impl_mod = crate::metadata_provider::impl_ft_metadata_provider
);

crate::macros::extern_impl_resolver!(
    //
    stored_type = Contract,
    impl_mod = crate::resolver::impl_resolver
);

crate::macros::extern_impl_storage!(
    //
    stored_type = Contract,
    impl_mod = crate::storage::impl_storage
);
