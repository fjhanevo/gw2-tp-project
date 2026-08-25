//TODO: Fix other files before this
use crate::money::Money;


pub enum TransactionError {
    InsufficientFunds,
    WalletOverflow,
}

#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub wallet: Money,
}

impl Account {

    pub const LISTING_FEE: f64 = 0.05;  
    pub const TRANSACTION_FEE: f64 = 0.10;
    pub fn new(name: impl Into<String>, initial_balance: Money) -> Self {
        Self {
            name: name.into(),
            wallet: initial_balance,
        }
    }
}
