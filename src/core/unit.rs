use std::{
    sync::{Arc, Weak},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Item;
use crate::{Timestamp, Utc};

// #[serde(from = "UnitSer")]
// #[serde(into = "UnitSer")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Unit {
    /// The UUID of the unit
    pub uuid: Uuid,

    /// The item which this unit belongs to
    #[serde(skip)]
    pub item: Weak<Item>,

    /// The item which this unit belongs to
    pub item_uuid: Uuid,

    /// The duration after opening after which the unit expires (if applicable)
    pub use_up_after: Option<Duration>,

    /// The name of the item
    pub name: String,

    /// The percentage of how much of the unit is left
    pub percent_left: f64,

    /// The timestamp of the creation of the unit
    pub created_on: Timestamp,

    /// The timestamp this unit was opened for the first time (if ever)
    pub opened_on: Option<Timestamp>,
}

impl Unit {
    /// Generates a new unit
    pub fn new(
        item: &Arc<Item>,
        use_up_after: Option<Duration>,
        name: String,
        percent_left: f64,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            item: Arc::downgrade(item),
            item_uuid: item.uuid.clone(),
            use_up_after,
            name,
            percent_left,
            created_on: Utc::now(),
            opened_on: None,
        }
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
