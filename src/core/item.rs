use std::sync::{Arc, RwLock, Weak};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Inventory, Unit};
use crate::{Timestamp, Utc};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Item {
    /// The UUID of the item
    pub uuid: Uuid,

    // /// The inventory which this item belongs to
    // #[serde(skip)]
    // pub inventory: Weak<Inventory>,
    /// The inventory which this item belongs to
    pub inventory_uuid: Uuid,

    /// The name of the item
    pub name: String,

    /// The units ot the item
    pub units: Vec<Unit>,

    /// The timestamp of the creation of the item
    pub created_on: Timestamp,

    /// The EAN code of the item
    pub ean: Option<String>,
    // TODO categories
}

impl Item {
    /// Generates a new item
    pub fn new(inventory_uuid: Uuid, name: String, ean: Option<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            // inventory: Arc::downgrade(inventory.read().unwrap()),
            inventory_uuid,
            name,
            units: vec![],
            created_on: Utc::now(),
            ean,
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
