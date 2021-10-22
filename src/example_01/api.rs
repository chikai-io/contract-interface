use super::{interface, trait4, Struct};

// must be created by macro (or by hand)

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_mut_manual() {
    use interface::CalledInRefMut;
    trait4::method_ref_mut::CalledIn::<Struct>::exposed_called_in();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_ref_manual() {
    use interface::CalledInRef;
    trait4::method_ref::CalledIn::<Struct>::exposed_called_in();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_owned_manual() {
    use interface::CalledInOwned;
    trait4::method_owned::CalledIn::<Struct>::exposed_called_in();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_stateless_manual() {
    use interface::CalledInStateless;
    trait4::method_stateless::CalledIn::<Struct>::exposed_called_in();
}

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_no_arg_manual() {
    use interface::CalledInStateless;
    trait4::method_no_arg::CalledIn::<Struct>::exposed_called_in();
}
