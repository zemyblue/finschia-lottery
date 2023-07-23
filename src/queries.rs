use cosmwasm_std::{Deps, Env, StdResult, Binary, to_binary, entry_point};
use crate::msg::{QueryMsg, InfoResponse};
use crate::state::{CONTRACT_INFO, TOKEN_INFO};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Info {} => to_binary(&contract_info(deps)?),
    }
}

fn contract_info(deps: Deps) -> StdResult<InfoResponse> {
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
