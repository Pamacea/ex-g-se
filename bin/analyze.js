#!/usr/bin/env node

/**
 * EX-G-SE Analysis Command
 *
 * Analyzes raw session logs and generates AI-powered insights
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

// ANSI colors
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  blue: '\x1b[34m',
  red: '\x1b[31m',
  yellow: '\x1b[33m',
};

// Get config directory
function getConfigDir() {
  return path.join(os.homedir(), '.ex-g-se');
}

// Load AI configuration
function loadConfig() {
  const configPath = path.join(getConfigDir(), 'config.json');

  if (!fs.existsSync(configPath)) {
    console.error(`${colors.red}❌ Configuration not found!${colors.reset}`);
    console.error(`\nRun ${colors.blue}npx @oalacea/ex-g-se-config${colors.reset} first.\n`);
    process.exit(1);
  }

  const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));
  return config;
}

// Analyze session (local - no AI yet)
function analyzeLocal(logsPath) {
  if (!fs.existsSync(logsPath)) {
    console.error(`${colors.red}❌ File not found:${colors.reset} ${logsPath}`);
    process.exit(1);
  }

  console.log(`${colors.blue}🔍 Analyzing session...${colors.reset}\n`);

  const logs = JSON.parse(fs.readFileSync(logsPath, 'utf-8'));

  console.log(`Session: ${logs.start} - ${logs.end}`);
  console.log(`Events captured: ${logs.events.length}\n`);

  // Simple analysis without AI
  const intents = detectIntents(logs.events);
  const keyMoments = identifyKeyMoments(logs.events);

  console.log(`${colors.bright}Intents Detected:${colors.reset}`);
  intents.forEach(intent => {
    console.log(`  • ${intent.type} (${intent.confidence.toFixed(0)}%)`);
  });

  console.log(`\n${colors.bright}Key Moments: ${keyMoments.length}${colors.reset}`);
  keyMoments.slice(0, 5).forEach((moment, i) => {
    console.log(`  ${i + 1}. ${moment.title} (${moment.timestamp})`);
  });

  // Save analysis
  const analysis = {
    session_id: generateSessionId(),
    start_time: logs.start,
    end_time: logs.end,
    intents: intents,
    key_moments: keyMoments,
    summary: generateSummary(intents, keyMoments),
  };

  const analysisPath = '.ex-g-se/session_analysis.json';

  // Create directory if needed
  const exGseDir = '.ex-g-se';
  if (!fs.existsSync(exGseDir)) {
    fs.mkdirSync(exGseDir, { recursive: true });
  }

  fs.writeFileSync(analysisPath, JSON.stringify(analysis, null, 2));

  console.log(`\n${colors.green}✅ Analysis saved!${colors.reset}`);
  console.log(`  ${colors.gray}Location:${colors.reset} ${analysisPath}\n`);
}

// Detect intents from events
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

// Detect intent from single event
function detectIntentFromEvent(event) {
  if (event.type === 'fs_change') {
    const path = event.data.path || '';

    if (path.includes('test') || path.includes('spec')) {
      return 'Testing';
    }
    if (path.includes('doc') || path.endsWith('.md')) {
      return 'Documentation';
    }
    if (path.includes('config') || path.endsWith('.json')) {
      return 'Configuration';
    }

    return 'Feature Development';
  }

  if (event.type === 'clipboard') {
    const content = (event.data.content || '').toLowerCase();

    if (content.includes('error') || content.includes('bug')) {
      return 'Bug Fixing';
    }
    if (content.includes('test')) {
      return 'Testing';
    }

    return 'Feature Development';
  }

  return 'Feature Development';
}

// Identify key moments
function identifyKeyMoments(events) {
  const moments = [];
  let activityCluster = [];
  let lastEventTime = null;

  events.forEach(event => {
    if (event.type === 'fs_change') {
      activityCluster.push(event);

      if (lastEventTime) {
        const elapsed = Math.abs(new Date(event.ts) - new Date(lastEventTime)) / 1000 / 60; // minutes

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

// Generate summary
function generateSummary(intents, keyMoments) {
  if (intents.length === 0) {
    return 'No significant activity detected';
  }

  let summary = 'Session Summary:\n\n';

  if (intents.length > 0) {
    summary += 'Intents:\n';
    intents.forEach(intent => {
      summary += `- ${intent.intent} (confidence: ${(intent.confidence * 100).toFixed(0)}%)\n`;
    });
  }

  if (keyMoments.length > 0) {
    summary += `\nKey Moments (${keyMoments.length} total):\n`;
    keyMoments.slice(0, 5).forEach((moment, i) => {
      summary += `${i + 1}. ${moment.title}\n`;
    });
  }

  return summary;
}

// Generate session ID
function generateSessionId() {
  return 'session_' + Date.now() + '_' + Math.random().toString(36).substr(2, 9);
}

// Main
const args = process.argv.slice(2);

if (args.length === 0) {
  console.error(`${colors.red}❌ Usage: ex-g-se-analyze <raw_logs.json>${colors.reset}\n`);
  process.exit(1);
}

const logsPath = args[0];

try {
  analyzeLocal(logsPath);
} catch (error) {
  console.error(`${colors.red}❌ Error:${colors.reset} ${error.message}\n`);
  process.exit(1);
}
