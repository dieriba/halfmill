pub(crate) const POOL_CONNECTIONS: u32 = 10;

pub mod config;
mod database;
mod error;
mod utils;
mod scripts;

pub use database::*;
pub use error::*;
pub use utils::*;
pub use scripts::*;