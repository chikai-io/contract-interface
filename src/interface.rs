pub use call_builder::Call;

// use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

// pub trait IContract {
//     fn methods(&self) -> Vec<Box<dyn IDynMethod>>;
// }

// pub trait IMethod {
//     type Input: ArgsType;
//     fn fn_name(&self) -> &'static str;
// }

// pub trait IDynMethod {
//     fn fn_input(&self) -> Box<dyn ArgsType>;
//     fn fn_name(&self) -> &'static str;
// }

// pub struct ExtStatusMessage_set_status1 {}
// impl IMethod for ExtStatusMessage_set_status1 {
//     type Input = Json1<String>;
//     fn fn_name(&self) -> &'static str {
//         "set_status1"
//     }
// }

// ordered_contract_call(
//     JsonArgs(Json1(message.clone(),)),
//     "set_status",
//     account_id.clone(),
//     0,
//     Gas::from(SINGLE_CALL_GAS)
// );

pub mod call_builder {
    use crate::args::ArgsType;
    use near_sdk::{AccountId, Balance, Gas};

    pub struct Call {
        contract_being_called: AccountId,
    }

    impl Call {
        pub fn contract(contract_being_called: AccountId) -> Self {
            Self {
                contract_being_called,
            }
        }
        pub fn method(self, method_name: String) -> MethodCall {
            MethodCall {
                method_name,
                contract_being_called: self.contract_being_called,
            }
        }
    }

    pub struct MethodCall {
        method_name: String,
        contract_being_called: AccountId,
    }

    impl MethodCall {
        pub fn new(contract_call: Call, method_name: String) -> Self {
            contract_call.method(method_name)
        }

        pub fn args<Args>(self, args: Args) -> ArgsCall<Args> {
            ArgsCall {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args,
            }
        }
    }

    pub struct ArgsCall<Args> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
    }

    impl<Args> ArgsCall<Args> {
        pub fn new(method_call: MethodCall, args: Args) -> Self {
            method_call.args(args)
        }

        pub fn send_amount(self, send_amount: Balance) -> AmountCall<Args> {
            AmountCall {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount,
            }
        }

        pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCall<Args> {
            GasCall {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount: 0,
                prepaid_gas: maximum_allowed_consumption,
            }
        }
    }

    pub struct AmountCall<Args> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
        send_amount: Balance,
    }

    impl<Args> AmountCall<Args> {
        pub fn new(args_call: ArgsCall<Args>, send_amount: Balance) -> Self {
            args_call.send_amount(send_amount)
        }

        pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCall<Args> {
            GasCall {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount: self.send_amount,
                prepaid_gas: maximum_allowed_consumption,
            }
        }
    }

    pub struct GasCall<Args> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
        send_amount: Balance,
        prepaid_gas: Gas,
    }

    impl<Args> GasCall<Args>
    where
        Args: ArgsType,
    {
        pub fn new(amount_call: AmountCall<Args>, maximum_allowed_consumption: Gas) -> Self {
            amount_call.prepaid_gas(maximum_allowed_consumption)
        }

        pub fn call(self) {
            near_sdk::Promise::new(self.contract_being_called).function_call(
                self.method_name.to_string(),
                self.args.to_byte_vec(),
                self.send_amount,
                self.prepaid_gas,
            );
        }
    }
}
