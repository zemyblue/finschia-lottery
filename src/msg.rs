use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub use_denom: String,     // denoms to treat
    pub exchange_ratio: u128,    // coin: token = 1: exchange_ratio
    pub min_exchange_amount: u32,
    pub first_winner_ratio: u8,
    pub second_winner_ratio: u8,
    pub owner_ratio: u8,

    pub token_name: String,
    pub token_symbol: String, 
    pub token_decimals: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Invest {},
    CloseInvestment {},
    // DistributeReward {},
    // RequestExchangeToken { amount: Uint128 },
    // CacelExchangeToken { amount: Uint128 },
    // StartExchange {},
    // EndExchange { round: Uint128 },
    // TransferToken { to: String, amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Info {},
    CurrentRound {},
    CurrentInvestors { start_after: Option<String>, limit: Option<u32> },
    Investors { round: u32, start_after: Option<String>, limit: Option<u32> },
    InvestResult { round: u32, },
    TotalTokenSupply {},
    TokenBalance { who: Addr },
    // CurrentExchangeRound {},
    // CurrentExchangeRequesters { start_after: Option<String>, limit: Option<u32> },
    // ExchangeResult { round: Uint128 },
    // ExchangeRequesters { round: Uint128, start_after: Option<String>, limit: Option<u32> }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InfoResponse {
    pub exchange_ratio: u128,    // coin: token = 1: exchange_ratio
    pub min_exchange_amount: u32,
    pub first_winner_ratio: u8,
    pub second_winner_ratio: u8,
    pub owner_ratio: u8,

    pub token_name: String,
    pub token_symbol: String, 
    pub token_decimals: u8,
}
