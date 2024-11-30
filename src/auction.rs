use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Timestamp, Uint128};

use crate::{state::Params, ContractError};

#[cw_serde]
pub struct Auction {
    pub creator: String,
    pub offered_asset: Coin,
    pub in_denom: String,
    pub starting_price: Uint128,
    pub end_price: Uint128,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub remaining_amount: Uint128,
}

impl Auction {
    pub fn new(
        creator: String,
        offered_asset: Coin,
        in_denom: String,
        starting_price: Uint128,
        end_price: Uint128,
        start_time: Timestamp,
        end_time: Timestamp,
    ) -> Self {
        Auction {
            creator,
            offered_asset: offered_asset.clone(),
            in_denom,
            starting_price,
            end_price,
            start_time,
            end_time,
            remaining_amount: offered_asset.amount,
        }
    }
    pub fn validate(&self, now: Timestamp, params: Params) -> Result<(), ContractError> {
        // Price validation
        if self.starting_price < self.end_price {
            return Err(ContractError::EndPriceHigherThanStartingPrice {
                starting_price: self.starting_price,
                end_price: self.end_price,
            });
        }

        // Time range validation
        if self.start_time > self.end_time {
            return Err(ContractError::InvalidTimeRange {
                start_time: self.start_time,
                end_time: self.end_time,
            });
        }

        // Denomination validation
        if self.offered_asset.denom == self.in_denom {
            return Err(ContractError::SameDenomination {
                denom: self.offered_asset.denom.clone(),
            });
        }

        // Start time in the past
        if self.start_time < now {
            return Err(ContractError::StartTimeInPast {
                start_time: self.start_time,
                now,
            });
        }

        // Start time too soon
        if now.plus_seconds(params.min_seconds_until_auction_start) > self.start_time {
            return Err(ContractError::StartTimeTooSoon {
                now,
                start_time: self.start_time,
                min_seconds: params.min_seconds_until_auction_start,
            });
        }

        // Duration validation
        let duration = self
            .end_time
            .minus_seconds(self.start_time.seconds())
            .seconds();
        if duration > params.max_auction_duration {
            return Err(ContractError::DurationTooLong {
                duration,
                max_duration: params.max_auction_duration,
            });
        }

        Ok(())
    }

    pub fn calculate_price(&self, now: Timestamp) -> Uint128 {
        let total_time = self.end_time.minus_nanos(self.start_time.nanos());
        let remaining_time = self.end_time.minus_nanos(now.nanos());
        let price_range = self.starting_price - self.end_price;
        let decimal_time_passed = Decimal::from_ratio(remaining_time.nanos(), total_time.nanos());
        let price = self.starting_price
            - (Decimal::from_ratio(price_range.u128(), 1u128) * decimal_time_passed)
                .to_uint_floor();
        price
    }

    pub fn is_active(&self, now: Timestamp) -> bool {
        self.start_time <= now && now <= self.end_time
    }

    pub fn is_started(&self, now: Timestamp) -> bool {
        self.start_time <= now
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::{Addr, Timestamp};

    #[test]
    fn test_validate_same_denomination() {
        let auction = Auction {
            creator: "creator".to_string(),
            offered_asset: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(1000u128),
            },
            in_denom: "uusd".to_string(),
            starting_price: Uint128::from(1000u128),
            end_price: Uint128::from(500u128),
            start_time: Timestamp::from_seconds(1000),
            end_time: Timestamp::from_seconds(2000),
            remaining_amount: Uint128::from(1000u128),
        };

        let params = Params {
            auction_creation_fee: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            admin: Addr::unchecked("admin"),
            min_seconds_until_auction_start: 1000,
            max_auction_duration: 1000,
            accepted_denoms: vec!["uusd".to_string()],
        };

        let now = Timestamp::from_seconds(500);
        let result = auction.validate(now, params);
        assert!(matches!(
            result,
            Err(ContractError::SameDenomination { denom }) if denom == "uusd"
        ));
    }

    #[test]
    fn test_validate_start_time_too_soon() {
        let auction = Auction {
            creator: "creator".to_string(),
            offered_asset: Coin {
                denom: "ubtc".to_string(),
                amount: Uint128::from(1000u128),
            },
            in_denom: "uusd".to_string(),
            starting_price: Uint128::from(1000u128),
            end_price: Uint128::from(500u128),
            start_time: Timestamp::from_seconds(1500),
            end_time: Timestamp::from_seconds(3000),
            remaining_amount: Uint128::from(1000u128),
        };

        let params = Params {
            auction_creation_fee: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
            admin: Addr::unchecked("admin"),
            min_seconds_until_auction_start: 1000,
            max_auction_duration: 2000,
            accepted_denoms: vec!["uusd".to_string()],
        };

        let now = Timestamp::from_seconds(1400);
        let result = auction.validate(now, params);
        assert!(matches!(
            result,
            Err(ContractError::StartTimeTooSoon {
                now: t_now,
                start_time,
                min_seconds
            }) if t_now == Timestamp::from_seconds(1400)
                && start_time == Timestamp::from_seconds(1500)
                && min_seconds == 1000
        ));
    }
}
