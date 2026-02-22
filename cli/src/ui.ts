/**
 * Brutalist ASCII UI utilities
 * No colors, no formatting - just raw text and ASCII characters
 */

import type { SpinnerOptions } from './types.js';

/**
 * Simple ASCII spinner using ora but without colors
 */
export function createSpinner(options: SpinnerOptions) {
  const chars = ['/', '-', '\\', '|'];
  let i = 0;
  let interval: ReturnType<typeof setInterval> | null = null;
  const text = options.text || 'Working...';
  const indent = options.indent || 0;
  const prefix = ' '.repeat(indent);

  return {
    start: () => {
      interval = setInterval(() => {
        process.stdout.write(`\r${prefix}[${chars[i % 4]}] ${text}`);
        i++;
      }, 100);
    },
    stop: (finalText?: string) => {
      if (interval) {
        clearInterval(interval);
        interval = null;
      }
      const output = finalText || text;
      process.stdout.write(`\r${prefix}[+] ${output}\n`);
    },
    error: (errorText: string) => {
      if (interval) {
        clearInterval(interval);
        interval = null;
      }
      process.stdout.write(`\r${prefix}[x] ${errorText}\n`);
    },
  };
}

/**
 * Print a section header
 */
export function printHeader(title: string): void {
  const line = '='.repeat(60);
  process.stdout.write(`\n${line}\n`);
  process.stdout.write(`${title}\n`);
  process.stdout.write(`${line}\n\n`);
}

/**
 * Print error message with ASCII indicator
 */
export function printError(message: string): void {
  process.stderr.write(`\n[ERROR] ${message}\n\n`);
}

/**
 * Print warning message
 */
export function printWarning(message: string): void {
  process.stdout.write(`\n[WARN] ${message}\n\n`);
}

/**
 * Print info message
 */
export function printInfo(message: string): void {
  process.stdout.write(`\n[INFO] ${message}\n\n`);
}

/**
 * Print success message
 */
export function printSuccess(message: string): void {
  process.stdout.write(`\n[OK] ${message}\n\n`);
}

/**
 * Print a list of items
 */
export function printList(items: string[]): void {
  items.forEach((item, index) => {
    process.stdout.write(`  ${index + 1}. ${item}\n`);
  });
  process.stdout.write('\n');
}

/**
 * Print command output box
 */
export function printBox(title: string, content: string): void {
  const lines = content.split('\n');
  const maxLength = Math.max(title.length, ...lines.map((l) => l.length));
  const border = '+'.repeat(maxLength + 4);

  process.stdout.write(`\n${border}\n`);
  process.stdout.write(`| ${title.padEnd(maxLength)} |\n`);
  process.stdout.write(`${border}\n`);

  lines.forEach((line) => {
    process.stdout.write(`| ${line.padEnd(maxLength)} |\n`);
  });

  process.stdout.write(`${border}\n\n`);
}

/**
 * Clear current line
 */
export function clearLine(): void {
  process.stdout.write('\r' + ' '.repeat(process.stdout.columns || 80) + '\r');
}

/**
 * Print session summary
 */
export function printSessionSummary(summary: {
  sessionId: string;
  duration: number;
  eventCount: number;
}): void {
  const durationSecs = (summary.duration / 1000).toFixed(2);
  const lines = [
    `Session ID: ${summary.sessionId}`,
    `Duration: ${durationSecs} seconds`,
    `Events captured: ${summary.eventCount}`,
  ];

  printBox('SESSION SUMMARY', lines.join('\n'));
}
