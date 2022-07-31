pub mod models;

fn main() {}

// las subastas deben tener la categoria "discoverable" // SOLVED

// las subastas deben suceder en tiempo real

// las subastas deben estar en un tipo de moneda "money exchange" // SOLVED

// crear una subasta apartir de una reputacion minima

// random bids para subasta en vivo // SOLVED

// retornar url al video de la subasta (metricas) desp de que finaliza -------------

#[cfg(test)]
mod tests {
    use super::models::auction::Auction;
    use super::models::money_exchange::MoneyExchange;
    use super::models::bid::Bid;
    use super::models::participant::Participant;

    #[test]
    fn get_best_bid_by_value() {
        let mut auction = Auction::create_auction(MoneyExchange::Ars(10), true, Some(10));

        let max_bid = Bid {
            amount: 100,
            participant: Participant { reputation: 9 },
        };

        let min_bid = Bid {
            amount: 10,
            participant: Participant { reputation: 10 },
        };

        Auction::make_bid(&mut auction, max_bid);
        Auction::make_bid(&mut auction, min_bid);

        let best_bid = auction.best_bid.unwrap();
        let best_bid_value = max_bid.amount;

        assert_eq!(best_bid_value, best_bid.amount);
    }

    // los participantes pueden ser rastreados por reputacion
}
