// use super::money_exchange::MoneyExchange;
use super::participant::Participant;

#[derive(Copy, Clone)]
pub struct Bid {
    // pub amount: MoneyExchange,
    pub amount: u32,
    pub participant: Participant,
}
