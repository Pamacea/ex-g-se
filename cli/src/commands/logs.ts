import * as logger from '../utils/logger.js';
import { readFileSync, existsSync, watchFile, unwatchFile } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const LOG_DIR = resolve(dirname(fileURLToPath(import.meta.url)), '../../.ex-g-se/logs');

export async function run(options: {
  follow: boolean;
  since: string;
  type: string;
  limit: string;
}): Promise<void> {
  const logFile = resolve(LOG_DIR, 'raw_logs.json');

  if (!existsSync(logFile)) {
    logger.error('No logs found. Start EX-G-SE first.');
    process.exit(1);
  }

  // Read and parse logs
  const content = readFileSync(logFile, 'utf-8');
  const logs = JSON.parse(content);

  if (!logs.events || logs.events.length === 0) {
    logger.info('No events captured yet');
    return;
  }

  // Filter by type if specified
  let events = logs.events;
  if (options.type !== 'all') {
    events = events.filter((e: { type: string }) => e.type === options.type);
  }

  // Apply limit
  const limit = parseInt(options.limit);
  events = events.slice(-limit);

  // Display events
  for (const event of events) {
    const timestamp = new Date(event.ts).toLocaleTimeString();
    const type = (event.type as string).padEnd(12);
    console.log(`[${timestamp}] ${type} ${JSON.stringify(event.data)}`);
  }

  if (options.follow) {
    console.log('');
    logger.info('Following logs... (Ctrl+C to exit)');

    watchFile(logFile, { interval: 1000 }, () => {
      // Re-read and display new events
      const newContent = readFileSync(logFile, 'utf-8');
      const newLogs = JSON.parse(newContent);

      if (newLogs.events.length > logs.events.length) {
        const newEvents = newLogs.events.slice(logs.events.length);
        for (const event of newEvents) {
          const timestamp = new Date(event.ts).toLocaleTimeString();
          const type = (event.type as string).padEnd(12);
          console.log(`[${timestamp}] ${type} ${JSON.stringify(event.data)}`);
        }
        logs.events = newLogs.events;
      }
    });

    process.on('SIGINT', () => {
      unwatchFile(logFile);
      process.exit(0);
    });
  }
}
