mod lens;

pub use lens::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::marker::PhantomData;

/*
pub trait Inherit {
    fn inherit_ref<Inner>(&self) -> Inherited<&Self, &Inner>
    where
        Self: Lens<Inner>,
    {
        Inherited {
            outer: self,
            inner: PhantomData,
        }
    }
    fn inherit_mut<Inner>(&mut self) -> Inherited<&mut Self, &mut Inner>
    where
        Self: Lens<Inner>,
    {
        Inherited {
            outer: self,
            inner: PhantomData,
        }
    }
    fn inherit_owned<Inner>(self) -> Inherited<Self, Inner>
    where
        Self: Sized + Lens<Inner>,
    {
        Inherited {
            outer: self,
            inner: PhantomData,
        }
    }
}

impl<Outer> Inherit for Outer {}

// TODO: Default and De/Serialize stuff should not be needed
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Inherited<Outer, Inner> {
    pub outer: Outer,
    #[borsh_skip]
    pub inner: PhantomData<Inner>,
}

impl<Outer, Inner> Lens<Inner> for Inherited<Outer, Inner>
where
    Outer: Lens<Inner>,
{
    fn with_ref<V, F>(&self, f: F) -> V
    where
        F: FnOnce(&Inner) -> V,
    {
        self.outer.with_ref(f)
    }

    fn with_mut<V, F>(&mut self, f: F) -> V
    where
        F: FnOnce(&mut Inner) -> V,
    {
        self.outer.with_mut(f)
    }
}

impl<Outer, Inner> From<Outer> for Inherited<Outer, Inner>
where
    Outer: Lens<Inner>,
{
    fn from(outer: Outer) -> Self {
        Self {
            outer,
            inner: PhantomData,
        }
    }
}
*/
