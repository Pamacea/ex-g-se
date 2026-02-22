import { spawn, ChildProcess } from 'node:child_process';
import { resolve } from 'node:path';
import { existsSync } from 'node:fs';
import * as logger from './logger.js';

const BINARY_PATHS = [
  resolve(__dirname, '../../core/target/release/ex-g-se'),
  resolve(__dirname, '../../bin/ex-g-se'),
  '/usr/local/bin/ex-g-se',
].filter((p) => existsSync(p));

export function getBinaryPath(): string {
  const path = BINARY_PATHS[0];
  if (!path) {
    throw new Error('EX-G-SE binary not found. Run: npm run build:rust');
  }
  return path;
}

export interface SpawnOptions {
  args: string[];
  cwd?: string;
  env?: NodeJS.ProcessEnv;
  onOutput?: (data: string) => void;
  onError?: (data: string) => void;
}

export function spawnBinary(options: SpawnOptions): ChildProcess {
  const binaryPath = getBinaryPath();
  logger.debug(`Spawning: ${binaryPath} ${options.args.join(' ')}`);

  const proc = spawn(binaryPath, options.args, {
    cwd: options.cwd,
    env: { ...process.env, ...options.env },
  });

  if (options.onOutput) {
    proc.stdout?.on('data', (data: Buffer) => {
      options.onOutput!(data.toString());
    });
  }

  if (options.onError) {
    proc.stderr?.on('data', (data: Buffer) => {
      options.onError!(data.toString());
    });
  }

  proc.on('error', (err) => {
    logger.error(`Failed to spawn binary: ${err.message}`);
  });

  return proc;
}

export function killProcess(pid: number, signal: NodeJS.Signals = 'SIGTERM'): boolean {
  try {
    process.kill(pid, signal);
    return true;
  } catch {
    return false;
  }
}
