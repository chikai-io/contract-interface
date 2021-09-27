pub mod borsh;
pub mod json;

pub use borsh::BorshArgs;
pub use json::{Json0, Json1, Json2, JsonArgs};

pub trait ArgsType: Sized {
    type Error: std::fmt::Debug;
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
}
