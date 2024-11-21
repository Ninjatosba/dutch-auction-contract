use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Timestamp, Uint128};

use crate::{state::Params, ContractError};

#[cw_serde]
pub struct Auction {
    pub creator: String,
    pub offered_asset: Coin,
    pub expected_denom: String,
    pub starting_price: Uint128,
    pub lowest_price: Uint128,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub remaining_amount: Uint128,
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
            offered_asset: offered_asset.clone(),
            expected_denom,
            starting_price,
            lowest_price,
            start_time,
            end_time,
            remaining_amount: offered_asset.amount,
        }
    }
    pub fn validate(&self, now: Timestamp, params: Params) -> Result<(), ContractError> {
        if self.starting_price < self.lowest_price {
            return Err(ContractError::InvalidPrice {});
        }
        if self.start_time > self.end_time {
            return Err(ContractError::InvalidTime {});
        }

        if self.offered_asset.denom == self.expected_denom {
            return Err(ContractError::InvalidDenom {});
        }
        if self.start_time < now {
            return Err(ContractError::InvalidTime {});
        }
        if now.plus_seconds(params.min_seconds_until_auction_start) > self.start_time {
            return Err(ContractError::InvalidTime {});
        }
        if (self.end_time.minus_seconds(self.start_time.seconds())).seconds()
            > params.max_aution_duration
        {
            return Err(ContractError::InvalidTime {});
        }

        Ok(())
    }

    pub fn calculate_price(&self, now: Timestamp) -> Uint128 {
        let total_time = self.end_time.minus_nanos(self.start_time.nanos());
        let remaining_time = self.end_time.minus_nanos(now.nanos());
        let price_range = self.starting_price - self.lowest_price;
        let decimal_time_passed = Decimal::from_ratio(remaining_time.nanos(), total_time.nanos());
        let price = self.starting_price
            - (Decimal::from_ratio(price_range.u128(), 1u128) * decimal_time_passed)
                .to_uint_floor();
        price
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use super::*;
    use cosmwasm_std::{Addr, Timestamp};

    #[test]
    fn test_calculate_price() {
        let start_time = Timestamp::from_seconds(1000);
        let end_time = Timestamp::from_seconds(2000);
        let auction = Auction {
            creator: "creator".to_string(),
            offered_asset: Coin {
                denom: "ubtc".to_string(),
                amount: Uint128::from(1000u128),
            },
            expected_denom: "uusd".to_string(),
            starting_price: Uint128::from(1000u128),
            lowest_price: Uint128::from(100u128),
            start_time,
            end_time,
            remaining_amount: Uint128::from(1000u128),
        };

        let current_time = Timestamp::from_seconds(1500);
        let price = auction.calculate_price(current_time);
        assert_eq!(price, Uint128::from(550u128));
    }

    #[test]
    fn test_validate() {
        let start_time = Timestamp::from_seconds(1000);
        let end_time = Timestamp::from_seconds(2000);
        let auction = Auction {
            creator: "creator".to_string(),
            offered_asset: Coin {
                denom: "ubtc".to_string(),
                amount: Uint128::from(1000u128),
            },
            expected_denom: "uusd".to_string(),
            starting_price: Uint128::from(1000u128),
            lowest_price: Uint128::from(100u128),
            start_time,
            end_time,
            remaining_amount: Uint128::from(1000u128),
        };
        let now = Timestamp::from_seconds(500);

        let params = Params {
            auction_creation_fee: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            admin: Addr::unchecked("admin"),
            min_seconds_until_auction_start: 1000,
            max_aution_duration: 1000,
            accepted_denoms: vec!["uusd".to_string()],
        };

        let result = auction.validate(now, params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_price() {
        let start_time = Timestamp::from_seconds(1000);
        let end_time = Timestamp::from_seconds(2000);
        let auction = Auction {
            creator: "creator".to_string(),
            offered_asset: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(1000u128),
            },
            expected_denom: "uusd".to_string(),
            starting_price: Uint128::from(100u128),
            lowest_price: Uint128::from(1000u128),
            start_time,
            end_time,
            remaining_amount: Uint128::from(1000u128),
        };

        let params = Params {
            auction_creation_fee: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            admin: Addr::unchecked("admin"),
            min_seconds_until_auction_start: 1000,
            max_aution_duration: 1000,
            accepted_denoms: vec!["uusd".to_string()],
        };
        let now = Timestamp::from_seconds(500);
        let result = auction.validate(now, params);
        assert!(result.is_err());
    }
}
