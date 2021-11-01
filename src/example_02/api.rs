//! Contains `extern "C"` functions, created by macros.  
//! The created functions can be seen in the documentation.

crate::macros::extern_struct_2!(
    impl_mod = super::struct_2,
    <TraitType> = u8,
    <TRAIT_CONST> = true,
    method_a <MethodTypeA> = u8,
    method_a <MethodTypeB> = u8
);
