#!/usr/bin/env node

/**
 * EX-G-SE - Main Entry Point with Secure Decryption
 *
 * Usage:
 *   npx @oalacea/ex-g-se config    Configure AI provider
 *   npx @oalacea/ex-g-se           Start recording (press Ctrl+Shift+X to stop)
 *
 * Environment variables (alternative):
 *   EX_G_SE_PROVIDER=openai
 *   EX_G_SE_API_KEY=sk-...
 */

const { execSync } = require('child_process');
const crypto = require('crypto');
const fs = require('fs');
const os = require('os');
const path = require('path');
const readline = require('readline');

// ============================================================================
// ENCRYPTION CONFIGURATION (same as config.js)
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
// CONFIG LOADING (Priority Order)
// ============================================================================

async function loadConfig() {
  // 1. Environment Variables (Priority #1 - for CI/CD)
  const envConfig = loadFromEnv();
  if (envConfig) {
    return envConfig;
  }

  // 2. Encrypted file (Priority #2 - for local dev)
  const configPath = path.join(os.homedir(), '.config', 'ex-g-se', 'settings.enc');

  if (fs.existsSync(configPath)) {
    try {
      const encrypted = JSON.parse(fs.readFileSync(configPath, 'utf8'));

      // Prompt for master password
      const masterPassword = await promptPassword('🔐 Mot de passe maître: ');

      // Decrypt
      const decrypted = decrypt(encrypted, masterPassword);
      const config = JSON.parse(decrypted);

      return config;
    } catch (error) {
      if (error.message.includes('Unsupported state')) {
        console.error('\n❌ Mot de passe incorrect');
      } else {
        console.error('\n❌ Erreur de déchiffrement:', error.message);
      }
      console.error('💡 Si vous avez oublié votre mot de passe, refaites:');
      console.error('   npx @oalacea/ex-g-se config\n');
      process.exit(1);
    }
  }

  // 3. No config found
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
// SESSION ANALYSIS (Local, no AI needed for basic analysis)
// ============================================================================

function detectIntents(events) {
  const intents = [];
  let currentIntent = null;
  let intentStart = null;
  let eventCount = 0;

  events.forEach(event => {
    const intent = detectIntentFromEvent(event);

    if (intent !== currentIntent) {
      if (currentIntent && eventCount > 3) {
        intents.push({
          intent: currentIntent,
          confidence: Math.min(eventCount / 10, 1),
          start_time: intentStart,
          end_time: event.ts,
        });
      }
      currentIntent = intent;
      intentStart = event.ts;
      eventCount = 1;
    } else {
      eventCount++;
    }
  });

  // Don't forget last intent
  if (currentIntent && eventCount > 3) {
    intents.push({
      intent: currentIntent,
      confidence: Math.min(eventCount / 10, 1),
      start_time: intentStart,
      end_time: events[events.length - 1].ts,
    });
  }

  return intents;
}

function detectIntentFromEvent(event) {
  if (event.type === 'fs_change') {
    const path = event.data.path || '';
    if (path.includes('test') || path.includes('spec')) return 'Testing';
    if (path.includes('doc') || path.endsWith('.md')) return 'Documentation';
    if (path.includes('config') || path.endsWith('.json')) return 'Configuration';
    return 'Feature Development';
  }

  if (event.type === 'clipboard') {
    const content = (event.data.content || '').toLowerCase();
    if (content.includes('error') || content.includes('bug')) return 'Bug Fixing';
    if (content.includes('test')) return 'Testing';
    return 'Feature Development';
  }

  return 'Feature Development';
}

function identifyKeyMoments(events) {
  const moments = [];
  let activityCluster = [];
  let lastEventTime = null;

  events.forEach(event => {
    if (event.type === 'fs_change') {
      activityCluster.push(event);

      if (lastEventTime) {
        const elapsed = Math.abs(new Date(event.ts) - new Date(lastEventTime)) / 1000 / 60;
        if (elapsed > 2 && activityCluster.length > 0) {
          moments.push({
            timestamp: activityCluster[0].ts,
            title: 'Activity Burst',
            description: `${activityCluster.length} files modified`,
            intent: 'Feature Development',
          });
          activityCluster = [];
        }
      }
      lastEventTime = event.ts;
    }
  });

  return moments;
}

function generateScript(analysis) {
  let markdown = `# Development Session\n\n`;
  markdown += `**Start**: ${analysis.start_time}\n`;
  markdown += `**End**: ${analysis.end_time}\n`;
  markdown += `**Intents**: ${analysis.intents.map(i => i.intent).join(', ')}\n\n`;
  markdown += `---\n\n`;

  let actNumber = 1;
  let sceneNumber = 1;

  analysis.intents.forEach((intent, i) => {
    markdown += `## ACT ${actNumber} - ${formatActTitle(intent.intent)}\n\n`;
    markdown += `**Time**: ${intent.start_time} - ${intent.end_time}\n`;
    markdown += `**Intent**: ${intent.intent}\n`;
    markdown += `**Confidence**: ${(intent.confidence * 100).toFixed(0)}%\n\n`;

    const intentMoments = analysis.key_moments.filter(moment => {
      const momentTime = new Date(moment.timestamp);
      const startTime = new Date(intent.start_time);
      const endTime = new Date(intent.end_time);
      return momentTime >= startTime && momentTime <= endTime;
    });

    if (intentMoments.length > 0) {
      intentMoments.forEach(moment => {
        markdown += `### Scene ${sceneNumber}: ${moment.title}\n\n`;
        markdown += `**Timestamp**: ${moment.timestamp}\n`;
        markdown += `**Description**: ${moment.description}\n\n`;
        markdown += `**Dialogue**:\n\n`;
        markdown += `> **NARRATOR**: ${moment.description}\n`;
        markdown += `> **DEVELOPER**: "${generateThought(moment.intent)}"\n\n`;
        sceneNumber++;
      });
    }

    actNumber++;
  });

  const exGseDir = '.ex-g-se';
  if (!fs.existsSync(exGseDir)) {
    fs.mkdirSync(exGseDir, { recursive: true });
  }

  fs.writeFileSync('.ex-g-se/session_script.md', markdown);
}

function generateVideoAssets(analysis) {
  const timeline = analysis.key_moments.map((moment, i) => {
    const nextMoment = analysis.key_moments[i + 1];
    let duration = 30;

    if (nextMoment) {
      const currentTime = new Date(moment.timestamp);
      const nextTime = new Date(nextMoment.timestamp);
      duration = Math.max(10, Math.floor((nextTime - currentTime) / 1000));
    }

    return {
      timestamp: moment.timestamp,
      duration_seconds: duration,
      title: moment.title,
      description: moment.description,
      screenshot: null,
      actions: [
        { type: 'highlight', target: 'current file', duration: 3 },
        { type: 'typewriter', text: moment.title, duration: 2 },
        { type: 'fade_out', duration: 1 },
      ],
      voiceover: `At this moment: ${moment.title}. ${moment.description}`,
    };
  });

  const videoAssetsDir = '.ex-g-se/video_assets';
  if (!fs.existsSync(videoAssetsDir)) {
    fs.mkdirSync(videoAssetsDir, { recursive: true });
  }

  fs.writeFileSync(
    '.ex-g-se/video_assets/scenes.json',
    JSON.stringify(timeline, null, 2)
  );
}

function formatActTitle(intent) {
  const titles = {
    'Bug Fixing': 'The Investigation',
    'Feature Development': 'The Creation',
    'Refactoring': 'The Improvement',
    'Testing': 'The Verification',
    'Deployment': 'The Release',
    'Documentation': 'The Documentation',
    'Configuration': 'The Setup',
    'Learning': 'The Exploration',
  };
  return titles[intent] || `The ${intent}`;
}

function generateThought(intent) {
  const thoughts = {
    'Bug Fixing': "Hmm, this isn't working. Let me debug this issue...",
    'Feature Development': "Now I'll implement this new feature...",
    'Refactoring': "This code could be cleaner. Let me refactor it...",
    'Testing': "Let me verify this works with a test...",
    'Deployment': "Time to deploy this to production...",
    'Documentation': "I should document this for future reference...",
    'Configuration': "Let me configure this setting...",
    'Learning': "Interesting! Let me explore how this works...",
  };
  return thoughts[intent] || "Working on the code...";
}

function generateSessionId() {
  return 'session_' + Date.now() + '_' + Math.random().toString(36).substr(2, 9);
}

function formatDuration(start, end) {
  const startTime = new Date(start);
  const endTime = new Date(end);
  const diff = (endTime - startTime) / 1000 / 60;
  if (diff < 60) return `${Math.round(diff)} minutes`;
  const hours = Math.floor(diff / 60);
  const mins = Math.round(diff % 60);
  return `${hours}h ${mins}m`;
}

// ============================================================================
// MAIN FLOW
// ============================================================================

async function main() {
  // Handle subcommand 'config'
  const args = process.argv.slice(2);
  if (args[0] === 'config') {
    // Execute config.js
    const { execSync } = require('child_process');
    const configPath = path.join(__dirname, 'config.js');
    execSync(`node "${configPath}"`, { stdio: 'inherit' });
    return;
  }

  console.log('\nEX-G-SE v0.3.2 - Ghost Mode Observability\n');

  // Load config
  const config = await loadConfig();

  if (!config) {
    console.error('❌ Aucune configuration trouvée !\n');
    console.error('Configurez EX-G-SE:\n');
    console.error('  npx @oalacea/ex-g-se config\n');
    console.error('Ou utilisez les variables d\'environnement:\n');
    console.error('  export EX_G_SE_PROVIDER=openai');
    console.error('  export EX_G_SE_API_KEY=sk-...\n');
    process.exit(1);
  }

  console.log(`✅ Configuration: ${config.provider} (${config.model})\n`);

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

  console.log(`▶ Démarrage de l'enregistrement (${platform}-${arch})...`);
  console.log(`⚠️  Appuyez sur Ctrl+Shift+X pour arrêter\n`);

  try {
    // Run the Rust binary
    execSync(`"${binaryPath}"`, { stdio: 'inherit' });
  } catch (e) {
    // Binary exited (user pressed Ctrl+Shift+X or Ctrl+C)
  }

  console.log(`\n⏸ Enregistrement terminé\n`);

  // Check if raw_logs.json was created
  if (!fs.existsSync('raw_logs.json')) {
    console.error('❌ Aucune donnée de session trouvée');
    process.exit(1);
  }

  console.log(`🔍 Analyse de la session...\n`);

  const logs = JSON.parse(fs.readFileSync('raw_logs.json', 'utf-8'));

  console.log(`  Événements: ${logs.events.length}`);
  console.log(`  Durée: ${formatDuration(logs.start, logs.end)}`);

  // Analyze
  const intents = detectIntents(logs.events);
  const keyMoments = identifyKeyMoments(logs.events);

  console.log(`  Intents détectés: ${intents.length}`);
  console.log(`  Moments clés: ${keyMoments.length}`);

  // Create .ex-g-se directory
  const exGseDir = '.ex-g-se';
  if (!fs.existsSync(exGseDir)) {
    fs.mkdirSync(exGseDir, { recursive: true });
  }

  // Save analysis
  const analysis = {
    session_id: generateSessionId(),
    start_time: logs.start,
    end_time: logs.end,
    intents: intents,
    key_moments: keyMoments,
    summary: `Session with ${logs.events.length} events`,
  };

  fs.writeFileSync(
    '.ex-g-se/session_analysis.json',
    JSON.stringify(analysis, null, 2)
  );

  // Generate outputs
  generateScript(analysis);
  generateVideoAssets(analysis);

  console.log(`\n✅ Session terminée !\n`);
  console.log(`📁 Fichiers générés:`);
  console.log(`  • .ex-g-se/session_analysis.json`);
  console.log(`  • .ex-g-se/session_script.md`);
  console.log(`  • .ex-g-se/video_assets/scenes.json\n`);
}

main().catch(error => {
  console.error('\n❌ Erreur:', error.message);
  process.exit(1);
});
