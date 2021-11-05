use contract_standards as cs;

cs::extern_impl_fungible_token_core_lensed!(
    impl_mod = cs::fungible_token::core::impl_fungible_token_core_lensed,
    <_State> = crate::Struct3A,
    <_LensTarget> = cs::ft::FungibleToken
);
