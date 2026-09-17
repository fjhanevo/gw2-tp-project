use std::collections::HashMap;


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

/// Represents the inventory of the account with Item and quantity
#[derive(Debug, Clone, Default)]
pub struct Inventory {
    pub items: HashMap<Item, u32>,
}

impl Inventory {
    pub fn add(&mut self, item: Item, qunatity: u32) {
        *self.items.entry(item).or_insert(0) += qunatity;
    }

    pub fn remove(&mut self, item: &Item, quantity: u32) -> Result<(), &'static str> {
        let entry = self.items.get_mut(item).ok_or("Item not in Inventory")?;
        if *entry < quantity {
            return Err("Not enough items in inventory!");
        }
        *entry -= quantity;
        if *entry == 0 {
            self.items.remove(item);
        }
        Ok(())
    }
}
