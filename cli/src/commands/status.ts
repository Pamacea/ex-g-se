import * as logger from '../utils/logger.js';
import { readFileSync, existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const PID_FILE = resolve(dirname(fileURLToPath(import.meta.url)), '../../.ex-g-se/ex-g-se.pid');

export async function run(): Promise<void> {
  console.log('');
  console.log('EX-G-SE Status');
  console.log('=============');
  console.log('');

  if (!existsSync(PID_FILE)) {
    console.log('Daemon:     Stopped');
    console.log('');
    process.exit(0);
  }

  const pid = parseInt(readFileSync(PID_FILE, 'utf-8'));

  try {
    process.kill(pid, 0); // Check if process exists
    const startTime = process.pid === pid ? process.hrtime() : [0, 0];

    console.log(`Daemon:     Running`);
    console.log(`PID:        ${pid}`);
    console.log(`Started:    ${new Date().toISOString()}`);
    console.log('');
    console.log('Activity Summary');
    console.log('================');
    console.log('Files watched:    Unknown');
    console.log('Clipboard events: Unknown');
    console.log('Screenshots:      Unknown');
    console.log('Keystrokes:       Unknown');
    console.log('');
  } catch {
    console.log('Daemon:     Stale PID file');
    console.log('PID:        ' + pid);
    console.log('');
    logger.warn('Daemon is not running (cleaning up PID file)');
  }
}
