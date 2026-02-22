/**
 * Platform detection utilities
 * Maps Node.js platform/arch to EX-G-SE binary names
 */

import type { PlatformInfo } from './types.js';

/**
 * Get platform information and corresponding binary name
 */
export function getPlatformInfo(): PlatformInfo {
  const platform = process.platform as 'darwin' | 'linux' | 'win32';
  const arch = process.arch as 'x64' | 'arm64';

  if (platform !== 'darwin' && platform !== 'linux' && platform !== 'win32') {
    throw new Error(`Unsupported platform: ${platform}`);
  }

  if (arch !== 'x64' && arch !== 'arm64') {
    throw new Error(`Unsupported architecture: ${arch}`);
  }

  const binaryName = getBinaryName(platform, arch);

  return { platform, arch, binaryName };
}

/**
 * Map platform and architecture to binary filename
 */
function getBinaryName(
  platform: 'darwin' | 'linux' | 'win32',
  arch: 'x64' | 'arm64'
): string {
  const mapping: Record<string, string> = {
    'linux-x64': 'ex-g-se-linux',
    'linux-arm64': 'ex-g-se-linux-arm64',
    'win32-x64': 'ex-g-se-win.exe',
    'win32-arm64': 'ex-g-se-win-arm64.exe',
    'darwin-x64': 'ex-g-se-macos-intel',
    'darwin-arm64': 'ex-g-se-macos-silicon',
  };

  const key = `${platform}-${arch}`;
  const binaryName = mapping[key];

  if (!binaryName) {
    throw new Error(`No binary found for platform: ${key}`);
  }

  return binaryName;
}

/**
 * Get the expected binary path based on platform
 */
export function getBinaryPath(customPath?: string): string {
  if (customPath) {
    return customPath;
  }

  const { binaryName } = getPlatformInfo();
  const url = import.meta.url;
  const moduleDir = url.slice('file://'.length);

  // When running from installed package, binary is in bin/../binaries/
  // When running from dev, binary is in project root/binaries/
  const path = require('path');
  const distDir = path.dirname(moduleDir);
  const possiblePaths = [
    // Installed package location (dist/)
    path.join(path.dirname(distDir), 'binaries', binaryName),
    // Dev location (cli/src -> cli/../binaries)
    path.join(distDir, '..', '..', 'binaries', binaryName),
    // Relative to CWD
    path.join(process.cwd(), 'binaries', binaryName),
  ];

  return possiblePaths[0];
}

/**
 * Validate current platform support
 */
export function validatePlatform(): { valid: boolean; error?: string } {
  try {
    getPlatformInfo();
    return { valid: true };
  } catch (error) {
    return {
      valid: false,
      error: error instanceof Error ? error.message : 'Unknown platform error',
    };
  }
}

/**
 * Get download URL for current platform binary
 */
export function getBinaryDownloadUrl(): string {
  const { binaryName } = getPlatformInfo();
  return `https://github.com/oalacea/ex-g-se/releases/latest/download/${binaryName}`;
}
