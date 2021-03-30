use crate::{
    store::{ProjectionEntry, ProjectionEvent},
    Inventory, Timestamp, Unit,
};
use libocc::{Event, Utc};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    sync::{Arc, Weak},
};
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

impl<'a> Item {
    /// Generates a new inventory (and returns the associated event together with the new timestamp and the new UUID)
    pub fn create(
        inventory: &Arc<Inventory>,
        name: String,
        ean: Option<String>,
    ) -> ProjectionEvent<'a> {
        let uuid = Uuid::new_v4();
        let created_on = Utc::now();

        Event::create(Cow::Owned(ProjectionEntry::Item(Self {
            uuid,
            inventory: Arc::downgrade(inventory),
            inventory_uuid: inventory.uuid().clone(),
            name,
            units: vec![],
            created_on,
            ean,
        })))
    }

    pub fn delete(self) -> ProjectionEvent<'a> {
        // TODO notify the inventory of the deletion of this item
        Event::delete(Cow::Owned(ProjectionEntry::Item(Self {
            units: vec![],
            inventory: Weak::new(),
            ..self
        })))
    }

    pub fn update_inventory(self, inventory: &Arc<Inventory>) -> ProjectionEvent<'a> {
        // TODO notify the inventories of the moving of this item
        Event::update(Cow::Owned(ProjectionEntry::Item(Self {
            inventory: Arc::downgrade(inventory),
            ..self
        })))
    }

    pub fn update_name(self, name: String) -> ProjectionEvent<'a> {
        Event::update(Cow::Owned(ProjectionEntry::Item(Self { name, ..self })))
    }

    // TODO implement some kind of access to push, find and so on of the units vev
    // pub fn update_units(self, units: Vec<>) -> ProjectionEvent<'a> {
    //     Event::update(Cow::Owned(ProjectionEntry::Item(Self {
    //         units,
    //         ..self
    //     })))
    // }

    pub fn update_ean(self, ean: Option<String>) -> ProjectionEvent<'a> {
        Event::update(Cow::Owned(ProjectionEntry::Item(Self { ean, ..self })))
    }
}
