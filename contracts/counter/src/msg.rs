use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
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
