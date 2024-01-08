
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetPoolResponse};
use crate::state::{State, Bet, STATE};

const CONTRACT_NAME: &str = "crates.io:flipcoin";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cw_serde]
pub struct InstantiateMsg {
    // Add any fields needed for initialization, if necessary
}

#[cw_serde]
pub enum ExecuteMsg {
    PlaceBet { side: BetSide, amount: u128 },
    Resolve {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Query to get the current pool size for heads and tails
    GetPool {},
}

#[cw_serde]
pub struct GetPoolResponse {
    pub head_pool: u128,
    pub tail_pool: u128,
}

#[cw_serde]
pub enum BetSide {
    Heads,
    Tails,
}
