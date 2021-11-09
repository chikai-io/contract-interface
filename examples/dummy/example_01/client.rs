//! Shows how a client could use the builder to make
//! a request into a method of an implementation of a trait.
//!
//! One uses the "impl" builder, and the other uses the "trait" builder.

pub fn client_example() {
    // using the "impl" request builder, where the state is known (Struct1)
    use super::impl_trait_1::method_ref_mut::Request;
    let _promise = Request::contract("account.id".parse().unwrap())
        .args(true)
        .send_amount(0)
        .prepaid_gas(near_sdk::Gas::from(0))
        .request();

    // using the "trait" request builder, where the state must be informed
    type StructRequest = super::trait_1::method_ref_mut::Request<super::Struct1>;
    StructRequest::contract("account.id".parse().unwrap())
        .args(true)
        .send_amount(0)
        .prepaid_gas(near_sdk::Gas::from(0))
        .request();
}
