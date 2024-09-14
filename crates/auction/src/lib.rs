//! Resolve auctions using a variety of algorithms.
#![allow(unused)]
#![warn(clippy::all, clippy::nursery)]
use uuid::Uuid;

mod strategies;

/// The Bid type.
#[derive(Debug, Clone, Copy)]
pub struct Bid {
    /// The bids unique identifier.
    id: Uuid,
    /// The bid in cents.
    amount: i64,
    /// The amount of desired units being bid on. Typically one.
    quantity: usize,
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bid {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl Eq for Bid {}

impl Bid {
    /// Creates a new bid.
    pub fn new(amount: i64, quantity: usize) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            amount,
            quantity,
        }
    }
}

#[macro_export]
macro_rules! bid {
    ($amount:literal, $quantity:literal) => {{
        Bid::new($amount, $quantity)
    }};
}

pub type Bids = Vec<Bid>;

/// The Sale type.
#[derive(Debug, Clone, Copy)]
pub struct Sale {
    bidder_id: Uuid,
    amount: i64,
    quantity: usize,
}

impl Sale {
    /// Create a new Sale associated with a given Bid.
    pub const fn new(bidder_id: Uuid, amount: i64, quantity: usize) -> Self {
        Self {
            bidder_id,
            amount,
            quantity,
        }
    }
}

pub type Sales = Vec<Sale>;

/// Enum representing valid auction strategies.
#[derive(Debug, Clone)]
pub enum AuctionStrategy {
    SinglePrice,
    MultiPrice,
}

/// The auction type.
#[derive(Debug, Clone)]
pub struct Auction {
    lots: usize,
    reserve_price: i64,
    strategy: AuctionStrategy,
}

impl Auction {
    /// Resolve the bids against the given auction
    pub fn resolve_bids(&self, mut bids: Bids) -> Sales {
        match self.strategy {
            AuctionStrategy::SinglePrice => strategies::single_price(self, bids),
            AuctionStrategy::MultiPrice => strategies::multi_price(self, bids),
        }
    }
}

/// The AuctionBuilder type. Used to easily create Auctions.
#[derive(Default)]
pub struct AuctionBuilder {
    lots: usize,
    reserve_price: Option<i64>,
    strategy: Option<AuctionStrategy>,
}

impl AuctionBuilder {
    /// Create a new builder.
    pub const fn new() -> Self {
        Self {
            lots: 1,
            reserve_price: None,
            strategy: None,
        }
    }

    /// Set the number of auction lots.
    pub const fn lots(mut self, lots: usize) -> Self {
        self.lots = lots;
        self
    }

    /// Set the reserve price of the auction.
    pub const fn reserve_price(mut self, reserve_price: i64) -> Self {
        self.reserve_price = Some(reserve_price);
        self
    }

    /// Set the strategy of the auction.
    pub const fn strategy(mut self, strategy: AuctionStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// Build the auction.
    pub fn build(self) -> Auction {
        Auction {
            lots: self.lots,
            reserve_price: self.reserve_price.unwrap_or_default(),
            strategy: self.strategy.unwrap_or(AuctionStrategy::SinglePrice),
        }
    }
}
