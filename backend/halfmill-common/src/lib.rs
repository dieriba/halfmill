pub mod config;
mod database;

pub use database::Database;

#[cfg(test)]
mod tests {
    use super::*;
}
