pub mod config;
mod database;

pub use database::*;

pub(crate) const POOL_CONNECTIONS: u32 = 10;

#[cfg(test)]
mod tests {
    use super::*;
}
