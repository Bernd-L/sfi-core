use crate::{Inventory, Item, Unit};
use anyhow::{anyhow, bail};
use libocc::Projector;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

#[derive(Clone, Serialize, Deserialize)]
pub enum EventType {
    Inventory(Inventory),
    Item(Item),
    Unit(Unit),
}

impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Use the equality of the underlying types
            (EventType::Inventory(s), EventType::Inventory(o)) => s == o,
            (EventType::Item(s), EventType::Item(o)) => s == o,
            (EventType::Unit(s), EventType::Unit(o)) => s == o,

            // Non-matching variants can't be equal
            (EventType::Inventory(_), EventType::Item(_)) => false,
            (EventType::Inventory(_), EventType::Unit(_)) => false,
            (EventType::Item(_), EventType::Inventory(_)) => false,
            (EventType::Item(_), EventType::Unit(_)) => false,
            (EventType::Unit(_), EventType::Inventory(_)) => false,
            (EventType::Unit(_), EventType::Item(_)) => false,
        }
    }
}

/// The serialized version of Store
#[serde(try_from = "StoreSer")]
#[serde(into = "StoreSer")]
#[derive(Deserialize, Serialize, Clone)]
struct Store<'a> {
    inventory_handles: Vec<InventoryHandle<'a>>,
}

/// The serialized version of Store
#[derive(Deserialize, Serialize, Clone)]
struct StoreSer<'a> {
    inventory_projectors: Vec<Projector<'a, EventType>>,
}

impl<'a> Into<StoreSer<'a>> for Store<'a> {
    fn into(self) -> StoreSer<'a> {
        StoreSer {
            inventory_projectors: self
                .inventory_handles
                .into_iter()
                .map(|handle| handle.projector)
                .collect(),
        }
    }
}

impl<'a> TryFrom<StoreSer<'a>> for Store<'a> {
    type Error = anyhow::Error;

    fn try_from(ser: StoreSer<'a>) -> Result<Self, Self::Error> {
        let mut inventory_handles = vec![];

        for projector in ser.inventory_projectors {
            inventory_handles.push(projector.try_into()?);
        }

        Ok(Self { inventory_handles })
    }
}

#[derive(Clone)]
struct InventoryHandle<'a> {
    projector: Projector<'a, EventType>,
    inventory: Inventory,
}

impl<'a> TryFrom<Projector<'a, EventType>> for InventoryHandle<'a> {
    type Error = anyhow::Error;

    fn try_from(projector: Projector<'a, EventType>) -> Result<Self, Self::Error> {
        let mut inventory_option = None;
        let mut items = HashMap::new();
        let mut units = vec![];

        // Split up the entries in the projection based on their types
        for event in projector.get_projection().clone() {
            match event.into_owned() {
                EventType::Inventory(inventory) => {
                    if inventory_option.is_some() {
                        bail!("Cannot have two inventories in one projector");
                    } else {
                        inventory_option = Some(inventory);
                    }
                }
                EventType::Item(item) => {
                    if items.insert(item.uuid().clone(), item).is_some() {
                        bail!("Cannot have two items with the same uuid");
                    }
                }
                EventType::Unit(unit) => {
                    units.push(unit);
                }
            }
        }

        // Build the inventory to return
        let mut inventory = inventory_option.ok_or(anyhow!("No such inventory"))?;

        // Push the units into their respective items
        for unit in units {
            items
                .get_mut(unit.uuid())
                .ok_or(anyhow!("No such item"))?
                .units_mut()
                .push(unit)
        }

        // Push the items into the inventory (checked)
        for (_, item) in items {
            if item.inventory_uuid() != inventory.uuid() {
                bail!("The item is not part of the inventory");
            }

            inventory.items_mut().push(item);
        }

        // Return the projected inventory
        Ok(Self {
            projector,
            inventory,
        })
    }
}
