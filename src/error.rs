use cosmwasm_std::OverflowError;
use cosmwasm_std::StdError;
use cosmwasm_std::Timestamp;
use cosmwasm_std::Uint128;
use cw_utils::PaymentError;
use thiserror::Error;

use crate::helpers::CustomPaymentError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    CustomPayment(#[from] CustomPaymentError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Unauthorized: the sender is not authorized to perform this action")]
    Unauthorized {},

    #[error("Start time ({start_time}) must be less than end time ({end_time})")]
    InvalidTimeRange {
        start_time: Timestamp,
        end_time: Timestamp,
    },

    #[error("End price ({end_price}) cannot be higher than starting price ({starting_price})")]
    EndPriceHigherThanStartingPrice {
        starting_price: Uint128,
        end_price: Uint128,
    },

    #[error("Auction start time ({start_time}) cannot be in the past (current time: {now})")]
    StartTimeInPast {
        start_time: Timestamp,
        now: Timestamp,
    },
    #[error("Auction must start at least {min_seconds} seconds from now (current time: {now}, start time: {start_time})")]
    StartTimeTooSoon {
        now: Timestamp,
        start_time: Timestamp,
        min_seconds: u64,
    },
    #[error("Auction duration ({duration} seconds) exceeds the maximum allowed duration ({max_duration} seconds)")]
    DurationTooLong { duration: u64, max_duration: u64 },

    // Denomination-related errors
    #[error("Offered asset and input denom cannot be the same (denom: {denom})")]
    SameDenomination { denom: String },

    #[error("Invalid params")]
    InvalidParams {},

    #[error("Auction remaining amount is insufficient")]
    InsufficientRemainingAmount {},

    #[error("Auction not found")]
    AuctionNotFound {},

    #[error("Auction is not active")]
    AuctionNotActive {},

    #[error("Auction cannot be canceled")]
    AuctionCannotBeCanceled {},
}
