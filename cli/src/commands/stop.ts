import * as logger from '../utils/logger.js';
import { killProcess } from '../utils/process.js';
import { readFileSync, unlinkSync, existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const PID_FILE = resolve(dirname(fileURLToPath(import.meta.url)), '../../.ex-g-se/ex-g-se.pid');

export async function run(options: { force: boolean }): Promise<void> {
  if (!existsSync(PID_FILE)) {
    logger.error('EX-G-SE is not running');
    process.exit(1);
  }

  const pid = parseInt(readFileSync(PID_FILE, 'utf-8'));
  logger.info(`Stopping EX-G-SE (PID: ${pid})...`);

  const killed = killProcess(pid, options.force ? 'SIGKILL' : 'SIGTERM');

  if (killed) {
    unlinkSync(PID_FILE);
    logger.success('EX-G-SE stopped');
  } else {
    if (options.force) {
      logger.error('Failed to kill process');
      process.exit(1);
    } else {
      logger.warn('Graceful shutdown failed');
      logger.info('Try: ex-g-se stop --force');
    }
  }
}
