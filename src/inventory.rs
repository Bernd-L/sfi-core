use crate::{Item, Timestamp};
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
