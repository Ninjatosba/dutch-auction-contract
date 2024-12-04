use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Timestamp};

use crate::{auction::Auction, state::Params};

#[cw_serde]
pub struct InstantiateMsg {
    pub auction_creation_fee: Coin,
    pub admin: String,
    pub min_seconds_until_auction_start: u64,
    pub max_auction_duration: u64,
    pub accepted_denoms: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateAuction {
        offered_asset: Coin,
        in_denom: String,
        starting_price: Decimal,
        end_price: Decimal,
        start_time: Timestamp,
        end_time: Timestamp,
    },
    Bid {
        auction_id: u8,
    },
    ChangeParams {
        auction_creation_fee: Option<Coin>,
        min_seconds_until_auction_start: Option<u64>,
        max_auction_duration: Option<u64>,
        accepted_denoms: Option<Vec<String>>,
        admin: Option<String>,
    },
    CancelAuction {
        auction_id: u8,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<(u8,Auction)>)]
    Auctions {
        start_after: Option<u8>,
        limit: Option<u8>,
    },
    #[returns(Auction)]
    Auction { auction_id: u8 },

    #[returns(Params)]
    Params {},
}
