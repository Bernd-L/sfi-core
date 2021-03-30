use crate::{Inventory, Timestamp, Unit};
use libocc::Utc;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Weak};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Item {
    /// The UUID of the item
    uuid: Uuid,

    /// The inventory which this item belongs to
    #[serde(skip)]
    inventory: Weak<Inventory>,

    /// The inventory which this item belongs to
    inventory_uuid: Uuid,

    /// The name of the item
    name: String,

    /// The units ot the item
    units: Vec<Unit>,

    /// The timestamp of the creation of the item
    created_on: Timestamp,

    /// The EAN code of the item
    ean: Option<String>,
    // TODO categories
}

impl Item {
    /// Generates a new item
    pub(super) fn new(inventory: &Arc<Inventory>, name: String, ean: Option<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            inventory: Arc::downgrade(inventory),
            inventory_uuid: inventory.uuid().clone(),
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

    /// The inventory which this item belongs to
    pub(super) fn inventory_uuid(&self) -> &Uuid {
        &self.inventory_uuid
    }

    /// The name of the item
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The units ot the item
    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    /// The units ot the item
    pub(super) fn units_mut(&mut self) -> &mut Vec<Unit> {
        &mut self.units
    }

    /// The timestamp of the creation of the item
    pub fn created_on(&self) -> &Timestamp {
        &self.created_on
    }
}

impl Item {}
