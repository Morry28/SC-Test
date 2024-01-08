use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // Include your custom errors here
    #[error("Invalid bet")]
    InvalidBet {},

    #[error("Invalid bet amount")]
    InvalidBetAmount {},

    // You can add other custom errors as needed
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
