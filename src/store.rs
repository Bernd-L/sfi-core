use crate::{Inventory, Item, Unit};
use anyhow::{anyhow, bail};
use libocc::Projector;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

#[derive(Clone, Serialize, Deserialize)]
pub enum ProjectionEntry {
    Inventory(Inventory),
    Item(Item),
    Unit(Unit),
}

impl PartialEq for ProjectionEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Use the equality of the underlying types
            (ProjectionEntry::Inventory(s), ProjectionEntry::Inventory(o)) => s == o,
            (ProjectionEntry::Item(s), ProjectionEntry::Item(o)) => s == o,
            (ProjectionEntry::Unit(s), ProjectionEntry::Unit(o)) => s == o,

            // Non-matching variants can't be equal
            (ProjectionEntry::Inventory(_), ProjectionEntry::Item(_)) => false,
            (ProjectionEntry::Inventory(_), ProjectionEntry::Unit(_)) => false,
            (ProjectionEntry::Item(_), ProjectionEntry::Inventory(_)) => false,
            (ProjectionEntry::Item(_), ProjectionEntry::Unit(_)) => false,
            (ProjectionEntry::Unit(_), ProjectionEntry::Inventory(_)) => false,
            (ProjectionEntry::Unit(_), ProjectionEntry::Item(_)) => false,
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
    inventory_projectors: Vec<Projector<'a, ProjectionEntry>>,
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
    projector: Projector<'a, ProjectionEntry>,
    inventory: Inventory,
}

impl<'a> TryFrom<Projector<'a, ProjectionEntry>> for InventoryHandle<'a> {
    type Error = anyhow::Error;

    fn try_from(projector: Projector<'a, ProjectionEntry>) -> Result<Self, Self::Error> {
        let mut inventory_option = None;
        let mut items = HashMap::new();
        let mut units = vec![];

        // Split up the entries in the projection based on their types
        for projected_thing in projector.get_projection().clone() {
            match projected_thing.into_owned() {
                ProjectionEntry::Inventory(inventory) => {
                    if inventory_option.is_some() {
                        bail!("Cannot have two inventories in one projector");
                    } else {
                        inventory_option = Some(inventory);
                    }
                }
                ProjectionEntry::Item(item) => {
                    if items.insert(item.uuid().clone(), item).is_some() {
                        bail!("Cannot have two items with the same uuid");
                    }
                }
                ProjectionEntry::Unit(unit) => {
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
