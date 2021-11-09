//! Contains `extern "C"` functions, manually created.  
//! The created functions can also be seen in the documentation.
//!
//! See also [`super::api`] for an example of how to automatically
//! create those functions.

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_a_manual() {
    use contract_interface::ServeRefMut;

    type TraitType = u8;
    const TRAIT_CONST: bool = true;
    type MethodTypeA = u8;
    type MethodTypeB = u8;

    super::impl_trait_2::method_a::Serve::<
        //
        MethodTypeA,
        MethodTypeB,
        TraitType,
        TRAIT_CONST,
    >::extern_serve(|state| state);
}
