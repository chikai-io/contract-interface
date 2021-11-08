//! Contains `extern "C"` functions, manually created.  
//! The created functions can also be seen in the documentation.
//!
//! See also [`super::api`] for an example of how to automatically
//! create those functions.

use super::impl_trait_1;
use contract_interface::ci;

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_mut_manual() {
    use ci::ServeRefMut;
    impl_trait_1::method_ref_mut::Serve::extern_serve(|state| state);
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_manual() {
    use ci::ServeRef;
    impl_trait_1::method_ref::Serve::extern_serve(|state| state);
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_owned_manual() {
    use ci::ServeOwned;
    impl_trait_1::method_owned::Serve::extern_serve(|state| state);
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_stateless_manual() {
    use ci::ServeStateless;
    impl_trait_1::method_stateless::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_no_arg_manual() {
    use ci::ServeStateless;
    impl_trait_1::method_no_arg::Serve::extern_serve();
}
