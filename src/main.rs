pub mod models;

fn main() {}

#[cfg(test)]
mod tests {
    use super::models::auction::Auction;
    use super::models::bid::Bid;
    use super::models::money_exchange::MoneyExchange;
    use super::models::participant::Participant;

    #[test]
    fn get_best_bid_by_value() {
        let mut auction = Auction::create_auction(MoneyExchange::Ars(10), true, Some(10));

        let max_bid = Bid {
            amount: MoneyExchange::Ars(20),
            participant: Participant { reputation: 9 },
        };

        let min_bid = Bid {
            amount: MoneyExchange::Ars(20),
            participant: Participant { reputation: 10 },
        };

        Auction::make_bid(&mut auction, max_bid);
        Auction::make_bid(&mut auction, min_bid);

        let best_bid = auction.best_bid.unwrap();
        let best_bid_value = max_bid.amount;

        assert_eq!(best_bid_value, best_bid.amount);
    }
}
