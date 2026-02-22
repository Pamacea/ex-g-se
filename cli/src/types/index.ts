// Core event types from Rust engine
export interface LogEntry {
  ts: string;
  type: EventType;
  data: unknown;
}

export type EventType = 'fs_change' | 'clipboard' | 'screenshot' | 'trigger';

export interface FsChangeEvent {
  path: string;
  kind: string;
}

export interface ClipboardEvent {
  content: string;
  length: number;
}

export interface ScreenshotEvent {
  path: string;
}

export interface TriggerEvent {
  trigger: string;
  message: string;
}

// CLI configuration
export interface CliConfig {
  watch: string;
  clipboard: boolean;
  screenshot: boolean;
  keyboard: boolean;
  interval: number;
  output: string;
  daemon: boolean;
  verbose: boolean;
}

// Process state
export interface ProcessState {
  pid: number | null;
  status: 'running' | 'stopped' | 'unknown';
  startTime: Date | null;
}
