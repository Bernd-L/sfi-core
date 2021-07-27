use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Quantity {
    Pieces(u64),
    Kg(u64),
    G(u64),
    Cans(u64),
    Packs(u64),
    Other(String),
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Quantity::Pieces(n) if n == 1 => write!(f, "{} piece", n),
            Quantity::Pieces(n) => write!(f, "{} pieces", n),
            Quantity::Kg(n) => write!(f, "{} kg", n),
            Quantity::G(n) => write!(f, "{} g", n),
            Quantity::Cans(n) => write!(f, "{} cans", n),
            Quantity::Packs(n) => write!(f, "{} packs", n),
            Quantity::Other(ref s) => f.write_str(s),
        }
    }
}
