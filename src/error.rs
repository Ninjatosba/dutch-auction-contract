use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Start time must be less than end time")]
    InvalidTime {},

    #[error("Starting price must be less than or equal to lowest price")]
    InvalidPrice {},

    #[error("Offered asset cant be the same as expected denom")]
    InvalidDenom {},
}
