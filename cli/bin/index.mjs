#!/usr/bin/env node

/**
 * EX-G-SE CLI entry point
 * ES module wrapper for loading the compiled TypeScript
 */

import { fileURLToPath } from 'url';
import { dirname, join, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Convert path to file:// URL for Windows compatibility
function toFileURL(path) {
  const absolute = resolve(path);
  return 'file://' + absolute.replace(/\\/g, '/');
}

// Try to load from dist/ (installed package) or src/ (development)
const distPath = join(__dirname, '..', 'dist', 'index.js');
const srcPath = join(__dirname, '..', 'src', 'index.ts');

let entryPoint;

try {
  // Try dist first (production)
  entryPoint = await import(toFileURL(distPath));
} catch (error) {
  try {
    // Fall back to src (development)
    entryPoint = await import(toFileURL(srcPath));
  } catch (srcError) {
    console.error('[ERROR] Failed to load EX-G-SE');
    console.error('');
    console.error('If running from source, ensure TypeScript is compiled:');
    console.error('  npm run build');
    console.error('');
    console.error('If installed via npm, try reinstalling:');
    console.error('  npm install -g @oalacea/ex-g-se-cli');
    process.exit(1);
  }
}

// Run the main function
if (entryPoint.main && typeof entryPoint.main === 'function') {
  entryPoint.main().catch((error) => {
    console.error(`[ERROR] ${error.message}`);
    process.exit(1);
  });
} else {
  console.error('[ERROR] Invalid entry point');
  process.exit(1);
}
