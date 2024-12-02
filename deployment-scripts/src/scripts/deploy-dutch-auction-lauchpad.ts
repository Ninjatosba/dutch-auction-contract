import { coin } from '@cosmjs/proto-signing';
import Context from '../helpers/context';
import { logger } from '../utils/logger';
import { chain } from 'lodash';
import chainConfig from '../../configs/chain_config.json'


const deployDutchAuction = async () => {
    let context = new Context;
    await context.initialize();
    await context.instantiateDutchAuctionLaunchpad();
    let startTime = context.getNanoTimestamp(10);
    let endTime = context.getNanoTimestamp(200);
    let offeredAsset = {
        denom: chainConfig.denom,
        amount: '1000000000'
    }
    let startingPrice = "2"
    let endPrice = "1"


    await context.createDutchAuction(
        offeredAsset,
        startingPrice,
        endPrice,
        startTime,
        endTime,
    );

}

deployDutchAuction().then(() => {
    logger.info('Dutch Auction Launchpad deployed');
    process.exit(0);
}).catch((error) => {
    logger.error(`Error deploying Dutch Auction Launchpad: ${error}`);
    process.exit(1);
});

