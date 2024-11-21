import Context from '../helpers/context';
import { logger } from '../utils/logger';


const createChannel = async () => {
    let context = new Context();
    await context.initialize();

}

createChannel().then(() => {
    logger.info('Channel created successfully');
}).catch((error) => {
    logger.error(`Error creating channel: ${error}`);
});

