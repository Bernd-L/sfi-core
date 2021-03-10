use crate::{Item, Timestamp};
use serde::{Deserialize, Serialize};
use std::{sync::Weak, time::Duration};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct Unit {
    /// The UUID of the unit
    uuid: Uuid,

    /// The item which this unit belongs to
    #[serde(skip)]
    inventory: Weak<Item>,

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
