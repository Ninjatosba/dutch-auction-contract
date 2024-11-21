use cw_storage_plus::Item;
use cw_storage_plus::Map;

use crate::auction::Auction;

const AUCTIONS: Map<&u8, Auction> = Map::new("auctions");
const AUCTION_COUNT: Item<u8> = Item::new("auction_count");
