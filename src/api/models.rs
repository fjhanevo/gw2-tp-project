use serde::Deserialize;
use chrono::{DateTime, Utc};

/// Singular price for an item from /v2/commerce/prices
#[derive(Debug, Clone, Deserialize)]
pub struct AggregatedSide {
    pub unit_price: u32,
    pub quantity: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceLevel {
    pub listings: u32,
    pub unit_price: u32,
    pub quantity: u32,
}

/// Represents the full order book for a single item as 
/// given by /v2/commerce/listings
#[derive(Debug, Clone, Deserialize)]
pub struct Listings {
    pub id: u32,
    pub buys: Vec<PriceLevel>,
    pub sells: Vec<PriceLevel>, 
}

/// shape returned by /v2/commerce/prices for a single item
#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    pub id: u32,
    pub whitelisted: bool,
    pub buys: AggregatedSide,
    pub sells: AggregatedSide,
}

/// mirrors the price_snapshots table
pub struct PriceSnapshot {
    pub item_id: u32,
    pub captured_at: DateTime<Utc>,
    pub buy_price: u32,
    pub buy_quantity: u32,
    pub sell_price: u32,
    pub sell_quantity: u32,
}

impl From<Price> for PriceSnapshot {
    fn from(p: Price) -> Self {
        Self {
            item_id: p.id,
            captured_at: Utc::now(),
            buy_price: p.buys.unit_price, 
            buy_quantity: p.buys.quantity,
            sell_price: p.sells.unit_price,
            sell_quantity: p.sells.quantity,
        }
    }
}

enum Side {
    Buy,
    Sell
}

enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}
