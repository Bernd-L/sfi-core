use crate::{Item, Timestamp};
use libocc::Utc;
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Weak},
    time::Duration,
};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct Unit {
    /// The UUID of the unit
    uuid: Uuid,

    /// The item which this unit belongs to
    #[serde(skip)]
    item: Weak<Item>,

    /// The duration after opening after which the unit expires (if applicable)
    use_up_after: Option<Duration>,

    /// The name of the item
    name: String,

    /// The percentage of how much of the unit is left
    percent_left: f64,

    /// The timestamp of the creation of the unit
    created_on: Timestamp,

    /// The timestamp this unit was opened for the first time (if ever)
    opened_on: Option<Timestamp>,
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

impl Unit {
    /// The UUID of the unit
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// The item which this unit belongs to
    pub fn inventory(&self) -> &Weak<Item> {
        &self.item
    }

    /// The duration after opening after which the unit expires (if applicable)
    pub fn use_up_after(&self) -> &Option<Duration> {
        &self.use_up_after
    }

    /// The name of the item
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The percentage of how much of the unit is left
    pub fn percent_left(&self) -> &f64 {
        &self.percent_left
    }

    /// The timestamp of the creation of the unit
    pub fn created_on(&self) -> &Timestamp {
        &self.created_on
    }

    /// The timestamp this unit was opened for the first time (if ever)
    pub fn opened_on(&self) -> &Option<Timestamp> {
        &self.opened_on
    }
}
