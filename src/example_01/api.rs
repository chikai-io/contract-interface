use super::{interface, trait4, Struct};

// must be created by macro (or by hand)

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_mut_manual() {
    use interface::ServeRefMut;
    trait4::method_ref_mut::Serve::<Struct>::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_manual() {
    use interface::ServeRef;
    trait4::method_ref::Serve::<Struct>::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_owned_manual() {
    use interface::ServeOwned;
    trait4::method_owned::Serve::<Struct>::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_stateless_manual() {
    use interface::ServeStateless;
    trait4::method_stateless::Serve::<Struct>::extern_serve();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_no_arg_manual() {
    use interface::ServeStateless;
    trait4::method_no_arg::Serve::<Struct>::extern_serve();
}
