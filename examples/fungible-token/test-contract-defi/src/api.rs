use crate::DeFi;

crate::macros::extern_impl_defi!(
    //
    stored_type = DeFi,
    impl_mod = crate::impl_defi
);

crate::macros::extern_impl_receiver!(
    //
    stored_type = DeFi,
    impl_mod = crate::receiver::impl_receiver
);

crate::macros::extern_impl_value_return!(
    //
    stored_type = DeFi,
    impl_mod = crate::value_return::impl_value_return
);
