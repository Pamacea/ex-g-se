#!/usr/bin/env node

/**
 * EX-G-SE CLI entry point
 * Shebang executable that launches the compiled TypeScript
 */

const path = require('path');

// Try to load from dist/ (installed package) or src/ (development)
const distPath = path.join(__dirname, '..', 'dist', 'index.js');
const srcPath = path.join(__dirname, '..', 'src', 'index.js');

let entryPoint;

try {
  // Try dist first (production)
  entryPoint = require(distPath);
} catch (error) {
  try {
    // Fall back to src (development with ts-node or similar)
    entryPoint = require(srcPath);
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
