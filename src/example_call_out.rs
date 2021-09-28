//! Example of calling an external contract.

use near_sdk::{ext_contract, AccountId as AccId};

/// (Original ExtStatusMessage documentation)
#[ext_contract]
pub trait ExtStatusMessage {
    /// (Original set_status1 documentation)
    fn set_status1(&mut self, message: String);

    /// (Original set_status2 documentation)
    fn set_status2(&mut self, message1: String, message2: String);

    /// (Original get_status documentation)
    fn get_status(&self, account_id: AccId) -> Option<String>;

    // TODO: make it work with generics
    // TODO: test re-utilization from call_in expansions
    // (remember that the State is not necessary)
    // fn method_b<Y, Z>(&mut self, my_string: String, my_y: Y) -> Z;
}

pub fn example() {
    use near_sdk::Gas;

    const SINGLE_CALL_GAS: u64 = 200000000000000;

    // current/standard call syntax
    ext_status_message::set_status1(
        String::from("my_value"),
        "my.contract".parse().unwrap(),
        0,
        Gas::from(SINGLE_CALL_GAS),
    );

    // generic builder
    crate::CallOut::contract("my.contract".parse().unwrap())
        .method(String::from("set_status1"))
        .args(crate::args::json::named::NamedJson1::new(
            String::from("message"),
            String::from("my_value"),
        ))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call_out();

    // specialized builder
    ext_status_message_example::set_status1::CallOut::contract("my.contract".parse().unwrap())
        .args(String::from("my_value"))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call_out();

    //
    // later: using with promisses
    //
    // ext_status_message::set_status1(message, account_id.clone(), 0, Gas::from(SINGLE_CALL_GAS))
    //     .then(ext_status_message::get_status(
    //         env::signer_account_id(),
    //         account_id,
    //         0,
    //         Gas::from(SINGLE_CALL_GAS),
    //     ));
}

// example on how a macro-generated code could look like
// in order to facilitate the interface usage
///
///
/// (Original ExtStatusMessage documentation)
pub mod ext_status_message_example {

    ///
    ///
    /// (Original set_status1 documentation)
    pub mod set_status1 {
        use crate::interface::call_out;
        use near_sdk::serde::{Deserialize, Serialize};
        use near_sdk::{AccountId, Balance, Gas};

        ///
        ///
        /// (Original set_status1 documentation)
        #[derive(Serialize, Deserialize)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Args {
            message: String,
        }

        pub fn method_name() -> &'static str {
            "set_status1"
        }

        ///
        ///
        /// (Original set_status1 documentation)
        pub struct CallOut;
        impl CallOut {
            /// Builder for calling the `set_status1` method on a contract.
            ///
            /// (Original set_status1 documentation)
            pub fn contract(contract_being_called: AccountId) -> MethodCallOut {
                MethodCallOut {
                    contract_being_called,
                    method_name: method_name().to_string(),
                }
            }
        }

        ///
        ///
        /// (Original set_status1 documentation)
        pub struct MethodCallOut {
            contract_being_called: AccountId,
            method_name: String,
        }

        impl MethodCallOut {
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
            ///
            /// (Original set_status1 documentation)
            pub fn args(self, message: String) -> ArgsCallOut {
                let args = Args { message };
                ArgsCallOut::new(self.method_name, self.contract_being_called, args)
            }

            pub fn into_generic(self) -> call_out::MethodCallOut {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<call_out::MethodCallOut> for MethodCallOut {
            fn into(self) -> call_out::MethodCallOut {
                call_out::CallOut::contract(self.contract_being_called).method(self.method_name)
            }
        }
        ///
        ///
        /// (Original set_status1 documentation)
        pub struct ArgsCallOut {
            method_name: String,
            contract_being_called: AccountId,
            args: Args,
        }

        impl ArgsCallOut {
            pub fn new(method_name: String, contract_being_called: AccountId, args: Args) -> Self {
                Self {
                    method_name,
                    contract_being_called,
                    args,
                }
            }

            ///
            ///
            /// (Original set_status1 documentation)   
            pub fn send_amount(self, send_amount: Balance) -> AmountCallOut {
                AmountCallOut {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount,
                }
            }

            ///
            ///
            /// (Original set_status1 documentation)
            pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCallOut {
                GasCallOut {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount: 0,
                    prepaid_gas: maximum_allowed_consumption,
                }
            }

            pub fn into_generic(self) -> call_out::ArgsCallOut<Args, crate::args::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<call_out::ArgsCallOut<Args, crate::args::Json>> for ArgsCallOut {
            fn into(self) -> call_out::ArgsCallOut<Args, crate::args::Json> {
                call_out::CallOut::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
            }
        }

        ///
        ///
        /// (Original set_status1 documentation)
        pub struct AmountCallOut {
            method_name: String,
            contract_being_called: AccountId,
            args: Args,
            send_amount: Balance,
        }

        impl AmountCallOut {
            ///
            ///
            /// (Original set_status1 documentation)
            pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCallOut {
                GasCallOut {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount: self.send_amount,
                    prepaid_gas: maximum_allowed_consumption,
                }
            }

            pub fn into_generic(self) -> call_out::AmountCallOut<Args, crate::args::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<call_out::AmountCallOut<Args, crate::args::Json>> for AmountCallOut {
            fn into(self) -> call_out::AmountCallOut<Args, crate::args::Json> {
                call_out::CallOut::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
                    .send_amount(self.send_amount)
            }
        }

        ///
        ///
        /// (Original set_status1 documentation)
        pub struct GasCallOut {
            method_name: String,
            contract_being_called: AccountId,
            args: Args,
            send_amount: Balance,
            prepaid_gas: Gas,
        }

        impl GasCallOut {
            ///
            ///
            /// (Original set_status1 documentation)
            pub fn call_out(self) {
                use crate::args::ToBytes;
                near_sdk::Promise::new(self.contract_being_called).function_call(
                    self.method_name.to_string(),
                    self.args
                        .to_bytes()
                        .expect("Failed to serialize the cross contract args."),
                    self.send_amount,
                    self.prepaid_gas,
                );
            }

            pub fn into_generic(self) -> call_out::GasCallOut<Args, crate::args::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<call_out::GasCallOut<Args, crate::args::Json>> for GasCallOut {
            fn into(self) -> call_out::GasCallOut<Args, crate::args::Json> {
                call_out::CallOut::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
                    .send_amount(self.send_amount)
                    .prepaid_gas(self.prepaid_gas)
            }
        }
    }
}
