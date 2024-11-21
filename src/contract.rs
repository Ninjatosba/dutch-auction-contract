#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp, Uint128,
};
use cw_utils::must_pay;

use crate::auction::{self, Auction};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Params, AUCTIONS, AUCTION_INDEX, PARAMS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dutch-auction-launchpad";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the contract version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.min_seconds_until_auction_start == 0 {
        return Err(ContractError::InvalidParams {});
    }
    if msg.max_aution_duration == 0 {
        return Err(ContractError::InvalidParams {});
    }

    let admin = deps.api.addr_validate(&msg.admin)?;

    // Set the contract parameters
    let params = Params {
        auction_creation_fee: msg.auction_creation_fee,
        admin,
        min_seconds_until_auction_start: msg.min_seconds_until_auction_start,
        max_aution_duration: msg.max_aution_duration,
        accepted_denoms: msg.accepted_denoms,
    };

    PARAMS.save(deps.storage, &params)?;
    AUCTION_INDEX.save(deps.storage, &0)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateAuction {
            offered_asset,
            expected_denom,
            starting_price,
            lowest_price,
            start_time,
            end_time,
        } => execute_create_auction(
            deps,
            env,
            info,
            offered_asset,
            expected_denom,
            starting_price,
            lowest_price,
            start_time,
            end_time,
        ),
        ExecuteMsg::Bid { auction_id, amount } => {
            // Bid on auction
            unimplemented!()
        }
        ExecuteMsg::ChangeParams {
            auction_creation_fee,
            min_seconds_until_auction_start,
            max_aution_duration,
            accepted_denoms,
            admin,
        } => {
            // Change contract parameters
            unimplemented!()
        }
        ExecuteMsg::CancelAuction { auction_id } => {
            // Cancel auction
            unimplemented!()
        }
    }
}

fn execute_create_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offered_asset: Coin,
    expected_denom: String,
    starting_price: Uint128,
    lowest_price: Uint128,
    start_time: Timestamp,
    end_time: Timestamp,
) -> Result<Response, ContractError> {
    let params = PARAMS.load(deps.storage)?;
    let funds = info.funds.clone();
    let expected_funds = vec![params.auction_creation_fee.clone(), offered_asset.clone()];
    // TODO - normalize and compare funds
    if funds != expected_funds {
        return Err(ContractError::InvalidParams {});
    }
    let auction = Auction::new(
        info.sender.to_string(),
        offered_asset.clone(),
        expected_denom,
        starting_price,
        lowest_price,
        start_time,
        end_time,
    );
    auction.validate(env.block.time, params.clone())?;
    let updated_auction_index = AUCTION_INDEX.update(deps.storage, |index| -> StdResult<u8> {
        let new_index = index + 1;
        Ok(new_index)
    })?;
    AUCTIONS.save(deps.storage, &updated_auction_index, &auction)?;

    let creation_fee_msg = BankMsg::Send {
        to_address: params.admin.to_string(),
        amount: vec![params.auction_creation_fee],
    };

    let res: Response = Response::default()
        .add_attribute("action", "create_auction")
        .add_attribute("auction_id", updated_auction_index.to_string())
        .add_attribute("creator", info.sender)
        .add_attribute("offered_asset_denom", offered_asset.denom.to_string())
        .add_attribute("offered_asset_amount", offered_asset.amount.to_string())
        .add_message(creation_fee_msg);
    Ok(res)
}

fn execute_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    auction_id: u8,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut auction = AUCTIONS.load(deps.storage, &auction_id)?;

    let remaining_amount = auction.remaining_amount;
    if amount > remaining_amount {
        return Err(ContractError::InsufficientAmount {});
    }
    auction.remaining_amount = remaining_amount.checked_sub(amount)?;
    AUCTIONS.save(deps.storage, &auction_id, &auction)?;

    let price = auction.calculate_price(env.block.time);
    let expected_amount = price * amount;
    let expected_denom = auction.offered_asset.denom.clone();
    let sent_amount = must_pay(&info, &expected_denom)?;

    if sent_amount <= expected_amount {
        return Err(ContractError::InvalidBid {});
    }
    let refund_amount = sent_amount.checked_sub(expected_amount)?;

    let refund_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: expected_denom,
            amount: refund_amount,
        }],
    };
    let asset_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![auction.offered_asset],
    };

    let res: Response = Response::default()
        .add_attribute("action", "bid")
        .add_attribute("auction_id", auction_id.to_string())
        .add_attribute("bidder", info.sender)
        .add_attribute("amount", amount.to_string())
        .add_message(refund_msg)
        .add_message(asset_msg);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
