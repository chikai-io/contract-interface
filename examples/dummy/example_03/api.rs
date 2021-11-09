//! Contains `extern "C"` functions, created by macros,
//! where those macros were created automatically.  
//! Both the macros and the created functions can be seen in the
//! geenrated documentation of this project.
//!
//! See also [`super::api_manual`] for an example of how to
//! manually create those functions, without using any macros.

use contract_standards::cs;

// this uses `state_access` to be able to access the FungibleToken
// from within a `Struct3`
cs::extern_impl_fungible_token!(
    stored_type = super::Struct3,
    state_access = state.token,
    impl_mod = cs::ft::core_impl::impl_fungible_token
);
