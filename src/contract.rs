#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Timestamp, Uint128,
};
use cw_storage_plus::Bound;
use cw_utils::must_pay;

use crate::auction::Auction;
use crate::error::ContractError;
use crate::helpers::check_payment;
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
            in_denom,
            starting_price,
            end_price,
            start_time,
            end_time,
        } => execute_create_auction(
            deps,
            env,
            info,
            offered_asset,
            in_denom,
            starting_price,
            end_price,
            start_time,
            end_time,
        ),
        ExecuteMsg::Bid { auction_id } => execute_bid(deps, env, info, auction_id),
        ExecuteMsg::ChangeParams {
            auction_creation_fee,
            min_seconds_until_auction_start,
            max_aution_duration,
            accepted_denoms,
            admin,
        } => execute_change_params(
            deps,
            env,
            info,
            auction_creation_fee,
            min_seconds_until_auction_start,
            max_aution_duration,
            accepted_denoms,
            admin,
        ),
        ExecuteMsg::CancelAuction { auction_id } => {
            execute_cancel_auction(deps, env, info, auction_id)
        }
    }
}

fn execute_create_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offered_asset: Coin,
    in_denom: String,
    starting_price: Uint128,
    end_price: Uint128,
    start_time: Timestamp,
    end_time: Timestamp,
) -> Result<Response, ContractError> {
    let params = PARAMS.load(deps.storage)?;
    let funds = info.funds.clone();

    let expected_funds = vec![params.auction_creation_fee.clone(), offered_asset.clone()];
    // Check if the sent funds are correct
    check_payment(&funds, &expected_funds)?;

    let auction = Auction::new(
        info.sender.to_string(),
        offered_asset.clone(),
        in_denom,
        starting_price,
        end_price,
        start_time,
        end_time,
    );

    auction.validate(env.block.time, params.clone())?;

    let updated_auction_index = AUCTION_INDEX.update(deps.storage, |index| -> StdResult<u8> {
        let new_index = index + 1;
        Ok(new_index)
    })?;

    AUCTIONS.save(deps.storage, updated_auction_index, &auction)?;

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
) -> Result<Response, ContractError> {
    let mut auction = AUCTIONS
        .load(deps.storage, auction_id)
        .map_err(|_| ContractError::AuctionNotFound {})?;

    if auction.is_active(env.block.time) {
        return Err(ContractError::AuctionNotActive {});
    }
    let price = auction.calculate_price(env.block.time);
    let amount = must_pay(&info, &auction.in_denom)?;

    let acquired_amount = amount.checked_mul(price)?;
    let acquired_asset = Coin {
        denom: auction.offered_asset.denom.clone(),
        amount: acquired_amount,
    };
    auction.remaining_amount = auction.remaining_amount.checked_sub(acquired_amount)?;
    AUCTIONS.save(deps.storage, auction_id, &auction)?;

    let res: Response = Response::default()
        .add_attribute("action", "bid")
        .add_attribute("auction_id", auction_id.to_string())
        .add_attribute("bidder", info.sender)
        .add_attribute("amount", amount.to_string())
        .add_attribute("acquired_asset_denom", acquired_asset.denom)
        .add_attribute("acquired_asset_amount", acquired_asset.amount.to_string());
    Ok(res)
}

fn execute_change_params(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    auction_creation_fee: Option<Coin>,
    min_seconds_until_auction_start: Option<u64>,
    max_aution_duration: Option<u64>,
    accepted_denoms: Option<Vec<String>>,
    admin: Option<String>,
) -> Result<Response, ContractError> {
    let mut params = PARAMS.load(deps.storage)?;
    if info.sender != params.admin {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(auction_creation_fee) = auction_creation_fee {
        params.auction_creation_fee = auction_creation_fee;
    }
    if let Some(min_seconds_until_auction_start) = min_seconds_until_auction_start {
        params.min_seconds_until_auction_start = min_seconds_until_auction_start;
    }
    if let Some(max_aution_duration) = max_aution_duration {
        params.max_aution_duration = max_aution_duration;
    }
    if let Some(accepted_denoms) = accepted_denoms {
        params.accepted_denoms = accepted_denoms;
    }
    if let Some(admin) = admin {
        let admin = deps.api.addr_validate(&admin)?;
        params.admin = admin;
    }

    PARAMS.save(deps.storage, &params)?;

    let res: Response = Response::default().add_attribute("action", "change_params");
    Ok(res)
}

fn execute_cancel_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    auction_id: u8,
) -> Result<Response, ContractError> {
    let auction = AUCTIONS
        .load(deps.storage, auction_id)
        .map_err(|_| ContractError::AuctionNotFound {})?;

    if info.sender.to_string() != auction.creator {
        return Err(ContractError::Unauthorized {});
    }

    if auction.is_started(env.block.time) {
        return Err(ContractError::AuctionCannotBeCanceled {});
    }

    AUCTIONS.remove(deps.storage, auction_id);

    let res: Response = Response::default()
        .add_attribute("action", "cancel_auction")
        .add_attribute("auction_id", auction_id.to_string());
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::Auctions { start_after, limit } => {
            to_json_binary(&query_auctions(_deps, start_after, limit)?)
        }

        QueryMsg::Auction { auction_id } => to_json_binary(&query_auction(_deps, auction_id)?),
        QueryMsg::Params {} => to_json_binary(&query_params(_deps)?),
    }
}

fn query_params(deps: Deps) -> Result<Params, ContractError> {
    PARAMS
        .load(deps.storage)
        .map_err(|_| ContractError::InvalidParams {})
}

fn query_auction(deps: Deps, auction_id: u8) -> Result<Auction, ContractError> {
    AUCTIONS
        .load(deps.storage, auction_id)
        .map_err(|_| ContractError::AuctionNotFound {})
}

const MAX_LIMIT: u8 = 30;

fn query_auctions(
    deps: Deps,
    start_after: Option<u8>,
    limit: Option<u8>,
) -> StdResult<Vec<(u8, Auction)>> {
    let start = start_after.map(Bound::exclusive);
    let limit = limit.unwrap_or(MAX_LIMIT).min(MAX_LIMIT) as usize;

    let auctions = AUCTIONS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (index, auction) = item?;
            Ok((index, auction))
        })
        .collect::<StdResult<Vec<_>>>()?;
    Ok(auctions)
}

#[cfg(test)]
mod tests {}
