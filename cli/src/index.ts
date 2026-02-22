/**
 * EX-G-SE CLI wrapper entry point
 * TypeScript source - compiled to dist/index.js
 */

import { launchBinary } from './binary.js';
import { handlePostSession } from './session.js';
import { printError, printHeader } from './ui.js';

/**
 * Main CLI entry point
 */
export async function main(): Promise<void> {
  printHeader('EX-G-SE v0.1.0');

  // Get command line args
  const args = process.argv.slice(2);

  // Check for help flag
  if (args.includes('--help') || args.includes('-h')) {
    showHelp();
    return;
  }

  // Check for version flag
  if (args.includes('--version') || args.includes('-v')) {
    showVersion();
    return;
  }

  try {
    // Launch the binary
    const result = await launchBinary(args);

    // Handle post-session workflow
    if (!args.includes('--no-post-session')) {
      await handlePostSession();
    }

    // Exit with binary's exit code
    process.exit(result.exitCode ?? 0);
  } catch (error) {
    printError(`CLI error: ${error}`);
    process.exit(1);
  }
}

/**
 * Show help message
 */
function showHelp(): void {
  process.stdout.write(`
EX-G-SE: Execute and Get Session Enhanced

USAGE:
  ex-g-se [options]

OPTIONS:
  --help, -h           Show this help message
  --version, -v        Show version information
  --no-post-session    Skip post-session workflow

ENVIRONMENT:
  EXGSE_LOG_DIR        Custom log directory (default: ~/.exgse)
  OALACEA_KEY          API key for Oalacea synthesis

For more information: https://github.com/oalacea/ex-g-se

`);
}

/**
 * Show version information
 */
function showVersion(): void {
  process.stdout.write('EX-G-SE CLI v0.1.0\n');
  process.stdout.write('Node.js ' + process.version + '\n');
}

// Run main function if this is the entry point
if (import.meta.url === `file://${process.argv[1].replace(/\\/g, '/')}`) {
  main().catch((error) => {
    printError(`Unhandled error: ${error}`);
    process.exit(1);
  });
}
