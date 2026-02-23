#!/usr/bin/env node

/**
 * EX-G-SE v4.0.0 - Main Entry Point with All Features
 *
 * Commands:
 *   exg config          Configure AI provider
 *   exg record          Start recording session
 *   exg record --label  Record with label
 *   exg stats           Show session stats
 *   exg list            List all sessions
 *   exg search          Search sessions
 *   exg export          Export session
 *   exg update          Update to latest version
 */

const { execSync } = require('child_process');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');
const readline = require('readline');

// Import chalk for colors (ESM)
let chalk;
try {
  chalk = require('chalk');
} catch (e) {
  // Fallback if chalk not available
  chalk = {
    green: (t) => `\x1b[32m${t}\x1b[0m`,
    red: (t) => `\x1b[31m${t}\x1b[0m`,
    yellow: (t) => `\x1b[33m${t}\x1b[0m`,
    blue: (t) => `\x1b[34m${t}\x1b[0m`,
    cyan: (t) => `\x1b[36m${t}\x1b[0m`,
    gray: (t) => `\x1b[90m${t}\x1b[0m`,
    bold: (t) => `\x1b[1m${t}\x1b[0m`
  };
}

// ============================================================================
// ENCRYPTION CONFIGURATION
// ============================================================================

const ARGON2_CONFIG = {
  memoryCost: 65536,
  timeCost: 3,
  parallelism: 4,
  keyLength: 32,
  saltLength: 16,
};

function deriveKey(password, salt) {
  return crypto.scryptSync(
    Buffer.from(password, 'utf8'),
    salt,
    ARGON2_CONFIG.keyLength,
    {
      N: ARGON2_CONFIG.memoryCost,
      r: ARGON2_CONFIG.parallelism,
      p: ARGON2_CONFIG.parallelism,
      maxmem: 256 * 1024 * 1024,
    }
  );
}

function decrypt(encryptedData, password) {
  const salt = Buffer.from(encryptedData.salt, 'hex');
  const iv = Buffer.from(encryptedData.iv, 'hex');
  const authTag = Buffer.from(encryptedData.authTag, 'hex');

  const key = deriveKey(password, salt);

  const decipher = crypto.createDecipheriv('aes-256-gcm', key, iv);
  decipher.setAuthTag(authTag);

  let decrypted = decipher.update(encryptedData.encrypted, 'hex', 'utf8');
  decrypted += decipher.final('utf8');

  return decrypted;
}

// ============================================================================
// CONFIG LOADING
// ============================================================================

async function loadConfig() {
  // 1. Environment Variables
  const envConfig = loadFromEnv();
  if (envConfig) {
    return envConfig;
  }

  // 2. Encrypted file
  const configPath = path.join(os.homedir(), '.config', 'ex-g-se', 'settings.enc');

  if (fs.existsSync(configPath)) {
    try {
      const encrypted = JSON.parse(fs.readFileSync(configPath, 'utf8'));
      const masterPassword = await promptPassword('🔐 Mot de passe maître: ');
      const decrypted = decrypt(encrypted, masterPassword);
      const config = JSON.parse(decrypted);
      return config;
    } catch (error) {
      if (error.message.includes('Unsupported state')) {
        console.error(chalk.red('❌ Mot de passe incorrect'));
      } else {
        console.error(chalk.red('❌ Erreur de déchiffrement:'), error.message);
      }
      console.error(chalk.gray('💡 Si vous avez oublié votre mot de passe, refaites:'));
      console.error(chalk.gray('   exg config\n'));
      process.exit(1);
    }
  }

  return null;
}

function loadFromEnv() {
  const provider = process.env.EX_G_SE_PROVIDER;
  const apiKey = process.env.EX_G_SE_API_KEY;
  const apiUrl = process.env.EX_G_SE_API_URL;
  const model = process.env.EX_G_SE_MODEL;

  if (!provider || !apiKey) {
    return null;
  }

  return {
    provider,
    api_key: apiKey,
    api_url: apiUrl || getDefaultUrl(provider),
    model: model || getDefaultModel(provider),
  };
}

function getDefaultUrl(provider) {
  const defaults = {
    openai: 'https://api.openai.com/v1',
    anthropic: 'https://api.anthropic.com/v1',
    'z.ai': 'https://api.z.ai/v1',
  };
  return defaults[provider] || '';
}

function getDefaultModel(provider) {
  const defaults = {
    openai: 'gpt-4o',
    anthropic: 'claude-3-5-sonnet-20241022',
    'z.ai': 'zai-latest',
  };
  return defaults[provider] || '';
}

function promptPassword(question) {
  return new Promise((resolve) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    process.stdout.write(question);
    process.stdin.setRawMode(true);
    process.stdin.resume();
    process.stdin.setEncoding('utf8');

    let password = '';

    const onData = (char) => {
      if (char === '\r' || char === '\n' || char === '\u0004') {
        process.stdin.setRawMode(false);
        process.stdin.pause();
        process.stdin.removeListener('data', onData);
        process.stdout.write('\n');
        rl.close();
        resolve(password);
      } else if (char === '\u007f') {
        if (password.length > 0) {
          password = password.slice(0, -1);
          process.stdout.write('\b \b');
        }
      } else if (char.length === 1) {
        password += char;
        process.stdout.write('*');
      }
    };

    process.stdin.on('data', onData);
  });
}

// ============================================================================
// SESSION MANAGEMENT
// ============================================================================

function getSessionsDir() {
  const sessionsDir = path.join(os.homedir(), '.ex-g-se', 'sessions');
  if (!fs.existsSync(sessionsDir)) {
    fs.mkdirSync(sessionsDir, { recursive: true });
  }
  return sessionsDir;
}

function listSessions() {
  const sessionsDir = getSessionsDir();
  const files = fs.readdirSync(sessionsDir)
    .filter(f => f.endsWith('.json'))
    .sort()
    .reverse();

  if (files.length === 0) {
    console.log(chalk.yellow('\n⚠️  No sessions found\n'));
    return;
  }

  console.log(chalk.bold('\n📚 Sessions:\n'));

  files.forEach(file => {
    const filePath = path.join(sessionsDir, file);
    const session = JSON.parse(fs.readFileSync(filePath, 'utf8'));
    const startTime = new Date(session.start_time);
    const label = session.label || chalk.gray('(no label)');
    const tags = session.tags ? session.tags.join(', ') : '';

    console.log(`  ${chalk.cyan(file.replace('.json', ''))}`);
    console.log(`    ${chalk.gray('Label:')} ${label}`);
    if (tags) {
      console.log(`    ${chalk.gray('Tags:')} ${tags}`);
    }
    console.log(`    ${chalk.gray('Date:')} ${startTime.toLocaleString()}`);
    console.log(`    ${chalk.gray('Events:')} ${session.events?.length || 0}`);
    console.log('');
  });
}

function searchSessions(query) {
  const sessionsDir = getSessionsDir();
  const files = fs.readdirSync(sessionsDir)
    .filter(f => f.endsWith('.json'));

  if (files.length === 0) {
    console.log(chalk.yellow('\n⚠️  No sessions found\n'));
    return;
  }

  console.log(chalk.bold(`\n🔍 Searching for: "${query}"\n`));

  const results = [];

  files.forEach(file => {
    const filePath = path.join(sessionsDir, file);
    const session = JSON.parse(fs.readFileSync(filePath, 'utf8'));

    // Search in label, tags
    const labelMatch = session.label?.toLowerCase().includes(query.toLowerCase());
    const tagsMatch = session.tags?.some(t => t.toLowerCase().includes(query.toLowerCase()));

    // Search in events
    const eventsMatch = session.events?.some(e => {
      const content = JSON.stringify(e).toLowerCase();
      return content.includes(query.toLowerCase());
    });

    if (labelMatch || tagsMatch || eventsMatch) {
      results.push({ file, session, labelMatch, tagsMatch, eventsMatch });
    }
  });

  if (results.length === 0) {
    console.log(chalk.yellow('No matches found\n'));
    return;
  }

  console.log(chalk.green(`Found ${results.length} session(s):\n`));

  results.forEach(({ file, session, labelMatch, tagsMatch, eventsMatch }) => {
    const reasons = [];
    if (labelMatch) reasons.push('label');
    if (tagsMatch) reasons.push('tags');
    if (eventsMatch) reasons.push('events');

    console.log(`  ${chalk.cyan(file.replace('.json', ''))}`);
    console.log(`    ${chalk.gray('Matched in:')} ${reasons.join(', ')}`);
    console.log(`    ${chalk.gray('Label:')} ${session.label || '(no label)'}`);
    console.log('');
  });
}

function showSessionStats(sessionId) {
  const sessionsDir = getSessionsDir();

  if (!sessionId) {
    // Show stats for most recent session
    const files = fs.readdirSync(sessionsDir)
      .filter(f => f.endsWith('.json'))
      .sort()
      .reverse();

    if (files.length === 0) {
      console.log(chalk.yellow('\n⚠️  No sessions found\n'));
      return;
    }

    sessionId = files[0].replace('.json', '');
  }

  const filePath = path.join(sessionsDir, `${sessionId}.json`);

  if (!fs.existsSync(filePath)) {
    console.error(chalk.red(`\n❌ Session not found: ${sessionId}\n`));
    process.exit(1);
  }

  const session = JSON.parse(fs.readFileSync(filePath, 'utf8'));

  // Handle both old (start_time) and new (start) field names
  const startStr = session.start_time || session.start;
  const endStr = session.end_time || session.end;

  const startTime = new Date(startStr);
  const endTime = new Date(endStr);
  const duration = (endTime - startTime) / 1000;

  console.log(chalk.bold('\n📊 Session Statistics\n'));
  console.log(`  ${chalk.gray('ID:')} ${chalk.cyan(sessionId)}`);
  console.log(`  ${chalk.gray('Label:')} ${session.label || '(no label)'}`);
  if (session.tags) {
    console.log(`  ${chalk.gray('Tags:')} ${session.tags.join(', ')}`);
  }
  console.log(`  ${chalk.gray('Duration:')} ${formatDurationSeconds(duration)}`);
  console.log(`  ${chalk.gray('Start:')} ${startTime.toLocaleString()}`);
  console.log(`  ${chalk.gray('End:')} ${endTime.toLocaleString()}`);

  const events = session.events || [];
  console.log(`\n  ${chalk.gray('Total Events:')} ${events.length}`);

  // Breakdown by type (handle both type and event_type)
  const breakdown = {};
  events.forEach(e => {
    const type = e.type || e.event_type || 'unknown';
    breakdown[type] = (breakdown[type] || 0) + 1;
  });

  console.log(`\n  ${chalk.bold('Event Breakdown:')}`);
  Object.entries(breakdown).forEach(([type, count]) => {
    console.log(`    ${type}: ${count}`);
  });

  // Disk usage
  const stats = fs.statSync(filePath);
  console.log(`\n  ${chalk.gray('Disk Usage:')} ${formatBytes(stats.size)}`);
  console.log('');
}

// ============================================================================
// EXPORT FUNCTIONS
// ============================================================================

function exportSession(sessionId, format) {
  const sessionsDir = getSessionsDir();

  if (!sessionId) {
    // Use most recent session
    const files = fs.readdirSync(sessionsDir)
      .filter(f => f.endsWith('.json'))
      .sort()
      .reverse();

    if (files.length === 0) {
      console.error(chalk.yellow('\n⚠️  No sessions found\n'));
      return;
    }

    sessionId = files[0].replace('.json', '');
  }

  const filePath = path.join(sessionsDir, `${sessionId}.json`);

  if (!fs.existsSync(filePath)) {
    console.error(chalk.red(`\n❌ Session not found: ${sessionId}\n`));
    process.exit(1);
  }

  const session = JSON.parse(fs.readFileSync(filePath, 'utf8'));

  switch (format.toLowerCase()) {
    case 'json':
      exportJson(session, sessionId);
      break;
    case 'markdown':
    case 'md':
      exportMarkdown(session, sessionId);
      break;
    case 'csv':
      exportCsv(session, sessionId);
      break;
    default:
      console.error(chalk.red(`\n❌ Unknown format: ${format}`));
      console.error(chalk.gray('Supported formats: json, markdown, csv\n'));
      process.exit(1);
  }
}

function exportJson(session, sessionId) {
  const outputFile = `${sessionId}_export.json`;
  fs.writeFileSync(outputFile, JSON.stringify(session, null, 2));
  console.log(chalk.green(`\n✅ Exported to: ${outputFile}\n`));
}

function exportMarkdown(session, sessionId) {
  let md = `# Session: ${sessionId}\n\n`;
  md += `**Label:** ${session.label || 'No label'}\n`;
  md += `**Start:** ${new Date(session.start_time).toLocaleString()}\n`;
  md += `**End:** ${new Date(session.end_time).toLocaleString()}\n`;
  if (session.tags) {
    md += `**Tags:** ${session.tags.join(', ')}\n`;
  }
  md += `\n---\n\n## Events\n\n`;

  const events = session.events || [];
  events.slice(0, 100).forEach((e, i) => {
    md += `### ${i + 1}. ${e.type}\n`;
    md += `**Time:** ${e.ts}\n\n`;
    md += `\`\`\`json\n${JSON.stringify(e.data, null, 2)}\n\`\`\`\n\n`;
  });

  if (events.length > 100) {
    md += `\n*_${events.length - 100} more events not shown_*\n`;
  }

  const outputFile = `${sessionId}_export.md`;
  fs.writeFileSync(outputFile, md);
  console.log(chalk.green(`\n✅ Exported to: ${outputFile}\n`));
}

function exportCsv(session, sessionId) {
  const events = session.events || [];
  let csv = 'timestamp,type,data\n';

  events.forEach(e => {
    const data = JSON.stringify(e.data).replace(/"/g, '""');
    csv += `${e.ts},${e.type},"${data}"\n`;
  });

  const outputFile = `${sessionId}_export.csv`;
  fs.writeFileSync(outputFile, csv);
  console.log(chalk.green(`\n✅ Exported to: ${outputFile}\n`));
}

// ============================================================================
// CONFIG LIST COMMAND
// ============================================================================

async function listConfig() {
  const configPath = path.join(os.homedir(), '.config', 'ex-g-se', 'settings.enc');

  if (!fs.existsSync(configPath)) {
    console.log(chalk.yellow('\n⚠️  No configuration found\n'));
    console.log(chalk.gray('Run: exg config\n'));
    return;
  }

  try {
    const encrypted = JSON.parse(fs.readFileSync(configPath, 'utf8'));
    const stats = fs.statSync(configPath);

    console.log(chalk.bold('\n📋 Current Configuration\n'));
    console.log(`  ${chalk.gray('Encrypted:')} ${chalk.green('Yes')}`);
    console.log(`  ${chalk.gray('File:')} ${configPath}`);
    console.log(`  ${chalk.gray('Size:')} ${formatBytes(stats.size)}`);
    console.log(`  ${chalk.gray('Created:')} ${stats.birthtime.toLocaleString()}`);
    console.log(`  ${chalk.gray('Modified:')} ${stats.mtime.toLocaleString()}`);
    console.log('');

    // Try to decrypt and show provider
    const masterPassword = await promptPassword(chalk.cyan('🔐 Enter master password to view details: '));
    const decrypted = decrypt(encrypted, masterPassword);
    const config = JSON.parse(decrypted);

    console.log(`  ${chalk.gray('Provider:')} ${chalk.cyan(config.provider)}`);
    console.log(`  ${chalk.gray('Model:')} ${config.model}`);
    console.log(`  ${chalk.gray('API URL:')} ${config.api_url}`);
    console.log(`  ${chalk.gray('API Key:')} ${chalk.gray('••••••••••••')}${config.api_key.slice(-4)}`);
    console.log('');
  } catch (error) {
    if (error.message.includes('Unsupported state')) {
      console.error(chalk.red('\n❌ Incorrect password\n'));
    } else {
      console.error(chalk.red('\n❌ Error:'), error.message, '\n');
    }
  }
}

async function validateConfig() {
  console.log(chalk.bold('\n🧪 Testing API Connection...\n'));

  const config = await loadConfig();

  if (!config) {
    console.error(chalk.red('❌ No configuration found\n'));
    console.error(chalk.gray('Run: exg config\n'));
    process.exit(1);
  }

  try {
    const url = config.api_url.replace(/\/v\d+$/, '/v1');
    const response = await fetch(`${url}/models`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${config.api_key}`,
      },
    });

    if (response.ok) {
      console.log(chalk.green('✅ API connection successful!\n'));
      console.log(`  Provider: ${chalk.cyan(config.provider)}`);
      console.log(`  Model: ${chalk.cyan(config.model)}\n`);
    } else if (response.status === 401) {
      console.error(chalk.yellow('⚠️  API key is invalid or expired\n'));
      console.error(chalk.gray('Run: exg config\n'));
      process.exit(1);
    } else {
      console.error(chalk.yellow(`⚠️  API returned status: ${response.status}\n`));
    }
  } catch (error) {
    console.error(chalk.red('❌ Connection failed:'), error.message);
    console.error(chalk.gray('\nCheck your internet connection and API URL\n'));
    process.exit(1);
  }
}

// ============================================================================
// HELPERS
// ============================================================================

function formatDurationSeconds(seconds) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);

  if (hours > 0) {
    return `${hours}h ${minutes}m ${secs}s`;
  } else if (minutes > 0) {
    return `${minutes}m ${secs}s`;
  } else {
    return `${secs}s`;
  }
}

function formatBytes(bytes) {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}

// ============================================================================
// MAIN FLOW
// ============================================================================

function showHelp() {
  console.log(chalk.bold('\nEX-G-SE v0.5.1 - Ghost Mode Observability\n'));
  console.log(chalk.cyan('Commands:\n'));
  console.log('  exg, exg --help, exg -h   Show this help message');
  console.log('  exg version, exg --version, exg -v  Show version information');
  console.log('  exg config              Configure AI provider');
  console.log('  exg config list         Show current configuration');
  console.log('  exg config test         Test API connection');
  console.log('  exg record              Start recording session');
  console.log('  exg record --label      Record with custom label');
  console.log('  exg record --duration   Auto-stop after N minutes');
  console.log('  exg record --max-events Stop after N events');
  console.log('  exg record --tags       Add tags to session');
  console.log('  exg stats [session]     Show session statistics');
  console.log('  exg list                List all sessions');
  console.log('  exg search <query>      Search sessions');
  console.log('  exg export <format>     Export session (json, markdown, csv)');
  console.log('  exg update              Update to latest version\n');
  console.log(chalk.gray('Examples:\n'));
  console.log('  exg record --label "Fixing bug #123"');
  console.log('  exg record --duration 30 --tags bugfix,payment');
  console.log('  exg export markdown > session.md\n');
}

async function main() {
  const args = process.argv.slice(2);

  // No arguments = show help
  if (args.length === 0) {
    showHelp();
    return;
  }

  const command = args[0];

  // ============================================================================
  // VERSION COMMAND
  // ============================================================================

  if (command === '--version' || command === '-v' || command === 'version') {
    // Version is hardcoded to avoid requiring package.json during runtime
    console.log(chalk.cyan('\nEX-G-SE v0.5.1\n'));
    console.log(chalk.gray('Rust Core: v0.5.1'));
    console.log(chalk.gray('Released: 2026-02-23\n'));
    return;
  }

  // ============================================================================
  // CONFIG COMMANDS
  // ============================================================================

  if (command === 'config') {
    const subCommand = args[1];

    if (subCommand === 'list') {
      await listConfig();
      return;
    }

    if (subCommand === 'test') {
      await validateConfig();
      return;
    }

    // Default: run config wizard
    const configPath = path.join(__dirname, 'config.js');
    execSync(`node "${configPath}"`, { stdio: 'inherit' });
    return;
  }

  // ============================================================================
  // UPDATE COMMAND
  // ============================================================================

  if (command === 'update') {
    console.log(chalk.cyan('\n🔄 Updating to latest version...\n'));
    console.log(chalk.gray('This will install the latest version from NPM.\n'));
    try {
      execSync('npm install -g @oalacea/ex-g-se@latest', { stdio: 'inherit' });
      console.log(chalk.green('\n✅ Updated successfully!\n'));
      console.log(chalk.gray('Current version: 0.5.1\n'));
    } catch (error) {
      console.error(chalk.red('\n❌ Update failed:'), error.message);
      process.exit(1);
    }
    return;
  }

  // ============================================================================
  // LIST COMMAND
  // ============================================================================

  if (command === 'list') {
    listSessions();
    return;
  }

  // ============================================================================
  // SEARCH COMMAND
  // ============================================================================

  if (command === 'search') {
    const query = args[1];
    if (!query) {
      console.error(chalk.red('\n❌ Usage: exg search <query>\n'));
      process.exit(1);
    }
    searchSessions(query);
    return;
  }

  // ============================================================================
  // STATS COMMAND
  // ============================================================================

  if (command === 'stats') {
    const sessionId = args[1];
    showSessionStats(sessionId);
    return;
  }

  // ============================================================================
  // EXPORT COMMAND
  // ============================================================================

  if (command === 'export') {
    const format = args[1] || 'json';
    const sessionId = args[2];
    exportSession(sessionId, format);
    return;
  }

  // ============================================================================
  // RECORD COMMAND
  // ============================================================================

  if (command === 'record' || command === 'rec') {
    // Parse options
    let label = null;
    let tags = [];
    let duration = null;
    let maxEvents = null;

    for (let i = 1; i < args.length; i++) {
      if (args[i] === '--label' && args[i + 1]) {
        label = args[++i];
      } else if (args[i] === '--tags' && args[i + 1]) {
        tags = args[++i].split(',').map(t => t.trim());
      } else if (args[i] === '--duration' && args[i + 1]) {
        duration = parseInt(args[++i]);
      } else if (args[i] === '--max-events' && args[i + 1]) {
        maxEvents = parseInt(args[++i]);
      }
    }

    // Load config
    const config = await loadConfig();

    if (!config) {
      console.error(chalk.red('\n❌ No configuration found!\n'));
      console.error(chalk.gray('Configure EX-G-SE:\n'));
      console.error(chalk.gray('  exg config\n'));
      console.error(chalk.gray('Or use environment variables:\n'));
      console.error(chalk.gray('  export EX_G_SE_PROVIDER=openai'));
      console.error(chalk.gray('  export EX_G_SE_API_KEY=sk-...\n'));
      process.exit(1);
    }

    console.log(chalk.green(`✅ Configuration: ${config.provider} (${config.model})\n`));

    const platform = os.platform();
    const arch = os.arch();

    let binaryName = '';
    if (platform === 'linux') {
      binaryName = 'ex-g-se-linux';
    } else if (platform === 'win32') {
      binaryName = 'ex-g-se-win.exe';
    } else if (platform === 'darwin') {
      binaryName = arch === 'arm64' ? 'ex-g-se-macos-silicon' : 'ex-g-se-macos-intel';
    }

    const binaryPath = path.join(__dirname, binaryName);

    // Show session info
    console.log(chalk.bold('▶ Starting Recording Session\n'));
    if (label) {
      console.log(`  ${chalk.gray('Label:')} ${chalk.cyan(label)}`);
    }
    if (tags.length > 0) {
      console.log(`  ${chalk.gray('Tags:')} ${chalk.cyan(tags.join(', '))}`);
    }
    if (duration) {
      console.log(`  ${chalk.gray('Auto-stop:')} ${chalk.yellow(duration + ' minutes')}`);
    }
    if (maxEvents) {
      console.log(`  ${chalk.gray('Max events:')} ${chalk.yellow(maxEvents)}`);
    }
    console.log(`  ${chalk.gray('Platform:')} ${platform}-${arch}`);
    console.log(chalk.gray('\n⚠️  Press Ctrl+Shift+X or Ctrl+C to stop\n'));
    console.log(chalk.gray('⏸️  Session will be saved and you can press ENTER to exit\n'));

    try {
      // Pass config to Rust binary via environment variables
      const env = {
        ...process.env,
        EX_G_SE_PROVIDER: config.provider,
        EX_G_SE_API_KEY: config.api_key,
        EX_G_SE_API_URL: config.api_url,
        EX_G_SE_MODEL: config.model,
      };

      // Run the Rust binary (it will save directly to ~/.ex-g-se/sessions/)
      execSync(`"${binaryPath}"`, { stdio: 'inherit', env });
    } catch (e) {
      // Binary exited (normal)
    }

    // The Rust binary now handles everything:
    // - Saves to ~/.ex-g-se/sessions/ with timestamp
    // - Shows summary
    // - Waits for ENTER to exit

    console.log(chalk.green('\n✅ Recording complete!\n'));
    console.log(chalk.gray('Session saved to: ~/.ex-g-se/sessions/\n'));
    console.log(chalk.gray('Use:'));
    console.log(chalk.gray('  exg list     - List all sessions'));
    console.log(chalk.gray('  exg stats    - Show session statistics'));
    console.log(chalk.gray('  exg search   - Search sessions\n'));

    return;
  }

  // ============================================================================
  // UNKNOWN COMMAND
  // ============================================================================

  console.error(chalk.red(`\n❌ Unknown command: ${command}\n`));
  showHelp();
  process.exit(1);
}

main().catch(error => {
  console.error(chalk.red('\n❌ Error:'), error.message);
  process.exit(1);
});
