## Dutch Auction Launchpad (CosmWasm)

Smart contract and tooling for creating and running Dutch auctions on Cosmos chains.

### Context
- Focused on Real-World Assets (RWAs) auction and token sale use cases.
- Built during a Mantra blockchain CosmWasm workshop (Dukong testnet).
- A mock frontend was used for demos; this repo contains the contract and scripts.
- You can find the frontend here [here](https://dutch-auction-webapp.vercel.app/) Implementation is not complete. So frontend is for demo purposes only.

### What it does
- Sellers create auctions for a specific `offered_asset` with a price that linearly decreases from `starting_price` to `end_price` between `start_time` and `end_time`.
- Bidders pay in `in_denom` and receive the corresponding amount of the offered asset at the current price.
- Admin can update parameters like fees, accepted denoms, and timing constraints.

### Contract interface (abridged)
- **InstantiateMsg**: `auction_creation_fee: Coin`, `admin: String`, `min_seconds_until_auction_start: u64`, `max_auction_duration: u64`, `accepted_denoms: string[]`.
- **ExecuteMsg**:
  - `CreateAuction { offered_asset, in_denom, starting_price, end_price, start_time, end_time }`
  - `Bid { auction_id }` (send funds in `in_denom`)
  - `ChangeParams { ... }` (admin only)
  - `CancelAuction { auction_id }` (before start, creator only)
- **QueryMsg**:
  - `Auctions { start_after?, limit? }` → `[(u8, Auction)]`
  - `Auction { auction_id }` → `Auction`
  - `Params {}` → `Params`

## Prerequisites
- Rust toolchain with `wasm32-unknown-unknown` target
- Docker (for optimizer image)
- Node.js >= 16 and Yarn (for TypeScript tooling and deployment scripts)

## Build and optimize
```bash
# Debug build
cargo build --target wasm32-unknown-unknown

# Optimize WASM (outputs to ./artifacts)
./scripts/optimize.sh
```

## Generate JSON schema and TypeScript clients
```bash
# Generates ./schema via rust binary and then TS types in ./ts/types
./scripts/schema.sh

# Or run TS codegen directly (expects ./schema to exist)
cd ts && yarn install && yarn generate-ts
```

## Deploy and try it (example script)
The `deployment-scripts` package uploads, instantiates, creates an example auction, waits, then bids.

```bash
cd deployment-scripts
yarn install

# Configure chain details in configs/chain_config.json (provided example points to Mantra Dukong)

# Provide test accounts the script can use
# Create configs/test_accounts.ts with at least an admin account
```

```ts
// deployment-scripts/configs/test_accounts.ts
export const TestAccounts = [
  {
    name: "admin",
    mnemonic: "<mnemonic>",
    address: "<bech32>"
  }
]
```

```bash
# Ensure optimized artifact exists at ../artifacts/dutch_auction_launchpad.wasm
ls ../artifacts

# Upload, instantiate, create an auction and bid
yarn deploy-dutch-auction

# To force re-uploading code IDs
yarn deploy-dutch-auction -- -store-code
```

Logs are written to a rotating `test.log` file in `deployment-scripts`.

## Project layout
- `src/` Rust contract (messages in `msg.rs`, logic in `contract.rs`, domain in `auction.rs`)
- `scripts/` helper scripts for schema and optimization
- `ts/` generated TypeScript clients and codegen driver
- `deployment-scripts/` upload/instantiate/invoke example using CosmJS

## License
MIT
