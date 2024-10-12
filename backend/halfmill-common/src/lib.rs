pub mod config;
mod database;

pub use database::*;

#[cfg(test)]
mod tests {
    use super::*;
}
