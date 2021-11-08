//! Contains `extern "C"` functions, manually created.  
//! The created functions can also be seen in the documentation.
//!
//! See also [`super::api`] for an example of how to automatically
//! create those functions.

use contract_interface::ci;

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_array_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_array::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_paren_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_paren::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_path_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_path::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_ptr_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_ptr::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_ptr_mut_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_ptr_mut::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_ref_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_ref::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_ref_mut_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_ref_mut::Serve::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn fn_tuple_manual() {
    use ci::ServeStateless;
    super::impl_trait_4::fn_tuple::Serve::extern_serve();
}
