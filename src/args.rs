pub mod borsh;
pub mod json;

pub use borsh::BorshArgs;
pub use json::{Json0, Json1, Json2, JsonArgs};

pub trait ArgsType {
    fn to_byte_vec(&self) -> Vec<u8>;
}
