import { ExecuteResult, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import chainConfig from '../../configs/chain_config.json'
import { TestAccounts } from '../../configs/test_accounts.ts'
import { getSigningClient } from '../utils/client'
import fs from 'fs'
import _ from 'lodash'
import path from 'path'
import { logger } from '../utils/logger'
import { coin, Coin, DirectSecp256k1HdWallet } from '@cosmjs/proto-signing'
import { InstantiateMsg as DutchAuctionLaunchpadInstantiateMsg } from '../types/DutchAuctionLaunchpad.types';
import { DutchAuctionLaunchpadClient } from '../types/DutchAuctionLaunchpad.client.ts'
export const CONTRACT_MAP = {
    DUTCH_AUCTION_LAUNCHPAD: 'dutch_auction_launchpad',
}

const UPLOAD_CONTRACTS = process.argv.includes('-store-code');
const testAccounts = TestAccounts

export type TestUser = {
    name: string
    address: string
    client: SigningCosmWasmClient
}

export type TestUserMap = { [name: string]: TestUser }

export default class Context {
    private codeIds: { [key: string]: number } = {}
    private events: string[] = []
    private contracts: { [key: string]: string[] } = {}
    collection_id: string = ''
    private testCachePath: string = path.join(__dirname, '../../tmp/test_cache.json')
    private testUserMap: TestUserMap = {}

    private initializeTestUsers = async () => {
        for (let i = 0; i < testAccounts.length; i++) {
            const mnemonic = testAccounts[i].mnemonic
            const signingClient = await getSigningClient(mnemonic)
            const testAccount = testAccounts[i]
            this.testUserMap[testAccount.name] = {
                name: testAccount.name,
                address: testAccounts[i].address,
                client: signingClient.client,
            }

            logger.log(1, `Test user ${testAccount.name} Balance: ${await this.checkAccountBalance(chainConfig.denom, testAccount.name) / 1000000}`)
        }
    }

    private uploadContracts = async () => {
        let { client, address: sender } = this.getTestUser('admin')
        let fileNames = fs.readdirSync(chainConfig.artifacts_path)
        logger.log(1, `Uploading contracts from ${fileNames}`)
        let wasmFileNames = _.filter(fileNames, (fileName) => _.endsWith(fileName, '.wasm'))

        for (const idx in wasmFileNames) {
            let wasmFileName = wasmFileNames[idx]
            if (!_.includes(_.values(CONTRACT_MAP), wasmFileName.replace('.wasm', ''))) {
                continue
            }
            let wasmFilePath = path.join(chainConfig.artifacts_path, wasmFileName)
            let wasmFile = fs.readFileSync(wasmFilePath, { encoding: null })
            let uploadResult = await client.upload(sender, wasmFile, "auto")
            let codeIdKey = wasmFileName.replace('-aarch64', '').replace('.wasm', '')
            this.codeIds[codeIdKey] = uploadResult.codeId
            logger.log(1, `Uploaded ${codeIdKey} contract with codeId ${uploadResult.codeId}`)
        }
    }

    private writeContext = () => {
        const dir = path.dirname(this.testCachePath)

        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true })
        }

        fs.writeFileSync(
            this.testCachePath,
            JSON.stringify({
                codeIds: this.codeIds,
                contracts: this.contracts,
            }),
        )
    }

    readTempContext = () => {
        if (fs.existsSync(this.testCachePath)) {
            const context = JSON.parse(fs.readFileSync(this.testCachePath, 'utf-8'))
            this.codeIds = context.codeIds
            this.contracts = context.contracts
        }
        else {
            // return error
            throw new Error('No test cache file found')

        }
    }

    initialize = async () => {
        await this.initializeTestUsers()
        if ((fs.existsSync(this.testCachePath)) && (!UPLOAD_CONTRACTS)) {
            this.readTempContext()
        }
        else {
            await this.uploadContracts()
            this.writeContext()
        }

    }

    getTestUser = (userName: string) => {
        return this.testUserMap[userName]
    }

    getCodeId = (codeIdKey: string) => {
        return this.codeIds[codeIdKey]
    }

    getContractKeyByCodeId = (codeId: number) => {
        return _.findKey(this.codeIds, (value, key) => value === codeId)
    }

    getContractAddress = (contractKey: string, index: number = 0) => {
        try {
            return this.contracts[contractKey][index]
        } catch {
            console.log(`error ${contractKey} ${index} ${JSON.stringify(this.contracts)}}`)
        }
        return this.contracts[contractKey][index]
    }

    instantiateDutchAuctionLaunchpad = async () => {
        let { client, address: sender } = this.getTestUser('admin')
        let codeId = this.getCodeId(CONTRACT_MAP.DUTCH_AUCTION_LAUNCHPAD)
        let initMsg: DutchAuctionLaunchpadInstantiateMsg = {
            accepted_denoms: [chainConfig.denom],
            admin: sender,
            auction_creation_fee: { denom: chainConfig.denom, amount: '0' },
            max_auction_duration: 604800,
            min_seconds_until_auction_start: 1
        }
        let res = await client.instantiate(sender, codeId, initMsg, "test_dutch_auction", "auto")
        logger.log(1, `Instantiated ${CONTRACT_MAP.DUTCH_AUCTION_LAUNCHPAD} contract with address ${res.contractAddress}`)
        logger.log(1, `Tx Hash: ${res.transactionHash}`)
        this.addContractAddress(CONTRACT_MAP.DUTCH_AUCTION_LAUNCHPAD, res.contractAddress)
    }

    createDutchAuction = async (
        offeredAsset: { denom: string, amount: string },
        startPrice: string,
        endPrice: string,
        startTime: string,
        endTime: string,
    ) => {
        let { client, address: sender } = this.getTestUser('admin')
        let contractAddress = this.getContractAddress(CONTRACT_MAP.DUTCH_AUCTION_LAUNCHPAD)
        let dutchAuctionClient = new DutchAuctionLaunchpadClient(client, sender, contractAddress)
        let res = await dutchAuctionClient.createAuction({
            startTime: startTime,
            endTime: endTime,
            endPrice: endPrice,
            offeredAsset: offeredAsset,
            startingPrice: startPrice,
            inDenom: chainConfig.denom,

        }, "auto", undefined, [
            offeredAsset
        ])
        logger.log(1, `Created Dutch Auction with auction id ${this.getEventAttribute(res, 'wasm', 'auction_id')}`)
        logger.log(1, `Tx Hash: ${res.transactionHash}`)
    }

    bidDutchAuction = async (
        auctionId: number,
        amount: string,
    ) => {
        let { client, address: sender } = this.getTestUser('admin')
        let contractAddress = this.getContractAddress(CONTRACT_MAP.DUTCH_AUCTION_LAUNCHPAD)
        let dutchAuctionClient = new DutchAuctionLaunchpadClient(client, sender, contractAddress)
        let res = await dutchAuctionClient.bid({ auctionId: auctionId }, "auto", undefined, [
            coin(amount, chainConfig.denom)
        ]
        )
        logger.log(1, `Bid on Dutch Auction with auction id ${auctionId}`)
        logger.log(1, `Tx Hash: ${res.transactionHash}`)
    }

    addContractAddress = (contractKey: string, contractAddress: string) => {
        this.contracts[contractKey] = _.extend([], this.contracts[contractKey], [contractAddress])
    }

    checkAccountBalance = async (denom: string, account_name: string) => {
        let { client, address: sender } = this.getTestUser(account_name)
        let res = await client.getBalance(sender, denom)
        return Number(res.amount)
    }

    generateWallet = async (amount: number) => {
        let { client, address: sender } = this.getTestUser('admin');
        let wallets: DirectSecp256k1HdWallet[] = []
        for (let i = 0; i < amount; i++) {
            let wallet = await (await DirectSecp256k1HdWallet.generate(12, { prefix: chainConfig.prefix }))
            wallets.push(wallet)
        }
        return wallets
    }

    getNewWallets = async () => {
        // Get test Users length
        let newAccounts = await Promise.all(testAccounts.map(async (account: any) => {
            let wallet = await DirectSecp256k1HdWallet.generate(12, { prefix: chainConfig.prefix })
            let address = await wallet.getAccounts()
            let mnemonic = await wallet.mnemonic
            account.mnemonic = mnemonic
            account.address = address[0].address
            //await fetch(chainConfig.faucet_endpoint + `?address=${account.address}&token=uflix`)
            console.log(`New account created with address: ${account.address}`)
            return account
        }))
        fs.writeFileSync(path.join(__dirname, '../../configs/test_accounts.ts'), "export const TestAccounts = " + JSON.stringify(newAccounts, null, 2))
    }

    normalizeCoins = (coins: Coin[]): Coin[] => {
        const coinMap: { [denom: string]: number } = {};

        // Sum amounts by denom
        coins.forEach((coin) => {
            if (coin.amount !== '0') {
                if (coinMap[coin.denom]) {
                    coinMap[coin.denom] += Number(coin.amount);
                } else {
                    coinMap[coin.denom] = Number(coin.amount);
                }
            }
        });

        // Convert the map back to an array of Coin objects
        const normalizedCoins: Coin[] = Object.keys(coinMap).map((denom) => {
            return { denom, amount: coinMap[denom].toString() };
        });

        // Sort alphabetically by denom
        normalizedCoins.sort((a, b) => a.denom.localeCompare(b.denom));

        return normalizedCoins;
    };

    generateRandomSalt = (length: number) => {
        const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
        let result = '';

        for (let i = 0; i < length; i++) {
            const randomIndex = Math.floor(Math.random() * characters.length);
            result += characters.charAt(randomIndex);
        }

        return btoa(result);
    };

    updateEvent = (event: string) => {
        const currentTimestamp = new Date().toUTCString()
        this.events.push(`[${currentTimestamp}] ${event}`)
    }
    getEventAttribute = (res: ExecuteResult, eventType = 'wasm', attributeKey: string) => {
        const event = res.events.find(event => event.type === eventType);

        if (event) {
            const attribute = event.attributes.find(attr => attr.key === attributeKey);
            return attribute ? attribute.value : '';
        }

        return "";
    }
    getNanoTimestamp = (seconds: number): string => {
        let nowNano = new Date().getTime() * 1000000;
        let nanoSeconds = nowNano + seconds * 1000000000;
        return nanoSeconds.toString();
    }


}