use crate::{Inventory, Timestamp, Unit};
use serde::{Deserialize, Serialize};
use std::sync::Weak;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct Item {
    /// The UUID of the item
    uuid: Uuid,

    /// The inventory which this item belongs to
    #[serde(skip)]
    inventory: Weak<Inventory>,

    /// The name of the item
    name: String,

    /// The units ot the item
    units: Vec<Unit>,

    /// The timestamp of the creation of the item
    created_on: Timestamp,
    // TODO categories
}
