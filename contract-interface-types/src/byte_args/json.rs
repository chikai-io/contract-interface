use crate::byte_args::{FromBytes, ToBytes};
use near_sdk::serde::{de::DeserializeOwned, Serialize};

pub struct Json;

impl<T> ToBytes<Json> for T
where
    T: Serialize,
{
    type Error = near_sdk::serde_json::Error;
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        near_sdk::serde_json::to_vec(self)
    }
}

impl<T> FromBytes<Json> for T
where
    T: DeserializeOwned,
{
    type Error = near_sdk::serde_json::Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        near_sdk::serde_json::from_slice(bytes)
    }
}

// based on https://github.com/serde-rs/serde/issues/766#issuecomment-280353386
pub mod named {
    use super::Serialize;

    pub struct NamedJson1<T0> {
        name0: String,
        value0: T0,
    }
    impl<T0> NamedJson1<T0> {
        pub fn new(name0: String, value0: T0) -> Self {
            Self { name0, value0 }
        }
    }

    impl<T0> Serialize for NamedJson1<T0>
    where
        T0: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: near_sdk::serde::Serializer,
        {
            use near_sdk::serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_entry(&self.name0, &self.value0)?;
            map.end()
        }
    }

    pub struct NamedJson2<T0, T1> {
        name0: String,
        value0: T0,
        name1: String,
        value1: T1,
    }

    impl<T0, T1> Serialize for NamedJson2<T0, T1>
    where
        T0: Serialize,
        T1: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: near_sdk::serde::Serializer,
        {
            use near_sdk::serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry(&self.name0, &self.value0)?;
            map.serialize_entry(&self.name1, &self.value1)?;
            map.end()
        }
    }
}
