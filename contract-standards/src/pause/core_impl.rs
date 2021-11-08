use crate::pause::core::pause_core;
use contract_interface::contract;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pause<T> {
    pub inner: T,
    pub paused: bool,
}

impl<T> Default for Pause<T>
where
    T: Default,
{
    fn default() -> Self {
        todo!("copy PanicOnDefault behavior")
    }
}

impl<Ft> Pause<Ft> {
    pub fn panic_if_paused(&self) {
        use near_sdk::require;
        require!(!self.paused, "Contract Is Paused");
    }
}

#[contract(mod = "impl_pause", trait = "pause_core")]
impl<T> crate::pause::PauseCore for Pause<T>
where
    T: Default + BorshSerialize + BorshDeserialize,
{
    fn pause(&mut self, p: bool) {
        self.paused = p;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}
