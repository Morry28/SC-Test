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
const ADMIN_WALLET: &str = "osmo1efcn8ae5k5jlxxza7r7yr2m4elmux454kd7q3g";
const ADMIN_FEE_PERCENTAGE: u128 = 2; // 2% creator fee from loosing side

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

    // Check if the sent funds are in $OSMO and match the bet amount
    let sent_funds = info.funds.iter().find(|coin| coin.denom == "uosmo");
    match sent_funds {
        Some(coin) if coin.amount.u128() == amount => {
            // Correct denomination and amount, proceed with placing the bet
            let mut state = STATE.load(deps.storage)?;
            let bet = Bet { bettor: info.sender, amount: Uint128::new(amount), side };
            match side {
                BetSide::Heads => state.head_bets.push(bet),
                BetSide::Tails => state.tail_bets.push(bet),
            }
            STATE.save(deps.storage, &state)?;

            Ok(Response::new().add_attribute("action", "place_bet"))
        },
        _ => {
            // Either the denomination is not $OSMO or the amount doesn't match
            Err(ContractError::InvalidFunds {})
        }
    }
}


fn try_resolve_game(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let winner = if rand::random() { BetSide::Heads } else { BetSide::Tails };

    let total_heads: u128 = state.head_bets.iter().map(|bet| bet.amount.u128()).sum();
    let total_tails: u128 = state.tail_bets.iter().map(|bet| bet.amount.u128()).sum();

    let (winning_bets, losing_total) = match winner {
        BetSide::Heads => (&state.head_bets, total_tails),
        BetSide::Tails => (&state.tail_bets, total_heads),
    };

    // Calculate the admin fee
    let admin_fee = losing_total * ADMIN_FEE_PERCENTAGE / 100;
    let winnings_pool = losing_total - admin_fee;

    let mut messages: Vec<BankMsg> = Vec::new();

    // Send admin fee to the admin wallet
    messages.push(BankMsg::Send {
        to_address: ADMIN_WALLET.to_string(),
        amount: vec![Coin { denom: "uosmo".to_string(), amount: Uint128::from(admin_fee) }],
    });

    // Distribute the remaining winnings proportionally
    for bet in winning_bets {
        let winnings = bet.amount.u128() * winnings_pool / (total_heads + total_tails);
        messages.push(BankMsg::Send {
            to_address: bet.bettor.to_string(),
            amount: vec![Coin { denom: "uosmo".to_string(), amount: Uint128::from(winnings) }],
        });
    }

    // Clear the bets for the next round
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
