//! Contains `extern "C"` functions, created by macros.  
//! The created functions can be seen in the documentation.
//!
//! See also [`super::api_manual`] for an example of how to
//! manually create those functions.

crate::macros::extern_struct_!(impl_mod = super::struct_);
