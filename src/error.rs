use cosmwasm_std::OverflowError;
use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Start time must be less than end time")]
    InvalidTime {},

    #[error("Starting price must be less than or equal to lowest price")]
    InvalidPrice {},

    #[error("Offered asset cant be the same as expected denom")]
    InvalidDenom {},

    #[error("Invalid params")]
    InvalidParams {},

    #[error("Invalid bid")]
    InvalidBid {},

    #[error("Remaining amount is less than bid amount")]
    InsufficientAmount {},
}
