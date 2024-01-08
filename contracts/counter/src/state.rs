use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum BetSide {
    Heads,
    Tails,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bet {
    pub bettor: Addr,
    pub amount: Uint128,
    pub side: BetSide,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub head_bets: Vec<Bet>,
    pub tail_bets: Vec<Bet>,
    pub admin: Addr,

}

pub const STATE: Item<State> = Item::new("state");
