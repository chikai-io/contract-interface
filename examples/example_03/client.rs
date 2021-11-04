use super::{Struct3A, Struct3B};
// use contract_interface::Inherited;
use contract_standards::fungible_token::{core::impl_inheritance, core_impl::FungibleToken};

use contract_interface::{IThen, Then};

pub fn client_example() {
    use impl_inheritance::ft_transfer::Request;

    type Ft = FungibleToken;
    type S3A = Struct3A;
    type S3B = Struct3B;

    type L1 = super::L3B3A;
    type L2 = super::L3AFt;

    pub type L12 = Then<L1, L2, Struct3A>;

    Request::<Struct3A, L2, Ft>::contract("a".parse().unwrap());

    Request::<Struct3B, L12, Ft>::contract("a".parse().unwrap());
    Request::<Struct3B, Then<L1, L2, Struct3A>, Ft>::contract("a".parse().unwrap());

    Request::<Struct3B, Then<super::L3B3A, super::L3AFt, Struct3A>, FungibleToken>::contract(
        "a".parse().unwrap(),
    );

    use contract_standards::fungible_token::FungibleTokenCore;

    type Inheritance = Then<L1, L2, Struct3A>;

    let val = Struct3B::default();

    <Struct3B as FungibleTokenCore<Inheritance, Ft>>::ft_total_supply(&val);

    <Struct3B as FungibleTokenCore<Then<L1, L2, _>, _>>::ft_total_supply(&val);

    // val.ft_total_supply();

    let valx = Ft::default();
    valx.ft_total_supply();

    // type I3AFt = Inherited<S3A, Ft>;
    // type I3B3A = Inherited<S3B, S3A>;
    // type I3BI3AFt = Inherited<I3B3A, Ft>;

    // Request::<S3A, Ft>::contract("a".parse().unwrap());

    // Request::<S3B, Ft>::contract("a".parse().unwrap());

    // Request::<I3B3A, FungibleToken>::contract("a".parse().unwrap());

    // Request::<I3BI3AFt, FungibleToken>::contract("a".parse().unwrap());

    // Request::<I3B3A, I3AFt>::contract("a".parse().unwrap());
    // Request::<S3B, I3AFt>::contract("a".parse().unwrap());

    // // using the "impl" request builder, where the state is known (Struct1)
    // use super::impl_trait_1::method_ref_mut::Request;
    // let _promise = Request::contract("account.id".parse().unwrap())
    //     .args(true)
    //     .send_amount(0)
    //     .prepaid_gas(near_sdk::Gas::from(0))
    //     .request();

    // // using the "trait" request builder, where the state must be informed
    // type StructRequest = super::trait_1::method_ref_mut::Request<super::Struct1>;
    // StructRequest::contract("account.id".parse().unwrap())
    //     .args(true)
    //     .send_amount(0)
    //     .prepaid_gas(near_sdk::Gas::from(0))
    //     .request();
}
