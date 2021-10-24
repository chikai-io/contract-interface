use super::struct_;

pub fn client_example() {
    use struct_::method_ref_mut::Request;

    let _promise = Request::contract("account.id".parse().unwrap())
        .args(true)
        .send_amount(0)
        .prepaid_gas(near_sdk::Gas::from(0))
        .request();
}
