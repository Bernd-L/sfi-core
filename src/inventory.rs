use crate::{Item, Timestamp};
use libocc::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct Inventory {
    /// The UUID of the inventory
    uuid: Uuid,

    /// The items in the inventory
    items: Vec<Item>,

    /// The name of the inventory
    name: String,

    /// The timestamp of the creation of the inventory
    created_on: Timestamp,

    /// The UUID of the owner of the inventory
    owner: Uuid,

    /// A list of UUIDs of users who have administrative access to this inventory
    admins: Vec<Uuid>,

    /// A list of UUIDs of users who have administrative write to this inventory
    writables: Vec<Uuid>,

    /// A list of UUIDs of users who have administrative read-only to this inventory
    readables: Vec<Uuid>,
}

impl Inventory {
    /// Generates a new inventory
    pub fn new(name: String, owner: Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            items: vec![],
            name,
            created_on: Utc::now(),
            owner,
            admins: vec![],
            writables: vec![],
            readables: vec![],
        }
    }

    pub(super) fn from_event_log() {}
}

impl PartialEq for Inventory {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

// Getters
impl Inventory {
    /// The UUID of the inventory
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// The items in the inventory
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    /// The items in the inventory
    pub(super) fn items_mut(&mut self) -> &mut Vec<Item> {
        &mut self.items
    }

    /// The name of the inventory
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The timestamp of the creation of the inventory
    pub fn created_on(&self) -> &Timestamp {
        &self.created_on
    }

    /// The UUID of the owner of the inventory
    pub fn owner(&self) -> &Uuid {
        &self.owner
    }

    /// A list of UUIDs of users who have administrative access to this inventory
    pub fn admins(&self) -> &Vec<Uuid> {
        &self.admins
    }

    /// A list of UUIDs of users who have administrative write to this inventory
    pub fn writables(&self) -> &Vec<Uuid> {
        &self.writables
    }

    /// A list of UUIDs of users who have administrative read-only to this inventory
    pub fn readables(&self) -> &Vec<Uuid> {
        &self.readables
    }
}
