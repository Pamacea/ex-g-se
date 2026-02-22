import * as logger from '../utils/logger.js';
import { spawnBinary, killProcess } from '../utils/process.js';
import { writeFileSync, readFileSync, existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const PID_FILE = resolve(dirname(fileURLToPath(import.meta.url)), '../../.ex-g-se/ex-g-se.pid');

export async function run(options: {
  watch: string;
  clipboard: boolean;
  screenshot: boolean;
  interval: string;
  keyboard: boolean;
  output: string;
  daemon: boolean;
  verbose: boolean;
}): Promise<void> {
  if (options.verbose) {
    logger.setLevel(logger.LogLevel.DEBUG);
  }

  // Check if already running
  if (existsSync(PID_FILE)) {
    const pid = parseInt(readFileSync(PID_FILE, 'utf-8'));
    try {
      process.kill(pid, 0); // Check if process exists
      logger.error('EX-G-SE is already running');
      logger.info(`Run 'ex-g-se stop' to stop the existing daemon`);
      process.exit(1);
    } catch {
      // Process doesn't exist, clean up stale PID file
      logger.debug('Cleaning up stale PID file');
    }
  }

  // Build args
  const args: string[] = [];
  if (options.watch !== './') {
    args.push('--watch', options.watch);
  }
  if (options.clipboard) {
    args.push('--clipboard');
  }
  if (options.screenshot) {
    args.push('--screenshot', '--screenshot-interval', options.interval);
  }
  if (options.keyboard) {
    args.push('--keyboard');
  }
  if (options.verbose) {
    args.push('--verbose');
  }

  // Spawn the process
  logger.info('Starting EX-G-SE daemon...');
  logger.debug(`Args: ${args.join(' ')}`);

  const proc = spawnBinary({
    args,
    onOutput: (data) => {
      if (options.verbose) {
        process.stdout.write(data);
      }
    },
    onError: (data) => {
      process.stderr.write(data);
    },
  });

  // Write PID file
  const pidDir = dirname(PID_FILE);
  if (!existsSync(pidDir)) {
    logger.debug(`Creating directory: ${pidDir}`);
  }

  writeFileSync(PID_FILE, proc.pid?.toString() || '', 'utf-8');
  logger.success(`EX-G-SE started (PID: ${proc.pid})`);

  if (!options.daemon) {
    logger.info('Running in foreground. Press Ctrl+C to stop.');
    proc.on('exit', (code) => {
      logger.info(`EX-G-SE exited with code ${code}`);
      process.exit(code || 0);
    });
  } else {
    logger.info('Running in daemon mode');
    logger.info(`Run 'ex-g-se logs' to view captured activity`);
    process.exit(0);
  }
}
