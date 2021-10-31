use super::{interface, struct_};

// must be created by macro (or by hand)

// extern 1 function (from a trait)
// extern all functions (from a trait)
//

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
