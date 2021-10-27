pub use request::Request;

///
/// `Diverged` is used to allow third-party specialization of this trait for arbitrary types.
/// See [RFC 1023](https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md)
/// for more information.
pub trait Serve<ArgsDeserialization, ReturnSerialization, Diverged = ()> {
    type State: near_sdk::borsh::BorshDeserialize + near_sdk::borsh::BorshSerialize + Default;
    type Args: crate::FromBytes<ArgsDeserialization>;
    type Return: crate::ToBytes<ReturnSerialization>;

    fn setup_panic_hook() {
        near_sdk::env::setup_panic_hook();
    }

    fn panic_on_already_existing_state() {
        if near_sdk::env::state_exists() {
            near_sdk::env::panic_str("The contract has already been initialized");
        }
    }

    fn panic_on_deposit() {
        if near_sdk::env::attached_deposit() != 0 {
            near_sdk::env::panic_str("Method doesn\'t accept deposit");
        }
    }

    fn panic_on_non_private() {
        if near_sdk::env::current_account_id() != near_sdk::env::predecessor_account_id() {
            near_sdk::env::panic_str("Method is private");
        }
    }

    fn deserialize_args_from_input() -> Self::Args {
        use crate::FromBytes;
        // TODO: test with method without arguments
        let bytes = near_sdk::env::input().expect("Expected input since method has arguments.");
        Self::Args::from_bytes(bytes.as_ref()).expect("Failed to deserialize the argument values")
    }

    fn state_read_or_default() -> Self::State {
        near_sdk::env::state_read().unwrap_or_default()
    }

    fn state_read_or_panic() -> Self::State {
        match near_sdk::env::state_read() {
            Some(state) => state,
            None => near_sdk::env::panic_str("State must be first initialized"),
        }
    }

    fn may_serialize_return_as_output(result: Option<Self::Return>) {
        if let Some(result) = result {
            Self::serialize_return_as_output(result)
        }
    }

    fn serialize_return_as_output(result: Self::Return) {
        use crate::ToBytes;
        let result = <Self::Return as ToBytes<ReturnSerialization>>::to_bytes(&result)
            .expect("Failed to serialize the return value.");
        near_sdk::env::value_return(&result);
    }

    fn state_write(contract: &Self::State) {
        near_sdk::env::state_write(contract);
    }
}

pub trait ServeRefMut<ArgsDeserialization, ReturnSerialization, Diverged = ()>:
    Serve<ArgsDeserialization, ReturnSerialization, Diverged>
{
    type Method: FnOnce(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn serve(method: Self::Method);
    fn extern_serve();
}

pub trait ServeRef<ArgsDeserialization, ReturnSerialization, Diverged = ()>:
    Serve<ArgsDeserialization, ReturnSerialization, Diverged>
{
    type Method: FnOnce(&Self::State, Self::Args) -> Option<Self::Return>;

    fn serve(method: Self::Method);
    fn extern_serve();
}

pub trait ServeOwned<ArgsDeserialization, Diverged = ()>:
    Serve<ArgsDeserialization, crate::Borsh, Diverged>
{
    type Method: FnOnce(Self::State, Self::Args) -> Self::State;

    fn serve(method: Self::Method);
    fn extern_serve();
}

pub trait ServeStateless<ArgsDeserialization, ReturnSerialization, Diverged = ()>:
    Serve<ArgsDeserialization, ReturnSerialization, Diverged>
{
    type Method: FnOnce(Self::Args) -> Option<Self::Return>;

    fn serve(method: Self::Method);
    fn extern_serve();
}

pub trait ServeStatelessInit<ArgsDeserialization, Diverged = ()>:
    Serve<ArgsDeserialization, crate::Borsh, Diverged>
{
    type Method: FnOnce(Self::Args) -> Self::State;

    fn serve(method: Self::Method);
    fn extern_serve();
}

pub mod request {
    use near_sdk::{AccountId, Balance, Gas};
    use std::marker::PhantomData;

    pub struct Request {
        contract_being_called: AccountId,
    }

    impl Request {
        pub fn contract(contract_being_called: AccountId) -> Self {
            Self {
                contract_being_called,
            }
        }
        pub fn method(self, method_name: String) -> MethodRequest {
            MethodRequest {
                method_name,
                contract_being_called: self.contract_being_called,
            }
        }
    }

    pub struct MethodRequest {
        method_name: String,
        contract_being_called: AccountId,
    }

    impl MethodRequest {
        pub fn new(contract_call: Request, method_name: String) -> Self {
            contract_call.method(method_name)
        }

        pub fn args<Args, ArgsSerialization>(
            self,
            args: Args,
        ) -> ArgsRequest<Args, ArgsSerialization> {
            ArgsRequest {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args,
                args_serialization: PhantomData,
            }
        }
    }

    pub struct ArgsRequest<Args, ArgsSerialization> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
        args_serialization: PhantomData<ArgsSerialization>,
    }

    impl<Args, ArgsSerialization> ArgsRequest<Args, ArgsSerialization> {
        pub fn new(method_call: MethodRequest, args: Args) -> Self {
            method_call.args(args)
        }

        pub fn send_amount(self, send_amount: Balance) -> AmountRequest<Args, ArgsSerialization> {
            AmountRequest {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                args_serialization: self.args_serialization,
                send_amount,
            }
        }

        pub fn prepaid_gas(
            self,
            maximum_allowed_consumption: Gas,
        ) -> GasRequest<Args, ArgsSerialization> {
            GasRequest {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                args_serialization: self.args_serialization,
                send_amount: 0,
                prepaid_gas: maximum_allowed_consumption,
            }
        }
    }

    pub struct AmountRequest<Args, ArgsSerialization> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
        args_serialization: PhantomData<ArgsSerialization>,
        send_amount: Balance,
    }

    impl<Args, ArgsSerialization> AmountRequest<Args, ArgsSerialization> {
        pub fn new(args_call: ArgsRequest<Args, ArgsSerialization>, send_amount: Balance) -> Self {
            args_call.send_amount(send_amount)
        }

        pub fn prepaid_gas(
            self,
            maximum_allowed_consumption: Gas,
        ) -> GasRequest<Args, ArgsSerialization> {
            GasRequest {
                method_name: self.method_name,
                contract_being_called: self.contract_being_called,
                args: self.args,
                args_serialization: self.args_serialization,
                send_amount: self.send_amount,
                prepaid_gas: maximum_allowed_consumption,
            }
        }
    }

    pub struct GasRequest<Args, ArgsSerialization> {
        method_name: String,
        contract_being_called: AccountId,
        args: Args,
        args_serialization: PhantomData<ArgsSerialization>,
        send_amount: Balance,
        prepaid_gas: Gas,
    }

    impl<Args, ArgsSerialization> GasRequest<Args, ArgsSerialization>
    where
        Args: crate::ToBytes<ArgsSerialization>,
    {
        pub fn new(
            amount_call: AmountRequest<Args, ArgsSerialization>,
            maximum_allowed_consumption: Gas,
        ) -> Self {
            amount_call.prepaid_gas(maximum_allowed_consumption)
        }

        pub fn custom_method_name(mut self, custom_method_name: String) -> Self {
            let name = &mut self.method_name;
            *name = custom_method_name;
            self
        }

        pub fn request(self) {
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
