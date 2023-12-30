
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    PlaceBet { bet: String, amount: u128 },  // Bet can be "head" or "tail"
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





/*

use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },

}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}


*/