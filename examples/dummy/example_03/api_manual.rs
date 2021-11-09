//! Contains `extern "C"` functions, manually created.  
//! The created functions can also be seen in the documentation.
//!
//! See also [`super::api`] for an example of how to automatically
//! create those functions.

use super::Struct3;
use contract_interface::ci;
use contract_standards::cs;
use cs::ft::core_impl::impl_fungible_token;

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn ft_transfer_manual() {
    use ci::ServeRefMut;
    impl_fungible_token::ft_transfer::Serve::extern_serve::<Struct3>(
        //
        |state| &mut state.token,
    );
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn ft_transfer_call() {
    use ci::ServeRefMut;
    impl_fungible_token::ft_transfer_call::Serve::extern_serve::<Struct3>(
        //
        |state| &mut state.token,
    );
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn ft_total_supply() {
    use ci::ServeRef;
    impl_fungible_token::ft_total_supply::Serve::extern_serve::<Struct3>(
        //
        |state| &state.token,
    );
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn ft_balance_of() {
    use ci::ServeRef;
    impl_fungible_token::ft_balance_of::Serve::extern_serve::<Struct3>(
        //
        |state| &state.token,
    );
}
