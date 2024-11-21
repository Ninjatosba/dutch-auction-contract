use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Timestamp, Uint128};

use crate::ContractError;

#[cw_serde]
pub struct Auction {
    pub creator: String,
    pub offered_asset: Coin,
    pub expected_denom: String,
    pub starting_price: Uint128,
    pub lowest_price: Uint128,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
}

impl Auction {
    pub fn new(
        creator: String,
        offered_asset: Coin,
        expected_denom: String,
        starting_price: Uint128,
        lowest_price: Uint128,
        start_time: Timestamp,
        end_time: Timestamp,
    ) -> Self {
        Auction {
            creator,
            offered_asset,
            expected_denom,
            starting_price,
            lowest_price,
            start_time,
            end_time,
        }
    }
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.starting_price > self.lowest_price {
            return Err(ContractError::InvalidPrice {});
        }
        if self.start_time > self.end_time {
            return Err(ContractError::InvalidTime {});
        }

        if self.offered_asset.denom == self.expected_denom {
            return Err(ContractError::InvalidDenom {});
        }
        Ok(())
    }

    pub fn calculate_price(&self, current_time: Timestamp) -> Uint128 {
        let total_time = self.end_time.minus_nanos(self.start_time.nanos());
        let remaining_time = self.end_time.minus_nanos(current_time.nanos());
        let price_range = self.starting_price - self.lowest_price;
        let decimal_time_passed = Decimal::from_ratio(remaining_time.nanos(), total_time.nanos());
        let price = self.starting_price
            - (Decimal::from_ratio(price_range.u128(), 1u128) * decimal_time_passed)
                .to_uint_floor();
        price
    }
}
