use crate::{Inventory, Timestamp, Unit};
use libocc::Utc;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Weak};
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

impl Item {
    /// Generates a new item
    pub fn new(inventory: &Arc<Inventory>, name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            inventory: Arc::downgrade(inventory),
            name,
            units: vec![],
            created_on: Utc::now(),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

// Getters
impl Item {
    /// The UUID of the item
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// The inventory which this item belongs to
    pub fn inventory(&self) -> &Weak<Inventory> {
        &self.inventory
    }

    /// The name of the item
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The units ot the item
    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    /// The timestamp of the creation of the item
    pub fn created_on(&self) -> &Timestamp {
        &self.created_on
    }
}
