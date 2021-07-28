use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Item;
use crate::{Timestamp, Utc};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Inventory {
    /// The UUID of the inventory
    pub uuid: Uuid,

    /// The items in the inventory
    pub items: Vec<Item>,

    /// The name of the inventory
    pub name: String,

    /// The timestamp of the creation of the inventory
    pub created_on: Timestamp,

    /// The UUID of the owner of the inventory
    pub owner: Uuid,

    /// A list of UUIDs of users who have administrative access to this inventory
    pub admins: Vec<Uuid>,

    /// A list of UUIDs of users who have administrative write to this inventory
    pub writables: Vec<Uuid>,

    /// A list of UUIDs of users who have administrative read-only to this inventory
    pub readables: Vec<Uuid>,
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
}

impl PartialEq for Inventory {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Inventory {
    pub fn is_owned_by(&self, user_uuid: &Uuid) -> bool {
        self.owner == *user_uuid
    }

    pub fn allow_admin(&self, user_uuid: &Uuid) -> bool {
        self.is_owned_by(user_uuid) || self.admins.iter().any(|a| a == user_uuid)
    }

    pub fn allow_write(&self, user_uuid: &Uuid) -> bool {
        self.allow_admin(user_uuid) || self.writables.iter().any(|w| w == user_uuid)
    }

    pub fn allow_read(&self, user_uuid: &Uuid) -> bool {
        self.allow_write(user_uuid) || self.readables.iter().any(|w| w == user_uuid)
    }
}
