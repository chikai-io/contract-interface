//! Example of calling an external contract.

use contract_interface as interface;

use near_sdk::{ext_contract, AccountId as AccId};

/// (Original TraitA documentation)
#[ext_contract]
pub trait TraitA {
    /// (Original method_a documentation)
    fn method_a(&mut self, my_string: String);

    /// (Original set_status2 documentation)
    fn method_b(&mut self, my_string_1: String, my_string_2: String);

    /// (Original get_status documentation)
    fn get_status(&self, account_id: AccId) -> Option<String>;
}

pub fn example() {
    use near_sdk::Gas;

    const SINGLE_CALL_GAS: u64 = 200000000000000;

    // current/standard call syntax
    trait_a::method_a(
        String::from("my_value"),
        "my.contract".parse().unwrap(),
        0,
        Gas::from(SINGLE_CALL_GAS),
    );

    // generic builder
    interface::Request::contract("my.contract".parse().unwrap())
        .method(String::from("method_a"))
        .args(interface::json::named::NamedJson1::new(
            String::from("my_string"),
            String::from("my_value"),
        ))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .request();

    // specialized builder
    _trait_a::method_a::Request::contract("my.contract".parse().unwrap())
        .args(String::from("my_value"))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call_out();

    //
    // later: using with promisses
    //

    trait_a::method_a(
        String::from("my_value"),
        "my.contract".parse().unwrap(),
        0,
        Gas::from(SINGLE_CALL_GAS),
    )
    .then(trait_a::method_a(
        String::from("my_value"),
        "my.contract".parse().unwrap(),
        0,
        Gas::from(SINGLE_CALL_GAS),
    ));
}

// example on how a macro-generated code could look like
// in order to facilitate the interface usage
///
///
/// (Original TraitA documentation)
pub mod _trait_a {

    ///
    ///
    /// (Original method_a documentation)
    pub mod method_a {
        use contract_interface as interface;
        use interface::request;
        use near_sdk::serde::{Deserialize, Serialize};
        use near_sdk::{AccountId, Balance, Gas};

        ///
        ///
        /// (Original method_a documentation)
        #[derive(Serialize, Deserialize)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Args {
            my_string: String,
        }

        pub fn method_name() -> &'static str {
            "set_status1"
        }

        ///
        ///
        /// (Original method_a documentation)
        pub struct Request;
        impl Request {
            /// Builder for calling the `set_status1` method on a contract.
            ///
            /// (Original method_a documentation)
            pub fn contract(contract_being_called: AccountId) -> MethodRequest {
                MethodRequest {
                    contract_being_called,
                    method_name: method_name().to_string(),
                }
            }
        }

        ///
        ///
        /// (Original method_a documentation)
        pub struct MethodRequest {
            contract_being_called: AccountId,
            method_name: String,
        }

        impl MethodRequest {
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
            /// (Original method_a documentation)
            pub fn args(self, message: String) -> ArgsRequest {
                let args = Args { my_string: message };
                ArgsRequest::new(self.method_name, self.contract_being_called, args)
            }

            pub fn into_generic(self) -> request::MethodRequest {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<request::MethodRequest> for MethodRequest {
            fn into(self) -> request::MethodRequest {
                request::Request::contract(self.contract_being_called).method(self.method_name)
            }
        }
        ///
        ///
        /// (Original method_a documentation)
        pub struct ArgsRequest {
            method_name: String,
            contract_being_called: AccountId,
            args: Args,
        }

        impl ArgsRequest {
            pub fn new(method_name: String, contract_being_called: AccountId, args: Args) -> Self {
                Self {
                    method_name,
                    contract_being_called,
                    args,
                }
            }

            ///
            ///
            /// (Original method_a documentation)   
            pub fn send_amount(self, send_amount: Balance) -> AmountRequest {
                AmountRequest {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount,
                }
            }

            ///
            ///
            /// (Original method_a documentation)
            pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasRequest {
                GasRequest {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount: 0,
                    prepaid_gas: maximum_allowed_consumption,
                }
            }

            pub fn into_generic(self) -> request::ArgsRequest<Args, interface::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<request::ArgsRequest<Args, interface::Json>> for ArgsRequest {
            fn into(self) -> request::ArgsRequest<Args, interface::Json> {
                request::Request::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
            }
        }

        ///
        ///
        /// (Original method_a documentation)
        pub struct AmountRequest {
            method_name: String,
            contract_being_called: AccountId,
            args: Args,
            send_amount: Balance,
        }

        impl AmountRequest {
            ///
            ///
            /// (Original method_a documentation)
            pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasRequest {
                GasRequest {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount: self.send_amount,
                    prepaid_gas: maximum_allowed_consumption,
                }
            }

            pub fn into_generic(self) -> request::AmountRequest<Args, interface::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<request::AmountRequest<Args, interface::Json>> for AmountRequest {
            fn into(self) -> request::AmountRequest<Args, interface::Json> {
                request::Request::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
                    .send_amount(self.send_amount)
            }
        }

        ///
        ///
        /// (Original method_a documentation)
        pub struct GasRequest {
            method_name: String,
            contract_being_called: AccountId,
            args: Args,
            send_amount: Balance,
            prepaid_gas: Gas,
        }

        impl GasRequest {
            ///
            ///
            /// (Original method_a documentation)
            pub fn call_out(self) {
                use interface::ToBytes;
                near_sdk::Promise::new(self.contract_being_called).function_call(
                    self.method_name.to_string(),
                    self.args
                        .to_bytes()
                        .expect("Failed to serialize the cross contract args."),
                    self.send_amount,
                    self.prepaid_gas,
                );
            }

            pub fn into_generic(self) -> request::GasRequest<Args, interface::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<request::GasRequest<Args, interface::Json>> for GasRequest {
            fn into(self) -> request::GasRequest<Args, interface::Json> {
                request::Request::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
                    .send_amount(self.send_amount)
                    .prepaid_gas(self.prepaid_gas)
            }
        }
    }
}
