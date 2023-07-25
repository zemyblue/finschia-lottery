use crate::msg::{InfoResponse, QueryMsg};
use crate::state::{
    Investor, Winner, BALANCES, CONTRACT_INFO, CURRENT, INVESTMENTS, INVESTORS, TOKEN_INFO,
};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, Env, Order, StdError, StdResult, Uint128,
};
use cw_storage_plus::Bound;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Info {} => to_binary(&query_contract_info(deps)?),
        QueryMsg::CurrentRound {} => to_binary(&query_current_round(deps)?),
        QueryMsg::CurrentInvestment {} => to_binary(&query_current_investment(deps)?),
        QueryMsg::CurrentInvestors { start_after, limit } => {
            to_binary(&query_current_investors(deps, start_after, limit)?)
        }
        QueryMsg::Investors {
            round,
            start_after,
            limit,
        } => to_binary(&query_investors(deps, round, start_after, limit)?),
        QueryMsg::InvestResult { round } => to_binary(&query_invest_result(deps, round)?),
        QueryMsg::TotalTokenSupply {} => to_binary(&query_token_total_supply(deps)?),
        QueryMsg::TokenBalance { who } => to_binary(&query_token_balance(deps, who)?),
    }
}

fn query_contract_info(deps: Deps) -> StdResult<InfoResponse> {
    let contract = CONTRACT_INFO.load(deps.storage)?;
    let token = TOKEN_INFO.load(deps.storage)?;
    Ok(InfoResponse {
        exchange_ratio: contract.exchange_ratio,
        min_exchange_amount: contract.min_exchange_amount,
        first_winner_ratio: contract.first_winner_ratio,
        second_winner_ratio: contract.second_winner_ratio,
        owner_ratio: contract.owner_ratio,
        token_name: token.name.clone(),
        token_symbol: token.symbol.clone(),
        token_decimals: token.decimals,
    })
}

pub fn query_current_round(deps: Deps) -> StdResult<CurrentRoundResponse> {
    let round = CURRENT.load(deps.storage)?.round;
    Ok(CurrentRoundResponse { round })
}

pub fn query_current_investment(deps: Deps) -> StdResult<CurrentInvestmentResponse> {
    let round = query_current_round(deps)?.round;
    let investment = INVESTMENTS.load(deps.storage, round.to_string())?;
    Ok(CurrentInvestmentResponse { round: investment.round, total_amount: investment.total_amount })
}

pub fn query_current_investors(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<InvestorsResponse> {
    let round = CURRENT.load(deps.storage)?.round;
    return query_investors(deps, round, start_after, limit);
}

pub fn query_investors(
    deps: Deps,
    round: u32,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<InvestorsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).max(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let investors = INVESTORS
        .prefix(round.to_string())
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            item.map(|(addr, amount)| Investor {
                addr: addr.to_string(),
                amount,
            })
        })
        .collect::<StdResult<_>>()?;

    Ok(InvestorsResponse { round, investors })
}

pub fn query_invest_result(deps: Deps, round: u32) -> StdResult<InvestResultResponse> {
    let investment = INVESTMENTS
        .may_load(deps.storage, round.to_string())?
        .unwrap();
    if investment.in_progress {
        return Err(StdError::generic_err("in progress"));
    }
    Ok(InvestResultResponse {
        round,
        first_winner: investment.first_winner.unwrap(),
        second_winner: investment.second_winner.unwrap(),
    })
}

pub fn query_token_total_supply(deps: Deps) -> StdResult<TotalSupplyResponse> {
    let supply = TOKEN_INFO.load(deps.storage)?.total_supply;
    Ok(TotalSupplyResponse { supply: supply })
}

pub fn query_token_balance(deps: Deps, who: String) -> StdResult<TokenBalanceResponse> {
    let who_addr = deps.api.addr_validate(&who)?;
    let balance = BALANCES.may_load(deps.storage, &who_addr)?.unwrap_or_default();
    Ok(TokenBalanceResponse { balance })
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CurrentRoundResponse {
    pub round: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CurrentInvestmentResponse {
    pub round: u32,
    pub total_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InvestorsResponse {
    pub round: u32,
    pub investors: Vec<Investor>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InvestResultResponse {
    pub round: u32,
    pub first_winner: Winner,
    pub second_winner: Winner,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TotalSupplyResponse {
    pub supply: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokenBalanceResponse {
    pub balance: Uint128,
}
