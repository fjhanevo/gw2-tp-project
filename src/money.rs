use core::fmt;
use std::ops::{Add, Sub};



/// Use Copper to hold all currency in a single u64
#[derive(Debug, PartialEq)]
pub struct Money(pub u64);

impl Money {
    // Conversions constants
    pub const COPPER_PER_SILVER: u64    = 100;
    pub const COPPER_PER_GOLD: u64      = 10_000;

    /// Initialize a new instance of Money
    pub fn new(gold: u64, silver: u64, copper: u64) -> Self {
        let total = copper
            + (silver * Self::COPPER_PER_SILVER)
            + (gold * Self::COPPER_PER_GOLD);
        Self(total)
    }

    /// Return the total Money as a tuple (gold, silver, copper)
    pub fn unpack(&self) -> (u64, u64, u64) {
        let gold = self.0 / Self::COPPER_PER_GOLD; 
        let remainder = self.0 % Self::COPPER_PER_GOLD;
        let silver = remainder / Self::COPPER_PER_SILVER;
        let copper = remainder % Self::COPPER_PER_SILVER;
        (gold, silver, copper)
    }

    /// Return the amount of gold
    pub fn gold(&self) -> u64 {
        self.unpack().0
    }

    /// Return the amount of silver 
    pub fn silver(&self) -> u64 {
        self.unpack().1
    }

    /// Return the amount of copper 
    pub fn copper(&self) -> u64 {
        self.unpack().2
    }

}

// Get a nice output of the total amount of Money
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (g, s, c) = self.unpack();
        if g > 0 {
            write!(f, "{}g {}s {}c", g, s, c)
        }
        else if s > 0 {
            write!(f, "{}s {}c", s, c)
        }
        else {
            write!(f, "{}c", c)
        }
    }
}

// Overload the + operator
impl Add for Money {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

// Overload the - operator
impl Sub for Money {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_new_and_unpack() {
        // this should be 2g 50s 25c
        let money = Money::new(1, 150, 25);
        assert_eq!(money.unpack(), (2, 50, 25));
        assert_eq!(money.gold(), 2);
        assert_eq!(money.silver(), 50);
        assert_eq!(money.copper(), 25);
    }
    
    #[test]
    fn test_add_and_sub() {
        let wallet = Money::new(1, 50, 0);
        let cost = Money::new(0, 75, 0);
        let result = wallet - cost;
        assert_eq!(result, Money::new(0, 75, 0));
    }
    
    #[test]
    fn test_display_formatting() {
        assert_eq!(Money::new(1, 2, 3).to_string(), "1g 2s 3c");
        assert_eq!(Money::new(0, 5, 30).to_string(), "5s 30c");
        assert_eq!(Money::new(0, 0, 21).to_string(), "21c");
    }

    #[test]
    #[should_panic]
    fn test_underflow() {
        let wallet = Money::new(0, 0, 50);
        let cost = Money::new(0, 1, 0);
        let _ = wallet - cost; // should panic due to underflow
    }
}
