/**
 * TypeScript type definitions for EX-G-SE CLI wrapper
 */

export interface PlatformInfo {
  platform: 'darwin' | 'linux' | 'win32';
  arch: 'x64' | 'arm64';
  binaryName: string;
}

export interface BinaryLaunchResult {
  success: boolean;
  exitCode: number | null;
  error?: Error;
}

export interface RawLogEntry {
  timestamp: number;
  event: string;
  data?: unknown;
}

export interface RawLogs {
  session_id: string;
  start_time: number;
  end_time: number;
  events: RawLogEntry[];
}

export interface SessionSummary {
  sessionId: string;
  duration: number;
  eventCount: number;
  startTime: Date;
  endTime: Date;
}

export type PostSessionAction = 'browser' | 'api' | 'save' | 'exit';

export interface PostSessionChoice {
  action: PostSessionAction;
  filePath?: string;
}

export interface CliOptions {
  binaryPath?: string;
  logDir?: string;
  skipPostSession?: boolean;
}

export interface SpinnerOptions {
  text: string;
  indent?: number;
}

export interface ApiSynthesisResult {
  success: boolean;
  url?: string;
  error?: string;
}
