use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait SerDe: Serialize + DeserializeOwned {}
impl<T> SerDe for T where T: Serialize + DeserializeOwned {}

pub trait BorshSerDe: BorshSerialize + BorshDeserialize {}
impl<T> BorshSerDe for T where T: BorshSerialize + BorshDeserialize {}

pub trait OrderedJson: SerDe {}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct JsonArgs<T>(T);

// impl OrderedJson for JsonArgs<Json0> {}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Json0();

impl ArgsType for Json0 {
    fn to_byte_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize the cross contract args using JSON.")
    }
}
impl From<Json0> for JsonArgs<Json0> {
    fn from(val: Json0) -> Self {
        JsonArgs(val)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Json1<T0: SerDe>(#[serde(deserialize_with = "T0::deserialize")] pub T0);

impl<T0> ArgsType for Json1<T0>
where
    T0: SerDe,
{
    fn to_byte_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize the cross contract args using JSON.")
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
    fn to_byte_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize the cross contract args using JSON.")
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

// pub trait OrderedBorsh: BorshSerDe {}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BorshArgs<T: BorshSerDe>(T);

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct  Borsh0();

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct  Borsh1<T0: BorshSerDe>(T0,);

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct  Borsh2<T0: BorshSerDe, T1: BorshSerDe>(T0,T1,);

// impl OrderedBorsh for BorshArgs<Borsh0> {}

// impl<T0> OrderedBorsh for BorshArgs<Borsh1<T0>>
// where
//     T0: BorshSerDe
// {}

// impl<T0, T1> OrderedBorsh for BorshArgs<Borsh2<T0, T1>> where
//     T0: BorshSerDe,
//     T1: BorshSerDe,
// {}

pub trait ArgsType {
    fn to_byte_vec(&self) -> Vec<u8>;
}

impl<Args> ArgsType for JsonArgs<Args>
where
    Args: SerDe,
{
    fn to_byte_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize the cross contract args using JSON.")
    }
}
impl<Args> ArgsType for BorshArgs<Args>
where
    Args: BorshSerDe,
{
    fn to_byte_vec(&self) -> Vec<u8> {
        BorshSerialize::try_to_vec(self)
            .expect("Failed to serialize the cross contract args using Borsh.")
    }
}
