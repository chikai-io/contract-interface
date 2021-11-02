//! Contains `extern "C"` functions, manually created.  
//! The created functions can also be seen in the documentation.
//!
//! See also [`super::api`] for an example of how to automatically
//! create those functions.

use super::{impl_trait_1, interface};

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_mut_manual() {
    use interface::ServeRefMut;
    impl_trait_1::method_ref_mut::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_manual() {
    use interface::ServeRef;
    impl_trait_1::method_ref::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_owned_manual() {
    use interface::ServeOwned;
    impl_trait_1::method_owned::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_stateless_manual() {
    use interface::ServeStateless;
    impl_trait_1::method_stateless::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_no_arg_manual() {
    use interface::ServeStateless;
    impl_trait_1::method_no_arg::Serve::extern_serve();
}
