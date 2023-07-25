#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Addr};
use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Order, Response, StdResult, SubMsg,
    Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::event::{ClosedInvestmentEvent, Event, InvestedEvent};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{
    ContractInfo, Current, Investment, Investor, TokenInfo, Winner, BALANCES, CONTRACT_INFO,
    CURRENT, INVESTMENTS, INVESTORS, TOKEN_INFO,
};
// use sha2::{Digest, Sha256};

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
    if msg.use_denom.is_empty() {
        return Err(ContractError::InvalidParams {});
    }
    let contract = ContractInfo {
        owner: info.sender.clone(),
        use_denom: msg.use_denom.clone(),
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Invest {} => handle_invest(deps, &info),
        ExecuteMsg::CloseInvestment {} => handle_close_investment(deps, env, info),
        ExecuteMsg::TransferToken { to, amount } => handl_transfer_token(deps, info, to, amount),
    }
}

fn mint_token(
    deps: DepsMut,
    _rsp: &Response,
    to: &Addr,
    amount: Uint128,
) -> Result<(), ContractError> {
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

    // TokenTransferredEvent {
    //     from: "",
    //     to: to.as_str(),
    //     amount,
    // }
    // .add_attributes(rsp);

    Ok(())
}

pub fn one_coin(info: &MessageInfo) -> Result<Coin, ContractError> {
    match info.funds.len() {
        0 => Err(ContractError::NoFunds {}),
        1 => {
            let coin = &info.funds[0];
            if coin.amount.is_zero() {
                Err(ContractError::NoFunds {})
            } else {
                Ok(coin.clone())
            }
        }
        _ => Err(ContractError::MultipleDenoms {}),
    }
}

pub fn must_pay(info: &MessageInfo, denom: &str) -> Result<Uint128, ContractError> {
    let coin = one_coin(info)?;
    if coin.denom != denom {
        Err(ContractError::MissingDenom(denom.to_string()))
    } else {
        Ok(coin.amount)
    }
}

pub fn handle_invest(deps: DepsMut, info: &MessageInfo) -> Result<Response, ContractError> {
    let contract = CONTRACT_INFO.load(deps.storage)?;
    let amount = must_pay(info, contract.use_denom.as_str())?;

    let round = CURRENT.load(deps.storage)?.round;

    // append investor
    let mut investment = INVESTMENTS
        .may_load(deps.storage, round.to_string())?
        .ok_or(ContractError::InvalidRound { round })?;
    investment.total_amount = investment.total_amount + amount;
    INVESTMENTS.save(deps.storage, round.to_string(), &investment)?;
    // INVESTORS.save(deps.storage, (round.to_string(), &info.sender), &amount)?;
    INVESTORS.update(deps.storage, (round.to_string(), &info.sender), |a| -> StdResult<_>{
        Ok(a.unwrap_or_default().checked_add(amount)?)
    })?;

    // calculate token to mint
    let exchange_ratio = CONTRACT_INFO.load(deps.storage)?.exchange_ratio;
    let exchange_amount = amount
        .checked_mul(Uint128::new(exchange_ratio))
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    let mut rsp = Response::default();
    // mint token to sender
    mint_token(deps, &mut rsp, &info.sender, exchange_amount)?;

    InvestedEvent {
        round,  
        who: &info.sender.as_ref(),
        amount,
    }
    .add_attributes(&mut rsp);

    Ok(rsp)
}

pub fn make_bank_send_msg(addr: String, amount: u128) -> SubMsg {
    return SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: addr,
        amount: vec![Coin::new(amount, "CONY")],
    }));
}

pub fn handle_close_investment(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // check if owner
    let contract = CONTRACT_INFO.load(deps.storage)?;
    if contract.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // close investment and add new investment
    let round = CURRENT.load(deps.storage)?.round;
    let mut investment = INVESTMENTS.load(deps.storage, round.to_string())?;

    // drawing winner
    let investors = INVESTORS
        .prefix(round.to_string())
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().map(|(addr, amount)| Investor {
                addr: addr.to_string(),
                amount,
            })
        })
        .collect::<Vec<_>>();

    let r_num = 7;
    let r_num2 = 8;
    let count = investors.len();
    // let uuid = new_uuid(&env, deps.storage, deps.api)?;
    // let hash = Sha256::digest(uuid.as_slice()).to_vec();

    let first_winner = &investors[r_num % count];
    let second_winner = &investors[r_num2 % count];

    // distribute invest amount
    let mut distribution = vec![];
    distribution.push(Winner {
        addr: first_winner.addr.clone(),
        amount: investment
            .total_amount
            .multiply_ratio(contract.first_winner_ratio as u128, 100u128),
    });
    distribution.push(Winner {
        addr: second_winner.addr.clone(),
        amount: investment
            .total_amount
            .multiply_ratio(contract.second_winner_ratio as u128, 100u128),
    });
    distribution.push(Winner {
        addr: contract.owner.to_string(),
        amount: investment
            .total_amount
            .multiply_ratio(contract.owner_ratio as u128, 100u128),
    });

    // update investment
    investment.in_progress = false;
    investment.first_winner = distribution.get(0).cloned();
    investment.second_winner = distribution.get(1).cloned();
    INVESTMENTS.save(deps.storage, round.to_string(), &investment)?;

    // update round
    CURRENT.update(deps.storage, |c| -> StdResult<_> {
        Ok(Current {
            round: round + 1,
            exchange_round: c.exchange_round,
        })
    })?;
    // create new investment & save
    let new_investment = Investment::new(round + 1);
    INVESTMENTS.save(deps.storage, (round + 1).to_string(), &new_investment)?;

    // distribute prize
    let mut submsgs: Vec<SubMsg> = vec![];
    for d in distribution {
        submsgs.push(make_bank_send_msg(d.addr, d.amount.u128()));
    }

    // staking

    let closed_investment_event = ClosedInvestmentEvent {
        round,
        first_winner: first_winner.addr.as_str(),
        second_winner: &second_winner.addr.as_str(),
        winner_hash: "",
    };

    let mut res = Response::new().add_submessages(submsgs);
    closed_investment_event.add_attributes(&mut res);

    Ok(res)
}

pub fn handl_transfer_token(
    deps: DepsMut,
    info: MessageInfo,
    to: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let to_addr = deps.api.addr_validate(&to)?;

    BALANCES.update(deps.storage, &info.sender, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(amount)?)
    })?;
    BALANCES.update(deps.storage, &to_addr, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_add(amount)?)
    })?;

    let rsp = Response::new();

    // TokenTransferredEvent {
    //     from: info.sender.as_ref(),
    //     to: to.as_ref(),
    //     amount,
    // }
    // .add_attributes(&mut rsp);

    Ok(rsp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{InfoResponse, QueryMsg};
    use crate::queries::*;
    use crate::state::Investor;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            use_denom: "cony".to_string(),
            exchange_ratio: 10,
            min_exchange_amount: 200000000u32,
            first_winner_ratio: 60u8,
            second_winner_ratio: 20u8,
            owner_ratio: 2u8,
            token_name: "lottery".to_string(),
            token_symbol: "LTT".to_string(),
            token_decimals: 6u8,
        };
        let info: MessageInfo = mock_info("creator", &coins(1000, "cony"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Info {}).unwrap();
        let value: InfoResponse = from_binary(&res).unwrap();
        assert_eq!(10, value.exchange_ratio);
    }

    fn do_instantiate(deps: DepsMut, info: MessageInfo) {
        let msg = InstantiateMsg {
            use_denom: "cony".to_string(),
            exchange_ratio: 10,
            min_exchange_amount: 200000000u32,
            first_winner_ratio: 60u8,
            second_winner_ratio: 20u8,
            owner_ratio: 2u8,
            token_name: "lottery".to_string(),
            token_symbol: "LTT".to_string(),
            token_decimals: 6u8,
        };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    fn do_invest(deps: DepsMut, addr: &str, amount: u128) {
        let auth_info = mock_info(addr, &coins(amount, "cony"));
        let msg = ExecuteMsg::Invest {};
        execute(deps, mock_env(), auth_info, msg).unwrap();
    }

    #[test]
    fn invest() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let auth_info = mock_info("creator", &coins(1000, "cony"));

        do_instantiate(deps.as_mut(), auth_info.clone());

        // invest
        do_invest(deps.as_mut(), "creator", 1000);

        let total_supply = query_token_total_supply(deps.as_ref()).unwrap();
        assert_eq!(Uint128::new(10000), total_supply.supply);
        assert_eq!(1u32, query_current_round(deps.as_ref()).unwrap().round);
        let investors = query_current_investors(deps.as_ref(), None, None).unwrap();
        assert_eq!(
            InvestorsResponse {
                round: 1u32,
                investors: vec![Investor {
                    addr: "creator".to_string(),
                    amount: Uint128::new(1000)
                }]
            },
            investors
        );
        let res = query_invest_result(deps.as_ref(), 1u32);
        match res.unwrap_err() {
            StdError::GenericErr { .. } => {}
            e => panic!("unexpected error {:?}", e),
        }
        let res = query_token_balance(deps.as_ref(), "creator".to_string()).unwrap();
        assert_eq!(Uint128::new(10000), res.balance);
    }

    #[test]
    fn close_investment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let auth_info = mock_info("creator", &coins(1000, "cony"));

        do_instantiate(deps.as_mut(), auth_info.clone());

        // invest
        do_invest(deps.as_mut(), "alpha", 1000);
        do_invest(deps.as_mut(), "beta", 1000);
        do_invest(deps.as_mut(), "chrlie", 1000);
        do_invest(deps.as_mut(), "delta", 1000);

        let total_supply = query_token_total_supply(deps.as_ref()).unwrap();
        assert_eq!(Uint128::new(40000), total_supply.supply);
        let investors = query_current_investors(deps.as_ref(), None, None).unwrap();
        assert_eq!(4usize, investors.investors.len());

        let msg = ExecuteMsg::CloseInvestment {};
        execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();

        assert_eq!(2u32, query_current_round(deps.as_ref()).unwrap().round);

        let res = query_invest_result(deps.as_ref(), 1).unwrap();
        assert_eq!(1u32, res.round);
        assert_eq!(Uint128::new(4000 * 60 / 100), res.first_winner.amount);
        assert_eq!(Uint128::new(4000 * 20 / 100), res.second_winner.amount);
        let res = query_investors(deps.as_ref(), 1, None, None).unwrap();
        assert_eq!(4, res.investors.len());
    }

    #[test]
    fn transfer_token() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let auth_info = mock_info("creator", &coins(1000, "cony"));

        do_instantiate(deps.as_mut(), auth_info.clone());

        let spender1 = String::from("alpha");
        let spender2 = String::from("beta");

        // invest
        do_invest(deps.as_mut(), spender1.as_str(), 1000);
        do_invest(deps.as_mut(), spender2.as_str(), 1000);

        assert_eq!(Uint128::new(20000), query_token_total_supply(deps.as_ref()).unwrap().supply);
        assert_eq!(Uint128::new(10000), query_token_balance(deps.as_ref(), spender1.clone()).unwrap().balance);
        assert_eq!(Uint128::new(10000), query_token_balance(deps.as_ref(), spender2.clone()).unwrap().balance);

        // transfer token
        let info = mock_info(spender1.as_str(), &[]);
        let msg = ExecuteMsg::TransferToken { to: spender2.clone(), amount:Uint128::new(5000) };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(Uint128::new(20000), query_token_total_supply(deps.as_ref()).unwrap().supply);
        assert_eq!(Uint128::new(5000), query_token_balance(deps.as_ref(), spender1.clone()).unwrap().balance);
        assert_eq!(Uint128::new(15000), query_token_balance(deps.as_ref(), spender2.clone()).unwrap().balance);
    }
}
