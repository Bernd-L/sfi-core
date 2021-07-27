#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

pub mod node;
pub mod store;
pub mod users;

mod inventory;
mod item;
mod quantity;
mod unit;

pub use inventory::*;
pub use item::*;
pub use quantity::*;
pub use unit::*;

pub use libocc::Timestamp;

#[cfg(test)]
mod tests {
    // TODO implement unit tests (when there is business logic to be tested)
}
