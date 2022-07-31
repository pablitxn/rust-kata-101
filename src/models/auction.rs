use super::bid::Bid;
use super::money_exchange::MoneyExchange;
use super::Reputation;
use chrono::{DateTime, Utc};

pub struct Auction {
    pub base_price: MoneyExchange,
    pub discoverable: bool,
    pub best_bid: Option<Bid>,
    pub bids: Vec<Bid>,
    pub min_reputation: Option<Reputation>,
    pub start_at: DateTime<Utc>,
}

impl Auction {
    pub fn create_auction(
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

    pub fn finish_auction(auction: Auction) -> Option<Bid> {
        auction.best_bid
    }

    pub fn make_bid(&mut self, bid: Bid) {
        if self.best_bid.is_none() {
            self.bids.push(bid);
            self.best_bid = Some(bid);
            return;
        } else {
            let best_bid = self.best_bid.as_ref().unwrap();
            if bid.amount > best_bid.amount {
                self.bids.push(bid);
                self.best_bid = Some(bid);
                return;
            }
        }
    }
}

// #[derive(Debug, PartialEq, PartialOrd)]
