/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.7.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Uint128, InstantiateMsg, Coin, ExecuteMsg, Decimal, Timestamp, Uint64, QueryMsg, Auction, ArrayOfTupleOfUint8AndAuction, Addr, Params } from "./DutchAuctionLaunchpad.types";
export interface DutchAuctionLaunchpadReadOnlyInterface {
  contractAddress: string;
  auctions: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: number;
  }) => Promise<ArrayOfTupleOfUint8AndAuction>;
  auction: ({
    auctionId
  }: {
    auctionId: number;
  }) => Promise<Auction>;
  params: () => Promise<Params>;
}
export class DutchAuctionLaunchpadQueryClient implements DutchAuctionLaunchpadReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.auctions = this.auctions.bind(this);
    this.auction = this.auction.bind(this);
    this.params = this.params.bind(this);
  }

  auctions = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: number;
  }): Promise<ArrayOfTupleOfUint8AndAuction> => {
    return this.client.queryContractSmart(this.contractAddress, {
      auctions: {
        limit,
        start_after: startAfter
      }
    });
  };
  auction = async ({
    auctionId
  }: {
    auctionId: number;
  }): Promise<Auction> => {
    return this.client.queryContractSmart(this.contractAddress, {
      auction: {
        auction_id: auctionId
      }
    });
  };
  params = async (): Promise<Params> => {
    return this.client.queryContractSmart(this.contractAddress, {
      params: {}
    });
  };
}
export interface DutchAuctionLaunchpadInterface extends DutchAuctionLaunchpadReadOnlyInterface {
  contractAddress: string;
  sender: string;
  createAuction: ({
    endPrice,
    endTime,
    inDenom,
    offeredAsset,
    startTime,
    startingPrice
  }: {
    endPrice: Decimal;
    endTime: Timestamp;
    inDenom: string;
    offeredAsset: Coin;
    startTime: Timestamp;
    startingPrice: Decimal;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  bid: ({
    auctionId
  }: {
    auctionId: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  changeParams: ({
    acceptedDenoms,
    admin,
    auctionCreationFee,
    maxAuctionDuration,
    minSecondsUntilAuctionStart
  }: {
    acceptedDenoms?: string[];
    admin?: string;
    auctionCreationFee?: Coin;
    maxAuctionDuration?: number;
    minSecondsUntilAuctionStart?: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  cancelAuction: ({
    auctionId
  }: {
    auctionId: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class DutchAuctionLaunchpadClient extends DutchAuctionLaunchpadQueryClient implements DutchAuctionLaunchpadInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createAuction = this.createAuction.bind(this);
    this.bid = this.bid.bind(this);
    this.changeParams = this.changeParams.bind(this);
    this.cancelAuction = this.cancelAuction.bind(this);
  }

  createAuction = async ({
    endPrice,
    endTime,
    inDenom,
    offeredAsset,
    startTime,
    startingPrice
  }: {
    endPrice: Decimal;
    endTime: Timestamp;
    inDenom: string;
    offeredAsset: Coin;
    startTime: Timestamp;
    startingPrice: Decimal;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_auction: {
        end_price: endPrice,
        end_time: endTime,
        in_denom: inDenom,
        offered_asset: offeredAsset,
        start_time: startTime,
        starting_price: startingPrice
      }
    }, fee, memo, _funds);
  };
  bid = async ({
    auctionId
  }: {
    auctionId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      bid: {
        auction_id: auctionId
      }
    }, fee, memo, _funds);
  };
  changeParams = async ({
    acceptedDenoms,
    admin,
    auctionCreationFee,
    maxAuctionDuration,
    minSecondsUntilAuctionStart
  }: {
    acceptedDenoms?: string[];
    admin?: string;
    auctionCreationFee?: Coin;
    maxAuctionDuration?: number;
    minSecondsUntilAuctionStart?: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      change_params: {
        accepted_denoms: acceptedDenoms,
        admin,
        auction_creation_fee: auctionCreationFee,
        max_auction_duration: maxAuctionDuration,
        min_seconds_until_auction_start: minSecondsUntilAuctionStart
      }
    }, fee, memo, _funds);
  };
  cancelAuction = async ({
    auctionId
  }: {
    auctionId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      cancel_auction: {
        auction_id: auctionId
      }
    }, fee, memo, _funds);
  };
}