//! Contains `extern "C"` functions, created by macros,
//! where those macros were created automatically.  
//! Both the macros and the created functions can be seen in the
//! geenrated documentation of this project.
//!
//! When creating those functions, all generic parameters must be
//! known at compile time, so the generated macro takes this is account
//! and require those definitions.

// besides the `stored_type` and `impl_mod` paths,
// the macro also requires defining every generic types,
// which are the <> at the beggining of the line for each parameter,
//
// and similarly it also requires each method generics to be defined,
// which are the method name followed by an <> for each parameter
crate::macros::extern_impl_trait_2!(
    stored_type = super::Struct2,
    impl_mod = super::impl_trait_2,
    <TraitType> = u8,
    <TRAIT_CONST> = true,
    method_a <MethodTypeA> = u8,
    method_a <MethodTypeB> = u8
);
