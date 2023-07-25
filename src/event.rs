use cosmwasm_std::{attr, Response, Uint128};

pub trait Event {
    /// Append attributes to response
    fn add_attributes(&self, response: &mut Response);
}

/// Invest actions
pub struct InvestedEvent<'a> {
    pub round: u32,
    pub who: &'a str,
    pub amount: Uint128,
}

impl<'a> Event for InvestedEvent<'a> {
    fn add_attributes(&self, rsp: &mut Response) {
        rsp.attributes.push(attr("action", "Invested"));
        rsp.attributes.push(attr("round", self.round.to_string()));
        rsp.attributes.push(attr("who", self.who));
        rsp.attributes.push(attr("amount", self.amount.to_string()));
    }
}

/// Token Transfer actions
pub struct TokenTransferredEvent<'a> {
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
    pub amount: Uint128,
}

impl<'a> Event for TokenTransferredEvent<'a> {
    fn add_attributes(&self, rsp: &mut Response) {
        rsp.attributes.push(attr("action", "TokenTransferred"));
        rsp.attributes.push(attr("from", self.from.unwrap_or("").to_string()));
        rsp.attributes.push(attr("to", self.to.unwrap_or("").to_string()));
        rsp.attributes.push(attr("amount", self.amount.to_string()));
    }
}

/// ClosedInvestment action
pub struct ClosedInvestmentEvent<'a> {
    pub round: u32,
    pub first_winner: &'a str,
    pub second_winner: &'a str,
    pub winner_hash: &'a str,
}

impl<'a> Event for ClosedInvestmentEvent<'a> {
    fn add_attributes(&self, rsp: &mut Response) {
        rsp.attributes.push(attr("action", "ClosedInvestment"));
        rsp.attributes.push(attr("first_winner", self.first_winner.to_string()));
        rsp.attributes.push(attr("second_winner", self.second_winner.to_string()));
        rsp.attributes.push(attr("winner_hash", self.winner_hash.to_string()));
    }
}
