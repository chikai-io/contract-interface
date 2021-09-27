use crate::Call;
use near_sdk::borsh::{self};
use near_sdk::ext_contract;

// TODO: test borsh usage
#[ext_contract]
pub trait Message {
    #[result_serializer(borsh)]
    fn method_a(&mut self, #[serializer(borsh)] message: String);
}

#[ext_contract]
pub trait ExtStatusMessage {
    /// (Original set_status1 documentation)
    fn set_status1(&mut self, message: String);

    /// (Original set_status2 documentation)
    fn set_status2(&mut self, message1: String, message2: String);

    /// (Original get_status documentation)
    fn get_status(&self, account_id: AccountId) -> Option<String>;
}

// example on how a macro-generated code could look like
// in order to facilitate the interface usage
pub mod ext_status_message_example {
    use crate::args::{Json1, JsonArgs};
    use crate::interface::call_builder::ArgsCall;
    use near_sdk::AccountId;

    #[allow(non_camel_case_types)]
    pub struct set_status1 {
        contract_being_called: AccountId,
        method_name: String,
    }

    impl set_status1 {
        /// Builder for calling the `set_status1` method on a contract.
        pub fn contract(contract_being_called: AccountId) -> Self {
            Self {
                contract_being_called,
                method_name: String::from("set_status1"),
            }
        }

        /// Informs the arguments (except for `self`) that `set_status1` should receive.
        /// 0. `message`: `String`
        ///
        /// (Original set_status1 documentation)
        pub fn args<Args>(self, args: Args) -> ArgsCall<JsonArgs<Json1<String>>>
        where
            Args: Into<JsonArgs<Json1<String>>>,
        {
            let args = args.into();
            ArgsCall::new(self.method_name, self.contract_being_called, args)
        }
    }
}

pub fn ex() {
    const SINGLE_CALL_GAS: u64 = 200000000000000;
    use crate::args::Json1;
    use near_sdk::Gas;

    // current/standard call syntax
    ext_status_message::set_status1(
        String::from("my_value"),
        "my.contract".parse().unwrap(),
        0,
        Gas::from(SINGLE_CALL_GAS),
    );

    // filling everything manually
    Call::contract("my.contract".parse().unwrap())
        .method(String::from("set_status1"))
        .args(Json1(String::from("my_value")))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call();

    // using with the method name and args types "auto-generated"
    ext_status_message_example::set_status1::contract("my.contract".parse().unwrap())
        .args(String::from("my_value"))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call();

    // ext_status_message::set_status1(message, account_id.clone(), 0, Gas::from(SINGLE_CALL_GAS))
    //     .then(ext_status_message::get_status(
    //         env::signer_account_id(),
    //         account_id,
    //         0,
    //         Gas::from(SINGLE_CALL_GAS),
    //     ));
}
