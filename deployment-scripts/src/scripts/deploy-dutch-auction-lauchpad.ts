import Context from '../helpers/context';
import { logger } from '../utils/logger';


const deployDutchAuction = async () => {
    let context = new Context;
    await context.initialize();
    await context.instantiateDutchAuctionLaunchpad();

}

deployDutchAuction().then(() => {
    logger.info('Dutch Auction Launchpad deployed');
    process.exit(0);
}).catch((error) => {
    logger.error(`Error deploying Dutch Auction Launchpad: ${error}`);
    process.exit(1);
});

