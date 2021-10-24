use super::{interface, trait4, Struct};

fn client_example() {
    use trait4::method_ref_mut::request::Request;
    // Request::contract("addr".parse().unwrap());

    // _trait_a::method_a::Request::contract("my.contract".parse().unwrap())
    //     .args(String::from("my_value"))
    //     .send_amount(0)
    //     .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
    //     .call_out();
}
