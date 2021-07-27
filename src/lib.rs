#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod tree;

pub mod core;
pub mod events;

pub use libocc::events::{Timestamp, Utc};

#[cfg(test)]
mod tests {
    // TODO implement unit tests (when there is business logic to be tested)
}
