use super::models::bid::Bid;
use super::models::money_exchange::MoneyExchange;
use super::models::Reputation;
use chrono::{DateTime, Utc};

pub struct Auction {
    base_price: MoneyExchange,
    discoverable: bool,
    best_bid: Option<Bid>,
    pub bids: Vec<Bid>,
    min_reputation: Option<Reputation>,
    start_at: DateTime<Utc>,
}

fn create_auction(
    base_price: MoneyExchange,
    discoverable: bool,
    min_reputation: Option<Reputation>,
) -> Auction {
    Auction {
        base_price,
        discoverable,
        best_bid: None,
        bids: Vec::new(),
        min_reputation,
        start_at: Utc::now(),
    }
}

fn finish_auction(auction: Auction) -> Option<Bid> {
    auction.best_bid
}
