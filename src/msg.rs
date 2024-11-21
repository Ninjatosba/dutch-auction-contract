use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub auction_creation_fee: Coin,
    pub admin: String,
    pub min_seconds_until_auction_start: u64,
    pub max_aution_duration: u64,
    pub accepted_denoms: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateAuction {
        offered_asset: Coin,
        expected_denom: String,
        starting_price: Uint128,
        lowest_price: Uint128,
        start_time: Timestamp,
        end_time: Timestamp,
    },
    Bid {
        auction_id: u8,
        amount: Uint128,
    },
    ChangeParams {
        auction_creation_fee: Option<Coin>,
        min_seconds_until_auction_start: Option<u64>,
        max_aution_duration: Option<u64>,
        accepted_denoms: Option<Vec<String>>,
        admin: Option<String>,
    },
    CancelAuction {
        auction_id: u8,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
