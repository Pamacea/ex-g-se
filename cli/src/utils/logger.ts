import chalk from 'chalk';

export enum LogLevel {
  DEBUG = 0,
  INFO = 1,
  WARN = 2,
  ERROR = 3,
}

let currentLevel: LogLevel = LogLevel.INFO;

export function setLevel(level: LogLevel): void {
  currentLevel = level;
}

export function debug(message: string): void {
  if (currentLevel <= LogLevel.DEBUG) {
    console.log(chalk.gray(`[DEBUG] ${message}`));
  }
}

export function info(message: string): void {
  if (currentLevel <= LogLevel.INFO) {
    console.log(chalk.white(`[INFO] ${message}`));
  }
}

export function warn(message: string): void {
  if (currentLevel <= LogLevel.WARN) {
    console.log(chalk.yellow(`[WARN] ${message}`));
  }
}

export function error(message: string): void {
  if (currentLevel <= LogLevel.ERROR) {
    console.error(chalk.red(`[ERROR] ${message}`));
  }
}

export function success(message: string): void {
  console.log(chalk.green(`[OK] ${message}`));
}
