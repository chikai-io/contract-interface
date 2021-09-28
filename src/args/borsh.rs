use crate::args::{FromBytes, ToBytes};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

pub struct Borsh;

impl<T> ToBytes<Borsh> for T
where
    T: BorshSerialize,
{
    type Error = std::io::Error;

    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        BorshSerialize::try_to_vec(self)
    }
}

impl<T> FromBytes<Borsh> for T
where
    T: BorshDeserialize,
{
    type Error = std::io::Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        BorshDeserialize::try_from_slice(bytes)
    }
}
