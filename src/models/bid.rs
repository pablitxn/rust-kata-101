use super::money_exchange::MoneyExchange;
use super::participant::Participant;
use crate::auction::Auction;

pub struct Bid {
    pub amount: MoneyExchange,
    pub participant: Participant,
}

fn make_bid(_auction: &mut Auction, bid: Bid) {
    _auction.bids.push(bid);
}

fn random_bid_from_live_auction() -> Bid {
    Bid {
        amount: MoneyExchange::Ars(10),
        participant: Participant { reputation: 9 },
    }
}
