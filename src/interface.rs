pub use call_out_builder::CallOut;

// generic
pub trait CalledIn {
    type State: crate::args::borsh::BorshSerDe + Default;
    type Method: FnOnce(&mut Self::State, Self::Args) -> Option<Self::Return>;
    type Args: crate::args::ArgsType;

    type Return: crate::args::ArgsType;
    // = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;
    // note: associated type defaults are unstable
    // see issue #29661 <https://github.com/rust-lang/rust/issues/29661> for more information

    fn called_in(method: Self::Method) {
        use crate::args::ArgsType;

        near_sdk::env::setup_panic_hook();
        if near_sdk::env::attached_deposit() != 0 {
            near_sdk::env::panic_str("Method method_a doesn\'t accept deposit");
        }

        let bytes = near_sdk::env::input().expect("Expected input since method has arguments.");

        let args = Self::Args::from_bytes(bytes.as_ref())
            .expect("Failed to deserialize the argument values");

        let mut contract: Self::State = near_sdk::env::state_read().unwrap_or_default();
        let result = method(&mut contract, args);
        if let Some(result) = result {
            let result = <Self::Return as ArgsType>::to_bytes(&result)
                .expect("Failed to serialize the return value.");
            near_sdk::env::value_return(&result);
        }
        near_sdk::env::state_write(&contract);
    }

    fn exposed_called_in();
}

pub mod call_out_builder {
    use crate::args::ArgsType;
    use near_sdk::{AccountId, Balance, Gas};

    pub struct CallOut {
        contract_being_called: AccountId,
    }

    impl CallOut {
        pub fn contract(contract_being_called: AccountId) -> Self {
            Self {
                contract_being_called,
            }
        }
        pub fn method(self, method_name: String) -> MethodCallOut {
            MethodCallOut {
                method_name,
                contract_being_called: self.contract_being_called,
            }
        }
    }

    pub struct MethodCallOut {
        method_name: String,
        contract_being_called: AccountId,
    }

    impl MethodCallOut {
        pub fn new(contract_call: CallOut, method_name: String) -> Self {
            contract_call.method(method_name)
        }

        pub fn args<Args>(self, args: Args) -> ArgsCallOut<Args> {
            ArgsCallOut {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args,
            }
        }
    }

    pub struct ArgsCallOut<Args> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
    }

    impl<Args> ArgsCallOut<Args> {
        pub fn new(method_call: MethodCallOut, args: Args) -> Self {
            method_call.args(args)
        }

        pub fn send_amount(self, send_amount: Balance) -> AmountCallOut<Args> {
            AmountCallOut {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount,
            }
        }

        pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCallOut<Args> {
            GasCallOut {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount: 0,
                prepaid_gas: maximum_allowed_consumption,
            }
        }
    }

    pub struct AmountCallOut<Args> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
        send_amount: Balance,
    }

    impl<Args> AmountCallOut<Args> {
        pub fn new(args_call: ArgsCallOut<Args>, send_amount: Balance) -> Self {
            args_call.send_amount(send_amount)
        }

        pub fn prepaid_gas(self, maximum_allowed_consumption: Gas) -> GasCallOut<Args> {
            GasCallOut {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                send_amount: self.send_amount,
                prepaid_gas: maximum_allowed_consumption,
            }
        }
    }

    pub struct GasCallOut<Args> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
        send_amount: Balance,
        prepaid_gas: Gas,
    }

    impl<Args> GasCallOut<Args>
    where
        Args: ArgsType,
    {
        pub fn new(amount_call: AmountCallOut<Args>, maximum_allowed_consumption: Gas) -> Self {
            amount_call.prepaid_gas(maximum_allowed_consumption)
        }

        pub fn call_out(self) {
            near_sdk::Promise::new(self.contract_being_called).function_call(
                self.method_name.to_string(),
                self.args
                    .to_bytes()
                    .expect("Failed to serialize the cross contract args."),
                self.send_amount,
                self.prepaid_gas,
            );
        }
    }
}
