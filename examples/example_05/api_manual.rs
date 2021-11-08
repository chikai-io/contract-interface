//! Contains `extern "C"` functions, manually created.  
//! The created functions can also be seen in the documentation.
//!
//! See also [`super::api`] for an example of how to automatically
//! create those functions.

use contract_interface::ci;

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_init_manual() {
    use ci::ServeStatelessInit;
    super::impl_trait_5::method_init::Serve::extern_serve::<super::Struct5>();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_payable_manual() {
    use ci::ServeRefMut;
    super::impl_trait_5::method_payable::Serve::extern_serve::<super::Struct5>(
        |state: &mut super::Struct5| state,
    );
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_private_manual() {
    use ci::ServeStateless;
    super::impl_trait_5::method_private::Serve::extern_serve();
}
