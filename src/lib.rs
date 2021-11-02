// used so that tests/examples inside this project don't error.
//
// proc-macro-crate#10
// https://github.com/bkchr/proc-macro-crate/issues/10#issuecomment-826386235
extern crate self as contract_interface;

pub use contract_interface_macros::contract;
pub use contract_interface_types::{
    borsh, json, request, Borsh, FromBytes, Json, Request, Serve, ServeOwned, ServeRef,
    ServeRefMut, ServeStateless, ServeStatelessInit, ToBytes,
};
