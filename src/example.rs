use crate::Call;
use near_sdk::borsh::{self};
use near_sdk::ext_contract;

// TODO: test borsh usage
#[ext_contract]
pub trait Message {
    #[result_serializer(borsh)]
    fn method_a(&mut self, #[serializer(borsh)] message: String);
}

/// (Original ExtStatusMessage documentation)
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
///
///
/// (Original ExtStatusMessage documentation)
pub mod ext_status_message_example {
    use crate::args::Json1;
    use crate::interface::call_builder::{
        AmountCall as GenericAmountCall, ArgsCall as GenericArgsCall, Call as GenericCall,
        GasCall as GenericGasCall, MethodCall as GenericMethodCall,
    };
    use near_sdk::{AccountId, Balance, Gas};

    ///
    ///
    /// (Original set_status1 documentation)
    #[allow(non_camel_case_types)]
    pub struct set_status1 {
        contract_being_called: AccountId,
        method_name: String,
    }

    impl set_status1 {
        /// Builder for calling the `set_status1` method on a contract.
        ///
        /// (Original set_status1 documentation)
        pub fn contract(contract_being_called: AccountId) -> Self {
            Self {
                contract_being_called,
                method_name: String::from("set_status1"),
            }
        }

        // For treating the arguments, there are two possibilities:
        // to utilize a generic interface::ArgsCall, or to re-define a
        // specialized ArgsCall (together with AmountCall and GasCall).
        //
        // The benefit of the former is to be able to build generic interfaces
        // on those calls.
        // The benefit of the later is to have the method documentation
        // also replicated on every method and definition on
        // Args/Amount/GasCall, besides it being more specialized.

        /// Informs the arguments (except for `self`) that `set_status1` should receive.
        /// 0. `message`: `String`
        ///
        /// (Original set_status1 documentation)
        pub fn args<Args>(self, args: Args) -> ArgsCall
        where
            Args: Into<Json1<String>>,
        {
            ArgsCall::new(self.method_name, self.contract_being_called, args)
        }

        pub fn into_generic(self) -> GenericMethodCall {
            self.into()
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<GenericMethodCall> for set_status1 {
        fn into(self) -> GenericMethodCall {
            GenericCall::contract(self.contract_being_called).method(self.method_name)
        }
    }

    ///
    ///
    /// (Original set_status1 documentation)
    pub struct ArgsCall {
        method_name: String,
        contract_being_called: AccountId,
        args: Json1<String>,
    }

    impl ArgsCall {
        pub fn new<Args>(method_name: String, contract_being_called: AccountId, args: Args) -> Self
        where
            Args: Into<Json1<String>>,
        {
            Self {
                method_name,
                contract_being_called,
                args: args.into(),
            }
        }

        ///
        ///
        /// (Original set_status1 documentation)   
        pub fn send_amount(self, send_amount: Balance) -> AmountCall {
            AmountCall {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount,
            }
        }

        ///
        ///
        /// (Original set_status1 documentation)
        pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCall {
            GasCall {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount: 0,
                prepaid_gas: maximum_allowed_consumption,
            }
        }

        pub fn into_generic(self) -> GenericArgsCall<Json1<String>> {
            self.into()
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<GenericArgsCall<Json1<String>>> for ArgsCall {
        fn into(self) -> GenericArgsCall<Json1<String>> {
            GenericCall::contract(self.contract_being_called)
                .method(self.method_name)
                .args(self.args)
        }
    }

    ///
    ///
    /// (Original set_status1 documentation)
    pub struct AmountCall {
        method_name: String,
        contract_being_called: AccountId,
        args: Json1<String>,
        send_amount: Balance,
    }

    impl AmountCall {
        ///
        ///
        /// (Original set_status1 documentation)
        pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCall {
            GasCall {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount: self.send_amount,
                prepaid_gas: maximum_allowed_consumption,
            }
        }

        pub fn into_generic(self) -> GenericAmountCall<Json1<String>> {
            self.into()
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<GenericAmountCall<Json1<String>>> for AmountCall {
        fn into(self) -> GenericAmountCall<Json1<String>> {
            GenericCall::contract(self.contract_being_called)
                .method(self.method_name)
                .args(self.args)
                .send_amount(self.send_amount)
        }
    }

    ///
    ///
    /// (Original set_status1 documentation)
    pub struct GasCall {
        method_name: String,
        contract_being_called: AccountId,
        args: Json1<String>,
        send_amount: Balance,
        prepaid_gas: Gas,
    }

    impl GasCall {
        ///
        ///
        /// (Original set_status1 documentation)
        pub fn call(self) {
            use crate::args::ArgsType;
            near_sdk::Promise::new(self.contract_being_called).function_call(
                self.method_name.to_string(),
                self.args.to_byte_vec(),
                self.send_amount,
                self.prepaid_gas,
            );
        }

        pub fn into_generic(self) -> GenericGasCall<Json1<String>> {
            self.into()
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<GenericGasCall<Json1<String>>> for GasCall {
        fn into(self) -> GenericGasCall<Json1<String>> {
            GenericCall::contract(self.contract_being_called)
                .method(self.method_name)
                .args(self.args)
                .send_amount(self.send_amount)
                .prepaid_gas(self.prepaid_gas)
        }
    }
}

pub fn example() {
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
