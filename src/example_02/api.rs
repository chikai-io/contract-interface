//! Contains `extern "C"` functions, created by macros,
//! where those macros were created automatically.  
//! Both the macros and the created functions can be seen in the
//! geenrated documentation of this project.
//!
//! When creating those functions, all generic parameters must be
//! known at compile time, so the generated macro takes this is account
//! and require those definitions.

// it first requires an `impl_mod` to base itself into, where the
// state and potentially other generics might get defined.
//
// then it requires the trait generics to be defined,
// which are the <> at the beggining of the line for each parameter,
//
// then it requires each method generics to be defined,
// which are the method name followed by an <> for each parameter
crate::macros::extern_impl_trait_2!(
    impl_mod = super::impl_trait_2,
    <TraitType> = u8,
    <TRAIT_CONST> = true,
    method_a <MethodTypeA> = u8,
    method_a <MethodTypeB> = u8
);
