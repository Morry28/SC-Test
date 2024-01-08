#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin};
use cosmwasm_std::{Uint128, BankMsg, Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetPoolResponse};
use crate::state::{State, Bet, STATE};
use crate::state::{Bet, BetSide};

const CONTRACT_NAME: &str = "crates.io:flipcoin";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        head_bets: vec![],
        tail_bets: vec![],
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PlaceBet { bet, amount } => try_place_bet(deps, info, bet, amount),
        ExecuteMsg::Resolve => try_resolve_game(deps, env),
    }
}

fn try_place_bet(deps: DepsMut, info: MessageInfo, bet: String, amount: u128) -> Result<Response, ContractError> {
    // Validate bet and amount
    let side = match bet.as_str() {
        "head" => BetSide::Heads,
        "tail" => BetSide::Tails,
        _ => return Err(ContractError::InvalidBet {}),
    };
    if amount <= 0 {
        return Err(ContractError::InvalidBetAmount {});
    }

    // Load and update state
    let mut state = STATE.load(deps.storage)?;
    let bet = Bet { bettor: info.sender, amount: Uint128::new(amount), side };
    match side {
        BetSide::Heads => state.head_bets.push(bet),
        BetSide::Tails => state.tail_bets.push(bet),
    }
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "place_bet"))
}

fn try_resolve_game(deps: DepsMut, _env: Env) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let winner = if rand::random() { BetSide::Heads } else { BetSide::Tails };

    // Calculate winnings and prepare messages
    let (winning_bets, losing_total) = match winner {
        BetSide::Heads => (state.head_bets, state.tail_bets.iter().map(|bet| bet.amount.u128()).sum()),
        BetSide::Tails => (state.tail_bets, state.head_bets.iter().map(|bet| bet.amount.u128()).sum()),
    };

    let mut messages = vec![];
    for bet in winning_bets {
        let winnings = bet.amount.u128() * 2; // Double the bet amount
        messages.push(BankMsg::Send {
            to_address: bet.bettor.to_string(),
            amount: vec![Coin { denom: "uosmo".to_string(), amount: Uint128::from(winnings) }],
        });
    }

    // Reset state for the next game
    STATE.save(deps.storage, &State::default())?;

    Ok(Response::new().add_messages(messages).add_attribute("action", "resolve_game"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPool {} => to_binary(&query_pool(deps)?),
    }
}

fn query_pool(deps: Deps) -> StdResult<GetPoolResponse> {
    let state = STATE.load(deps.storage)?;
    let head_pool: u128 = state.head_bets.iter().map(|bet| bet.amount).sum();
    let tail_pool: u128 = state.tail_bets.iter().map(|bet| bet.amount).sum();
    Ok(GetPoolResponse { head_pool, tail_pool })
}

// Implement additional helper functions as needed
