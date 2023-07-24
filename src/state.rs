use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};


// contract info struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub owner: Addr,
    pub exchange_ratio: u128,
    pub min_exchange_amount: u32,
    pub first_winner_ratio: u8,
    pub second_winner_ratio: u8,
    pub owner_ratio: u8,
}

// Toke info struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Investor {
    pub addr: String,
    pub amount: Uint128,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Winner {
    pub addr: String,
    pub amount: Uint128,
}

// Investment round struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Investment {
    pub round: u32,
    pub total_amount: Uint128,
    pub in_progress: bool,
    pub first_winner: Option<Winner>,
    pub second_winner: Option<Winner>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Exchange {
    pub round: u32,
    pub total_amount: Uint128,
    // pub requesters: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Current {
    pub round: u32,
    pub exchange_round: u32,
}

pub const CURRENT: Item<Current> = Item::new("current");
pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");
pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const STAKING: Item<Uint128> = Item::new("staking_amount");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");

pub const INVESTMENTS: Map<String, Investment> = Map::new("investments");   // <round, Investment>
pub const INVESTORS: Map<(String, &Addr), Uint128> = Map::new("investors");
pub const EXCHANGES: Map<String, Exchange> = Map::new("exchanges");
pub const EXCHANGERS: Map<(String, &Addr), Uint128> = Map::new("exchangers");
