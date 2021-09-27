use crate::args::ArgsType;
use near_sdk::serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait SerDe: Serialize + DeserializeOwned {}
impl<T> SerDe for T where T: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct JsonArgs<T>(T);

impl<Args> ArgsType for JsonArgs<Args>
where
    Args: SerDe,
{
    type Error = near_sdk::serde_json::Error;
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        near_sdk::serde_json::to_vec(self)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        near_sdk::serde_json::from_slice(bytes)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Json0();

impl ArgsType for Json0 {
    type Error = near_sdk::serde_json::Error;
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        near_sdk::serde_json::to_vec(self)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        near_sdk::serde_json::from_slice(bytes)
    }
}
impl From<Json0> for JsonArgs<Json0> {
    fn from(val: Json0) -> Self {
        JsonArgs(val)
    }
}
impl From<()> for Json0 {
    fn from((): ()) -> Self {
        Json0()
    }
}
impl From<()> for JsonArgs<Json0> {
    fn from((): ()) -> Self {
        Json0::from(()).into()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Json1<T0: SerDe>(#[serde(deserialize_with = "T0::deserialize")] pub T0);

impl<T0> ArgsType for Json1<T0>
where
    T0: SerDe,
{
    type Error = near_sdk::serde_json::Error;
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        near_sdk::serde_json::to_vec(self)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        near_sdk::serde_json::from_slice(bytes)
    }
}
impl<T0> From<Json1<T0>> for JsonArgs<Json1<T0>>
where
    T0: SerDe,
{
    fn from(val: Json1<T0>) -> Self {
        JsonArgs(val)
    }
}
impl<T0> From<(T0,)> for Json1<T0>
where
    T0: SerDe,
{
    fn from(t: (T0,)) -> Self {
        Json1(t.0)
    }
}
impl<T0> From<(T0,)> for JsonArgs<Json1<T0>>
where
    T0: SerDe,
{
    fn from(t: (T0,)) -> Self {
        Json1::from(t).into()
    }
}
// extra
impl<T0> From<T0> for Json1<T0>
where
    T0: SerDe,
{
    fn from(t0: T0) -> Self {
        Json1(t0)
    }
}
impl<T0> From<T0> for JsonArgs<Json1<T0>>
where
    T0: SerDe,
{
    fn from(t0: T0) -> Self {
        Json1::from(t0).into()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Json2<T0: SerDe, T1: SerDe>(
    #[serde(deserialize_with = "T0::deserialize")] pub T0,
    #[serde(deserialize_with = "T1::deserialize")] pub T1,
);

impl<T0, T1> ArgsType for Json2<T0, T1>
where
    T0: SerDe,
    T1: SerDe,
{
    type Error = near_sdk::serde_json::Error;
    fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        near_sdk::serde_json::to_vec(self)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        near_sdk::serde_json::from_slice(bytes)
    }
}
impl<T0, T1> From<Json2<T0, T1>> for JsonArgs<Json2<T0, T1>>
where
    T0: SerDe,
    T1: SerDe,
{
    fn from(val: Json2<T0, T1>) -> Self {
        JsonArgs(val)
    }
}
impl<T0, T1> From<(T0, T1)> for Json2<T0, T1>
where
    T0: SerDe,
    T1: SerDe,
{
    fn from(t: (T0, T1)) -> Self {
        Json2(t.0, t.1)
    }
}
impl<T0, T1> From<(T0, T1)> for JsonArgs<Json2<T0, T1>>
where
    T0: SerDe,
    T1: SerDe,
{
    fn from(t: (T0, T1)) -> Self {
        Json2::from(t).into()
    }
}
