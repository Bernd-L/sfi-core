use std::borrow::Cow;

use crate::{store::ProjectionEntry, Item, Timestamp};
use libocc::{Event, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
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
    pub(super) fn new(name: String, owner: Uuid, uuid: Uuid, created_on: Timestamp) -> Self {
        Self {
            uuid,
            items: vec![],
            name,
            created_on,
            owner,
            admins: vec![],
            writables: vec![],
            readables: vec![],
        }
    }
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

// Changers
impl<'a> Inventory {
    /// Generates a new inventory (and returns the associated event together with the new timestamp and the new UUID)
    pub fn create(name: String, owner: Uuid) -> (Event<'a, ProjectionEntry>, Uuid, Timestamp) {
        let uuid = Uuid::new_v4();
        let created_on = Utc::now();

        (
            Event::create(Cow::Owned(ProjectionEntry::Inventory(Self {
                uuid,
                items: vec![],
                name,
                created_on,
                owner,
                admins: vec![],
                writables: vec![],
                readables: vec![],
            }))),
            uuid,
            created_on,
        )
    }

    pub fn delete(self) -> Event<'a, ProjectionEntry> {
        Event::delete(Cow::Owned(ProjectionEntry::Inventory(Self {
            items: vec![],
            ..self
        })))
    }

    pub fn update_name(self, name: String) -> Event<'a, ProjectionEntry> {
        Event::update(Cow::Owned(ProjectionEntry::Inventory(Self {
            items: vec![],
            name,
            ..self
        })))
    }

    pub fn update_owner(self, owner: Uuid) -> Event<'a, ProjectionEntry> {
        Event::update(Cow::Owned(ProjectionEntry::Inventory(Self {
            items: vec![],
            owner,
            ..self
        })))
    }

    pub fn update_admins(self, admins: Vec<Uuid>) -> Event<'a, ProjectionEntry> {
        Event::update(Cow::Owned(ProjectionEntry::Inventory(Self {
            items: vec![],
            admins,
            ..self
        })))
    }

    pub fn update_writables(self, writables: Vec<Uuid>) -> Event<'a, ProjectionEntry> {
        Event::update(Cow::Owned(ProjectionEntry::Inventory(Self {
            items: vec![],
            writables,
            ..self
        })))
    }

    pub fn update_readables(self, readables: Vec<Uuid>) -> Event<'a, ProjectionEntry> {
        Event::update(Cow::Owned(ProjectionEntry::Inventory(Self {
            items: vec![],
            readables,
            ..self
        })))
    }
}
