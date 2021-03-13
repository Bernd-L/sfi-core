use crate::{Inventory, Item, Unit};
use libocc::Projector;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, convert::TryFrom};

/// The serialized version of Store
#[derive(Deserialize, Serialize, Clone)]
struct StoreSer<'a> {
    inventories_projector: Projector<'a, Inventory>,
    items_projector: Projector<'a, Item>,
    units_projector: Projector<'a, Unit>,
}

#[serde(try_from = "StoreSer")]
#[serde(into = "StoreSer")]
#[derive(Deserialize, Serialize, Clone)]
pub struct Store<'a> {
    inventories: Vec<Cow<'a, Inventory>>,

    inventories_projector: Projector<'a, Inventory>,
    items_projector: Projector<'a, Item>,
    units_projector: Projector<'a, Unit>,
}

impl<'a> Into<StoreSer<'a>> for Store<'a> {
    fn into(self) -> StoreSer<'a> {
        StoreSer {
            inventories_projector: self.inventories_projector,
            items_projector: self.items_projector,
            units_projector: self.units_projector,
        }
    }
}

impl<'a> TryFrom<StoreSer<'a>> for Store<'a> {
    type Error = anyhow::Error;

    fn try_from(store: StoreSer<'a>) -> Result<Self, Self::Error> {
        // Make mutable projections
        let mut inventories = store.inventories_projector.get_projection().clone();
        let mut items = store.items_projector.get_projection().clone();
        let mut units = store.units_projector.get_projection().clone();

        for inventory in inventories {}

        Ok(Self {
            inventories,

            inventories_projector: store.inventories_projector,
            items_projector: store.items_projector,
            units_projector: store.units_projector,
        })
    }
}

impl<'a> Store<'a> {
    pub fn inventories(&self) -> &Vec<Cow<Inventory>> {
        self.inventories.get_projection()
    }

    pub fn items(&self) -> &Vec<Cow<Item>> {
        self.items.get_projection()
    }

    pub fn units(&self) -> &Vec<Cow<Unit>> {
        self.units.get_projection()
    }
}

impl<'a> Store<'a> {
    pub fn new() -> Self {
        Self {
            inventories: Projector::new(),
            items: Projector::new(),
            units: Projector::new(),
        }
    }
}
