pub mod borsh;
pub mod json;

pub use borsh::Borsh;
pub use json::Json;

pub trait ToBytes<Type>: Sized {
    type Error: std::fmt::Debug;
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error>;
}

pub trait FromBytes<Type>: Sized {
    type Error: std::fmt::Debug;
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
}
