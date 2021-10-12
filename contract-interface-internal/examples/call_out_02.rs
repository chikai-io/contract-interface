//! Example of calling an external contract.

use contract_interface_internal as interface;
use near_sdk::ext_contract;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OneArg;

/// (Original Trait documentation)
// #[ext_contract]
pub trait Trait<X> {
    type MyType;

    /// (Original set_status1 documentation)
    fn method_a<Y, Z>(&mut self, message: String, my_x: X, my_y: Y, my_type: Self::MyType) -> Z;
}

pub fn example() {
    use near_sdk::Gas;

    const SINGLE_CALL_GAS: u64 = 200000000000000;

    // trait_a::method_a(
    //     String::from("my_value"),
    //     "my.contract".parse().unwrap(),
    //     0,
    //     Gas::from(SINGLE_CALL_GAS),
    // );

    // generic builder
    interface::CallOut::contract("my.contract".parse().unwrap())
        .method(String::from("set_status1"))
        .args(interface::json::named::NamedJson1::new(
            String::from("message"),
            String::from("my_value"),
        ))
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call_out();

    let my_x = true;
    let my_y = 1u8;
    let my_type = 5u16;
    // specialized builder
    _trait::method_a::CallOut::contract("my.contract".parse().unwrap())
        .args(String::from("my_value"), my_x, my_y, my_type)
        .send_amount(0)
        .prepaid_gas(Gas::from(SINGLE_CALL_GAS))
        .call_out();
}

// example on how a macro-generated code could look like
// in order to facilitate the interface usage
///
///
/// (Original ExtStatusMessage documentation)
pub mod _trait {

    ///
    ///
    /// (Original method_a documentation)
    pub mod method_a {
        use contract_interface_internal as interface;
        use interface::call_out;
        use near_sdk::serde::{Deserialize, Serialize};
        use near_sdk::{AccountId, Balance, Gas};

        ///
        ///
        /// (Original method_a documentation)
        #[derive(Serialize, Deserialize)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Args<X, MyType, Y> {
            message: String,
            my_x: X,
            my_y: Y,
            my_type: MyType,
        }

        pub fn method_name() -> &'static str {
            "method_a"
        }

        ///
        ///
        /// (Original method_a documentation)
        pub struct CallOut;
        impl CallOut {
            /// Builder for calling the `method_a` method on a contract.
            ///
            /// (Original method_a documentation)
            pub fn contract(contract_being_called: AccountId) -> MethodCallOut {
                MethodCallOut {
                    contract_being_called,
                    method_name: method_name().to_string(),
                }
            }
        }

        ///
        ///
        /// (Original method_a documentation)
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

            /// Informs the arguments (except for `self`) that `method_a` should receive.
            ///
            /// (Original method_a documentation)
            pub fn args<X, MyType, Y>(
                self,
                message: String,
                my_x: X,
                my_y: Y,
                my_type: MyType,
            ) -> ArgsCallOut<X, MyType, Y> {
                let args = Args::<X, MyType, Y> {
                    message,
                    my_x,
                    my_y,
                    my_type,
                };
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
        /// (Original method_a documentation)
        pub struct ArgsCallOut<X, MyType, Y> {
            method_name: String,
            contract_being_called: AccountId,
            args: Args<X, MyType, Y>,
        }

        impl<X, MyType, Y> ArgsCallOut<X, MyType, Y> {
            pub fn new(
                method_name: String,
                contract_being_called: AccountId,
                args: Args<X, MyType, Y>,
            ) -> Self {
                Self {
                    method_name,
                    contract_being_called,
                    args,
                }
            }

            ///
            ///
            /// (Original method_a documentation)
            pub fn send_amount(self, send_amount: Balance) -> AmountCallOut<X, MyType, Y> {
                AmountCallOut {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount,
                }
            }

            ///
            ///
            /// (Original method_a documentation)
            pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCallOut<X, MyType, Y> {
                GasCallOut {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount: 0,
                    prepaid_gas: maximum_allowed_consumption,
                }
            }

            pub fn into_generic(
                self,
            ) -> call_out::ArgsCallOut<Args<X, MyType, Y>, interface::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl<X, MyType, Y> Into<call_out::ArgsCallOut<Args<X, MyType, Y>, interface::Json>>
            for ArgsCallOut<X, MyType, Y>
        {
            fn into(self) -> call_out::ArgsCallOut<Args<X, MyType, Y>, interface::Json> {
                call_out::CallOut::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
            }
        }

        ///
        ///
        /// (Original method_a documentation)
        pub struct AmountCallOut<X, MyType, Y> {
            method_name: String,
            contract_being_called: AccountId,
            args: Args<X, MyType, Y>,
            send_amount: Balance,
        }

        impl<X, MyType, Y> AmountCallOut<X, MyType, Y> {
            ///
            ///
            /// (Original method_a documentation)
            pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCallOut<X, MyType, Y> {
                GasCallOut {
                    method_name: self.method_name,
                    contract_being_called: self.contract_being_called,
                    args: self.args,
                    send_amount: self.send_amount,
                    prepaid_gas: maximum_allowed_consumption,
                }
            }

            pub fn into_generic(
                self,
            ) -> call_out::AmountCallOut<Args<X, MyType, Y>, interface::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl<X, MyType, Y> Into<call_out::AmountCallOut<Args<X, MyType, Y>, interface::Json>>
            for AmountCallOut<X, MyType, Y>
        {
            fn into(self) -> call_out::AmountCallOut<Args<X, MyType, Y>, interface::Json> {
                call_out::CallOut::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
                    .send_amount(self.send_amount)
            }
        }

        ///
        ///
        /// (Original method_a documentation)
        pub struct GasCallOut<X, MyType, Y> {
            method_name: String,
            contract_being_called: AccountId,
            args: Args<X, MyType, Y>,
            send_amount: Balance,
            prepaid_gas: Gas,
        }

        impl<X, MyType, Y> GasCallOut<X, MyType, Y>
        where
            Args<X, MyType, Y>: interface::ToBytes<interface::Json>,
        {
            ///
            ///
            /// (Original method_a documentation)
            pub fn call_out(self) {
                use interface::ToBytes;
                near_sdk::Promise::new(self.contract_being_called).function_call(
                    self.method_name.to_string(),
                    ToBytes::to_bytes(&self.args)
                        .expect("Failed to serialize the cross contract args."),
                    self.send_amount,
                    self.prepaid_gas,
                );
            }

            pub fn into_generic(self) -> call_out::GasCallOut<Args<X, MyType, Y>, interface::Json> {
                self.into()
            }
        }

        #[allow(clippy::from_over_into)]
        impl<X, MyType, Y> Into<call_out::GasCallOut<Args<X, MyType, Y>, interface::Json>>
            for GasCallOut<X, MyType, Y>
        {
            fn into(self) -> call_out::GasCallOut<Args<X, MyType, Y>, interface::Json> {
                call_out::CallOut::contract(self.contract_being_called)
                    .method(self.method_name)
                    .args(self.args)
                    .send_amount(self.send_amount)
                    .prepaid_gas(self.prepaid_gas)
            }
        }
    }
}
