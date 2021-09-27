use crate::args::ArgsType;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub trait BorshSerDe: BorshSerialize + BorshDeserialize {}
impl<T> BorshSerDe for T where T: BorshSerialize + BorshDeserialize {}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BorshArgs<T: BorshSerDe>(T);

impl<Args> ArgsType for BorshArgs<Args>
where
    Args: BorshSerDe,
{
    fn to_byte_vec(&self) -> Vec<u8> {
        BorshSerialize::try_to_vec(self)
            .expect("Failed to serialize the cross contract args using Borsh.")
    }
}

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
