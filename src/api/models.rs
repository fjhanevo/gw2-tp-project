use serde::Deserialize;

/// 
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

#[derive(Debug, Clone, Deserialize)]
pub struct Prices {}
