import * as logger from '../utils/logger.js';
import { writeFileSync, readFileSync, existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const CONFIG_FILE = resolve(dirname(fileURLToPath(import.meta.url)), '../../.ex-g-se/config.json');

const DEFAULT_CONFIG = {
  watch: {
    dir: './',
    ignorePatterns: ['node_modules/**', '.git/**', 'dist/**'],
  },
  clipboard: {
    enabled: false,
    interval: 5000,
  },
  screenshot: {
    enabled: false,
    interval: 30000,
    format: 'png',
    quality: 80,
  },
  keyboard: {
    enabled: false,
    filterSensitive: true,
    ignorePatterns: ['password', 'secret', 'token'],
  },
  output: {
    dir: '.ex-g-se/logs',
    rotateSize: '100M',
    compress: true,
  },
  daemon: {
    pidFile: '.ex-g-se/ex-g-se.pid',
    autoStart: false,
  },
};

export async function run(command: string, key?: string, value?: string): Promise<void> {
  // Ensure config file exists
  if (!existsSync(CONFIG_FILE)) {
    writeFileSync(CONFIG_FILE, JSON.stringify(DEFAULT_CONFIG, null, 2), 'utf-8');
  }

  const config = JSON.parse(readFileSync(CONFIG_FILE, 'utf-8'));

  switch (command) {
    case 'get':
      if (!key) {
        logger.error('Key is required for get command');
        process.exit(1);
      }
      const keys = key.split('.');
      let result = config;
      for (const k of keys) {
        result = result[k];
        if (result === undefined) {
          logger.error(`Key not found: ${key}`);
          process.exit(1);
        }
      }
      console.log(JSON.stringify(result, null, 2));
      break;

    case 'set':
      if (!key || !value) {
        logger.error('Key and value are required for set command');
        process.exit(1);
      }
      const keys2 = key.split('.');
      let target = config;
      for (let i = 0; i < keys2.length - 1; i++) {
        target = target[keys2[i]];
      }
      target[keys2[keys2.length - 1]] = JSON.parse(value);
      writeFileSync(CONFIG_FILE, JSON.stringify(config, null, 2), 'utf-8');
      logger.success(`Set ${key} = ${value}`);
      break;

    case 'list':
      console.log(JSON.stringify(config, null, 2));
      break;

    case 'reset':
      writeFileSync(CONFIG_FILE, JSON.stringify(DEFAULT_CONFIG, null, 2), 'utf-8');
      logger.success('Configuration reset to defaults');
      break;

    default:
      logger.error(`Unknown command: ${command}`);
      logger.info('Available commands: get, set, list, reset');
      process.exit(1);
  }
}
