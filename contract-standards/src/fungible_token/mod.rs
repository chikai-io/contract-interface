pub mod core;
#[macro_use]
pub mod core_impl;
pub mod macros;
pub mod metadata;
pub mod receiver;
pub mod resolver;
#[macro_use]
pub mod storage_impl;

pub use self::core::FungibleTokenCore;
pub use core_impl::FungibleToken;
