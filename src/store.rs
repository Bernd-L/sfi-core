use std::convert::TryFrom;

use crate::{Inventory, Item, Unit};
use libocc::Projector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum EventType {
    Inventory(Inventory),
    Item(Item),
    Unit(Unit),
}

impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            // Use the equality of the underlying types
            (EventType::Inventory(s), EventType::Inventory(o)) => s == o,
            (EventType::Item(s), EventType::Item(o)) => s == o,
            (EventType::Unit(s), EventType::Unit(o)) => s == o,

            // Non-matching variants can't be equal
            (EventType::Inventory(_), EventType::Item(_)) => false,
            (EventType::Inventory(_), EventType::Unit(_)) => false,
            (EventType::Item(_), EventType::Inventory(_)) => false,
            (EventType::Item(_), EventType::Unit(_)) => false,
            (EventType::Unit(_), EventType::Inventory(_)) => false,
            (EventType::Unit(_), EventType::Item(_)) => false,
        }
    }
}

/// The serialized version of Store
#[serde(try_from = "StoreSer")]
#[serde(into = "StoreSer")]
#[derive(Deserialize, Serialize, Clone)]
struct Store<'a> {
    inventories: Vec<Projector<'a, EventType>>,
}

/// The serialized version of Store
#[derive(Deserialize, Serialize, Clone)]
struct StoreSer<'a> {
    inventories: Vec<Projector<'a, EventType>>,
}

impl<'a> Into<StoreSer<'a>> for Store<'a> {
    fn into(self) -> StoreSer<'a> {
        todo!()
    }
}

impl<'a> TryFrom<StoreSer<'a>> for Store<'a> {
    type Error = anyhow::Error;

    fn try_from(value: StoreSer<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}
