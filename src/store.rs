use crate::{Inventory, Item, Unit};
use anyhow::{anyhow, bail, Result};
use libocc::{Event, Projector};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    convert::{TryFrom, TryInto},
    mem,
    ops::Deref,
    rc::Rc,
};
use uuid::Uuid;

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
pub struct Store<'a> {
    inventory_handles: Vec<InventoryHandle<'a>>,
}

impl<'a> Deref for Store<'a> {
    type Target = Vec<InventoryHandle<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.inventory_handles
    }
}

impl<'a> Store<'a> {
    pub fn new() -> Self {
        Self {
            inventory_handles: vec![],
        }
    }
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
pub struct InventoryHandle<'a> {
    projector: Projector<'a, ProjectionEntry>,
    inventory: Inventory,
}

impl<'a> Deref for InventoryHandle<'a> {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.inventory
    }
}

impl<'a> InventoryHandle<'a> {
    pub fn get_projector(&self) -> &Projector<'a, ProjectionEntry> {
        &self.projector
    }

    pub fn is_owned_by(&self, user_uuid: &Uuid) -> bool {
        self.inventory.owner() == user_uuid
    }

    pub fn allow_admin(&self, user_uuid: &Uuid) -> bool {
        self.is_owned_by(user_uuid) || self.inventory.admins().iter().any(|a| a == user_uuid)
    }

    pub fn allow_write(&self, user_uuid: &Uuid) -> bool {
        self.allow_admin(user_uuid) || self.inventory.writables().iter().any(|w| w == user_uuid)
    }

    pub fn allow_read(&self, user_uuid: &Uuid) -> bool {
        self.allow_write(user_uuid) || self.inventory.readables().iter().any(|w| w == user_uuid)
    }

    // There is no create_inventory(), as every inventory has its own event log

    pub fn update_inventory(&mut self, mut inventory: Inventory) -> Result<()> {
        // TODO check for update permissions

        // Preserve the items of the inventory
        *inventory.items_mut() = mem::take(self.inventory.items_mut());

        // Replace the target
        self.inventory = inventory.clone();

        // Make an event
        self.projector
            .push(Event::update(Cow::Owned(ProjectionEntry::Inventory(
                inventory,
            ))))
    }

    pub fn delete_inventory(&mut self, inventory: Inventory) -> Result<()> {
        // TODO check for update permissions

        // TODO Remove the index
        // mem::take(&mut self.inventory);

        // Make an event
        self.projector
            .push(Event::delete(Cow::Owned(ProjectionEntry::Inventory(
                inventory,
            ))))
    }

    pub fn create_item(&mut self, item: Item) -> Result<()> {
        // Make an event
        self.projector
            .push(Event::create(Cow::Owned(ProjectionEntry::Item(
                item.clone(),
            ))))?;

        self.inventory.items_mut().push(item);

        Ok(())
    }

    pub fn update_item(&mut self, mut item: Item) -> Result<()> {
        let target = self
            .inventory
            .items_mut()
            .iter_mut()
            .find(|i| i.uuid() == item.uuid())
            .ok_or(anyhow!("Item not found"))?;

        // Preserve the units of the item
        *item.units_mut() = mem::take(target.units_mut());

        // Replace the target
        *target = item.clone();

        // Make an event
        self.projector
            .push(Event::update(Cow::Owned(ProjectionEntry::Item(item))))
    }

    pub fn delete_item(&mut self, item: Item) -> Result<()> {
        let index = self
            .inventory
            .items_mut()
            .iter_mut()
            .position(|i| i.uuid() == item.uuid())
            .ok_or(anyhow!("Item not found"))?;

        // Remove the index
        self.inventory.items_mut().remove(index);

        // Make an event
        self.projector
            .push(Event::delete(Cow::Owned(ProjectionEntry::Item(item))))
    }

    pub fn create_unit(&mut self, unit: Unit) -> Result<()> {
        // Get the units of the associated item
        let units = self
            .inventory
            .items_mut()
            .iter_mut()
            .find(|i| i.uuid() == unit.item_uuid())
            .ok_or(anyhow!("Item not found"))?
            .units_mut();

        // Add the unit
        units.push(unit.clone());

        // Make an event
        self.projector
            .push(Event::create(Cow::Owned(ProjectionEntry::Unit(unit))))
    }

    pub fn update_unit(&mut self, unit: Unit) -> Result<()> {
        // Get the units of the associated item
        let units = self
            .inventory
            .items_mut()
            .iter_mut()
            .find(|i| i.uuid() == unit.item_uuid())
            .ok_or(anyhow!("Item not found"))?
            .units_mut();

        // Find the unt to replace
        let target = units
            .iter_mut()
            .find(|u| u.uuid() == unit.uuid())
            .ok_or(anyhow!("Unit not found"))?;

        // Replace the target
        *target = unit.clone();

        // Make an event
        self.projector
            .push(Event::update(Cow::Owned(ProjectionEntry::Unit(unit))))
    }

    pub fn delete_unit(&mut self, unit: Unit) -> Result<()> {
        // Get the units of the associated item
        let units = self
            .inventory
            .items_mut()
            .iter_mut()
            .find(|i| i.uuid() == unit.item_uuid())
            .ok_or(anyhow!("Item not found"))?
            .units_mut();

        // Find the index of the unit to be deleted
        let index = units
            .iter()
            .position(|u| u.uuid() == unit.uuid())
            .ok_or(anyhow!("Unit not found"))?;

        // Remove the index
        units.remove(index);

        // Make an event
        self.projector
            .push(Event::delete(Cow::Owned(ProjectionEntry::Unit(unit))))
    }
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
