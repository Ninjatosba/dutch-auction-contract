use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cosmwasm_std::Coin;
use cw_storage_plus::Item;
use cw_storage_plus::Map;

use crate::auction::Auction;

pub const AUCTIONS: Map<&u8, Auction> = Map::new("auctions");
pub const AUCTION_INDEX: Item<u8> = Item::new("auction_index");

#[cw_serde]
pub struct Params {
    pub auction_creation_fee: Coin,
    pub admin: Addr,
    pub min_seconds_until_auction_start: u64,
    pub max_aution_duration: u64,
    pub accepted_denoms: Vec<String>,
}
pub const PARAMS: Item<Params> = Item::new("params");
