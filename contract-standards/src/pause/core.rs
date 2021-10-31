use contract_interface::contract;

#[contract]
pub trait PauseCore {
    fn pause(&mut self, p: bool);
    fn is_paused(&self) -> bool;
}
