/**
 * Post-session handler
 * Parses logs and offers user actions
 */

import inquirer from 'inquirer';
import { writeFile, readFile } from 'fs/promises';
import { resolve } from 'path';
import { homedir } from 'os';

import type { RawLogs, SessionSummary, PostSessionChoice, ApiSynthesisResult } from './types.js';
import { RAW_LOGS_PATH } from './binary.js';
import { printError, printInfo, printSuccess, createSpinner, printSessionSummary } from './ui.js';

/**
 * Parse raw logs into session summary
 */
export function parseSessionSummary(rawLogs: RawLogs): SessionSummary {
  const startTime = new Date(rawLogs.start_time);
  const endTime = new Date(rawLogs.end_time);
  const duration = rawLogs.end_time - rawLogs.start_time;

  return {
    sessionId: rawLogs.session_id,
    duration,
    eventCount: rawLogs.events.length,
    startTime,
    endTime,
  };
}

/**
 * Call Oalacea API for synthesis
 */
export async function callOalaceaApi(
  logs: RawLogs
): Promise<ApiSynthesisResult> {
  const apiKey = process.env.OALACEA_KEY;

  if (!apiKey) {
    return {
      success: false,
      error: 'OALACEA_KEY environment variable not set',
    };
  }

  const spinner = createSpinner({ text: 'Uploading session to Oalacea...' });
  spinner.start();

  try {
    const response = await fetch('https://api.oalacea.com/v1/synthesize', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${apiKey}`,
      },
      body: JSON.stringify({
        logs,
        source: 'ex-g-se',
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`API error: ${response.status} - ${error}`);
    }

    const data = await response.json() as { url?: string };
    spinner.stop('Session uploaded successfully');

    return {
      success: true,
      url: data.url,
    };
  } catch (error) {
    spinner.error(`Upload failed: ${error}`);
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

/**
 * Open in Oalacea web synthesis
 */
export function openInBrowser(sessionId: string): void {
  const url = `https://oalacea.com/synthesize?session=${sessionId}`;
  printInfo(`Opening: ${url}`);

  const { exec } = require('child_process');

  const command = process.platform === 'win32'
    ? `start ${url}`
    : process.platform === 'darwin'
    ? `open ${url}`
    : `xdg-open ${url}`;

  exec(command, (error: Error | null) => {
    if (error) {
      printError(`Failed to open browser: ${error.message}`);
      printInfo(`Please open manually: ${url}`);
    }
  });
}

/**
 * Save logs to file
 */
export async function saveLogsToFile(
  logs: RawLogs,
  filePath?: string
): Promise<void> {
  const defaultPath = resolve(homedir(), `ex-g-se-session-${logs.session_id}.json`);
  const targetPath = filePath || defaultPath;

  const spinner = createSpinner({ text: `Saving to ${targetPath}...` });
  spinner.start();

  try {
    await writeFile(targetPath, JSON.stringify(logs, null, 2), 'utf-8');
    spinner.stop('Logs saved successfully');
  } catch (error) {
    spinner.error(`Save failed: ${error}`);
    throw error;
  }
}

/**
 * Prompt user for post-session action
 */
export async function promptPostSessionAction(
  logs: RawLogs
): Promise<PostSessionChoice> {
  const summary = parseSessionSummary(logs);
  printSessionSummary(summary);

  const hasApiKey = !!process.env.OALACEA_KEY;

  const { action } = await inquirer.prompt([
    {
      type: 'list',
      name: 'action',
      message: 'What would you like to do with this session?',
      choices: [
        { name: 'Open in browser (Oalacea synthesis)', value: 'browser' },
        ...(hasApiKey ? [{ name: 'Upload via API (OALACEA_KEY set)', value: 'api' }] : []),
        { name: 'Save to file', value: 'save' },
        { name: 'Exit', value: 'exit' },
      ],
    },
  ]);

  switch (action) {
    case 'browser':
      openInBrowser(logs.session_id);
      return { action: 'browser' };

    case 'api': {
      const result = await callOalaceaApi(logs);
      if (result.success && result.url) {
        printSuccess(`View synthesis: ${result.url}`);
        // Optionally open in browser
        const { openBrowser } = await inquirer.prompt([
          {
            type: 'confirm',
            name: 'openBrowser',
            message: 'Open in browser?',
            default: true,
          },
        ]);
        if (openBrowser) {
          openInBrowser(logs.session_id);
        }
      } else {
        printError(`API call failed: ${result.error}`);
      }
      return { action: 'api' };
    }

    case 'save': {
      const { filePath } = await inquirer.prompt([
        {
          type: 'input',
          name: 'filePath',
          message: 'Save path (press Enter for default):',
          default: resolve(homedir(), `ex-g-se-session-${logs.session_id}.json`),
        },
      ]);
      await saveLogsToFile(logs, filePath);
      return { action: 'save', filePath };
    }

    case 'exit':
    default:
      return { action: 'exit' };
  }
}

/**
 * Handle post-session workflow
 */
export async function handlePostSession(): Promise<void> {
  try {
    const content = await readFile(RAW_LOGS_PATH, 'utf-8');
    const logs: RawLogs = JSON.parse(content);

    await promptPostSessionAction(logs);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
      printInfo('No session data found');
    } else {
      printError(`Failed to read logs: ${error}`);
    }
  }
}
