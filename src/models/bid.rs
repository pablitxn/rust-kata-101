use super::money_exchange::MoneyExchange;
use super::participant::Participant;

#[derive(Copy, Clone)]
pub struct Bid {
    pub amount: MoneyExchange,
    pub participant: Participant,
}
