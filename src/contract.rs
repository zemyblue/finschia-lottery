#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Addr};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::event::{Event, InvestedEvent, TokenTransferredEvent};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{
    ContractInfo, Current, Investment, TokenInfo, BALANCES, CONTRACT_INFO, CURRENT, INVESTMENTS, INVESTORS,
    TOKEN_INFO,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-lottery";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let contract = ContractInfo {
        owner: info.sender.clone(),
        exchange_ratio: msg.exchange_ratio,
        min_exchange_amount: msg.min_exchange_amount,
        first_winner_ratio: msg.first_winner_ratio,
        second_winner_ratio: msg.second_winner_ratio,
        owner_ratio: msg.owner_ratio,
    };
    let token = TokenInfo {
        name: msg.token_name.clone(),
        symbol: msg.token_symbol.clone(),
        decimals: msg.token_decimals,
        total_supply: Uint128::zero(),
    };
    let current = Current {
        round: 1u32,
        exchange_round: 1u32,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONTRACT_INFO.save(deps.storage, &contract)?;
    TOKEN_INFO.save(deps.storage, &token)?;
    CURRENT.save(deps.storage, &current)?;

    let new_investment = Investment {
        round: current.round,
        total_amount: Uint128::zero(),
        in_progress: true,
        first_winner: None,
        second_winner: None,
    };
    INVESTMENTS.save(deps.storage, current.round.to_string(), &new_investment)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Invest { amount } => handle_invest(deps, info, amount),
        ExecuteMsg::CloseInvestment {} => handle_close_investment(deps),
    }
}

fn mint_token(
    deps: DepsMut,
    to: &Addr,
    amount: Uint128,
) -> Result<TokenTransferredEvent, ContractError> {
    BALANCES.update(
        deps.storage,
        to,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_add(amount)?)
        },
    )?;
    TOKEN_INFO.update(deps.storage, |mut info| -> StdResult<_> {
        info.total_supply = info.total_supply.checked_add(amount)?;
        Ok(info)
    })?;

    Ok(TokenTransferredEvent {
        from: "".to_string(),
        to: to.to_string(),
        amount,
    })
}

pub fn handle_invest(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // 현재 round에 투자자를 추가하고, 전체 투자 금액을 증가시킨다.
    let round = CURRENT.load(deps.storage)?.round;

    // append investor
    let mut investment = INVESTMENTS
        .may_load(deps.storage, round.to_string())?
        .ok_or(ContractError::InvalidRound { round })?;
    investment.total_amount = investment.total_amount + amount;
    INVESTMENTS.save(deps.storage, round.to_string(), &investment)?;
    INVESTORS.save(deps.storage, (round.to_string(), &info.sender), &amount)?;

    // transfer token
    let exchange_ratio = CONTRACT_INFO.load(deps.storage)?.exchange_ratio;
    let exchange_amount = amount
        .checked_mul(Uint128::new(exchange_ratio))
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let event = mint_token(deps, &info.sender, exchange_amount)?;
    let invested_event = InvestedEvent {
        round,
        who: &info.sender.as_ref(),
        amount,
    };

    let mut rsp = Response::default();
    event.add_attributes(&mut rsp);
    invested_event.add_attributes(&mut rsp);

    Ok(rsp)
}

pub fn handle_close_investment(_deps: DepsMut) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{InfoResponse, QueryMsg};
    // use crate::queries::{query, token_total_supply};
    use crate::queries::*;
    use crate::state::Investor;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            exchange_ratio: 10,
            min_exchange_amount: 200000000u32,
            first_winner_ratio: 60u8,
            second_winner_ratio: 20u8,
            owner_ratio: 2u8,
            token_name: "lottery".to_string(),
            token_symbol: "LTT".to_string(),
            token_decimals: 6u8,
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Info {}).unwrap();
        let value: InfoResponse = from_binary(&res).unwrap();
        assert_eq!(10, value.exchange_ratio);
    }

    #[test]
    fn invest() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            exchange_ratio: 10,
            min_exchange_amount: 200000000u32,
            first_winner_ratio: 60u8,
            second_winner_ratio: 20u8,
            owner_ratio: 2u8,
            token_name: "lottery".to_string(),
            token_symbol: "LTT".to_string(),
            token_decimals: 6u8,
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // invest
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Invest {
            amount: Uint128::new(1000),
        };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let total_supply = query_token_total_supply(deps.as_ref()).unwrap();
        assert_eq!(Uint128::new(10000), total_supply.supply);
        assert_eq!(1u32, query_current_round(deps.as_ref()).unwrap().round);
        let investors = query_current_investors(deps.as_ref(), None, None).unwrap();
        assert_eq!(1u32, investors.round);
        assert_eq!(vec![Investor{addr: "creator".to_string(), amount: Uint128::new(1000)}], investors.investors);
        let res = query_invest_result(deps.as_ref(), 1u32);
        match res.unwrap_err() {
            StdError::GenericErr { .. } => {},
            e => panic!("unexpected error {:?}", e),
        }
        let who = Addr::unchecked("creator".to_string());
        let res = query_token_balance(deps.as_ref(), who).unwrap();
        assert_eq!(Uint128::new(10000), res.balance);
    }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
