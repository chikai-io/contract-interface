//! Contains `extern "C"` functions, manually created.  
//! The created functions can also be seen in the documentation.
//!
//! See also [`super::api`] for an example of how to automatically
//! create those functions.

use super::{interface, struct_};

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_mut_manual() {
    use interface::ServeRefMut;
    struct_::method_ref_mut::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_manual() {
    use interface::ServeRef;
    struct_::method_ref::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_owned_manual() {
    use interface::ServeOwned;
    struct_::method_owned::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_stateless_manual() {
    use interface::ServeStateless;
    struct_::method_stateless::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_no_arg_manual() {
    use interface::ServeStateless;
    struct_::method_no_arg::Serve::extern_serve();
}
