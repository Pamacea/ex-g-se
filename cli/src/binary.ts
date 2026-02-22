/**
 * Binary launcher and management
 */

import { spawn, ChildProcess } from 'child_process';
import { existsSync } from 'fs';
import { chmod } from 'fs/promises';
import { resolve, join, dirname } from 'path';
import { homedir } from 'os';
import { fileURLToPath } from 'url';

import type { BinaryLaunchResult, CliOptions } from './types.js';
import { getPlatformInfo, getBinaryPath } from './platform.js';
import { printError, printInfo, printSuccess, createSpinner } from './ui.js';

// Get __dirname equivalent in ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * EX-G-SE data directory
 */
export const EXGSE_DIR = join(homedir(), '.exgse');

/**
 * Raw logs file path
 */
export const RAW_LOGS_PATH = join(EXGSE_DIR, 'raw_logs.json');

/**
 * Ensure the EX-G-SE directory exists
 */
export async function ensureDataDir(): Promise<void> {
  const fs = await import('fs/promises');
  try {
    await fs.mkdir(EXGSE_DIR, { recursive: true });
  } catch (error) {
    throw new Error(`Failed to create data directory: ${error}`);
  }
}

/**
 * Make binary executable
 */
export async function makeExecutable(binaryPath: string): Promise<void> {
  if (process.platform === 'win32') {
    return; // Windows doesn't use execute permissions
  }

  try {
    await chmod(binaryPath, 0o755);
  } catch (error) {
    throw new Error(`Failed to make binary executable: ${error}`);
  }
}

/**
 * Check if binary exists at given path
 */
export function binaryExists(path: string): boolean {
  return existsSync(path);
}

/**
 * Find binary in various locations
 */
export function findBinary(): string | null {
  const { binaryName } = getPlatformInfo();

  const searchPaths = [
    // Local development
    join(process.cwd(), 'binaries', binaryName),
    // Package installation
    join(__dirname, '..', 'binaries', binaryName),
    // Data directory
    join(EXGSE_DIR, 'bin', binaryName),
  ];

  for (const path of searchPaths) {
    if (binaryExists(path)) {
      return path;
    }
  }

  return null;
}

/**
 * Show download instructions when binary is missing
 */
export function showDownloadInstructions(): void {
  const { binaryName, platform, arch } = getPlatformInfo();
  const downloadUrl = `https://github.com/oalacea/ex-g-se/releases/latest/download/${binaryName}`;

  printError('EX-G-SE binary not found');
  printInfo(`Platform: ${platform}-${arch}`);
  printInfo(`Expected binary: ${binaryName}`);

  process.stdout.write('To install:\n\n');
  process.stdout.write(`  1. Download: ${downloadUrl}\n`);
  process.stdout.write(`  2. Save to: ${EXGSE_DIR}/bin/${binaryName}\n`);

  if (process.platform !== 'win32') {
    process.stdout.write(`  3. Make executable: chmod +x ${EXGSE_DIR}/bin/${binaryName}\n`);
  }

  process.stdout.write('\nOr install via npm:\n');
  process.stdout.write(`  npm install -g @oalacea/ex-g-se-cli\n\n`);
}

/**
 * Launch the EX-G-SE binary
 */
export async function launchBinary(
  args: string[],
  options: CliOptions = {}
): Promise<BinaryLaunchResult> {
  await ensureDataDir();

  const binaryPath = options.binaryPath || findBinary();

  if (!binaryPath) {
    showDownloadInstructions();
    return {
      success: false,
      exitCode: 1,
      error: new Error('Binary not found'),
    };
  }

  // Ensure binary is executable
  await makeExecutable(binaryPath);

  const spinner = createSpinner({ text: 'Starting EX-G-SE...' });
  spinner.start();

  return new Promise((resolve) => {
    const child = spawn(binaryPath, args, {
      stdio: 'inherit',
      env: {
        ...process.env,
        EXGSE_LOG_DIR: EXGSE_DIR,
      },
    });

    spinner.stop();

    let resolved = false;

    // Handle process exit
    child.on('exit', (code, signal) => {
      if (resolved) return;
      resolved = true;

      if (signal === 'SIGINT' || signal === 'SIGTERM') {
        printInfo('Session interrupted');
        resolve({ success: true, exitCode: 0 });
      } else {
        resolve({
          success: code === 0,
          exitCode: code,
          error: code !== 0 ? new Error(`Process exited with code ${code}`) : undefined,
        });
      }
    });

    // Handle process error
    child.on('error', (error) => {
      if (resolved) return;
      resolved = true;

      printError(`Failed to launch binary: ${error.message}`);
      resolve({ success: false, exitCode: null, error });
    });

    // Forward SIGINT to child process
    process.on('SIGINT', () => {
      child.kill('SIGINT');
    });
  });
}

/**
 * Get raw logs from file
 */
export async function getRawLogs(): Promise<unknown> {
  const fs = await import('fs/promises');

  try {
    const content = await fs.readFile(RAW_LOGS_PATH, 'utf-8');
    return JSON.parse(content);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
      return null;
    }
    throw error;
  }
}
