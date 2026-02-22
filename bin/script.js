#!/usr/bin/env node

/**
 * EX-G-SE Script Generator Command
 *
 * Generates theater-play format scripts from session analysis
 */

const fs = require('fs');
const path = require('path');

// ANSI colors
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  blue: '\x1b[34m',
  red: '\x1b[31m',
};

function generateScript(analysisPath) {
  if (!fs.existsSync(analysisPath)) {
    console.error(`${colors.red}❌ File not found:${colors.reset} ${analysisPath}`);
    process.exit(1);
  }

  console.log(`${colors.blue}🎭 Generating script...${colors.reset}\n`);

  const analysis = JSON.parse(fs.readFileSync(analysisPath, 'utf-8'));

  // Generate script in theater play format
  let markdown = `# Development Session\n\n`;
  markdown += `**Start**: ${analysis.start_time}\n`;
  markdown += `**End**: ${analysis.end_time}\n`;
  markdown += `**Intents**: ${analysis.intents.map(i => i.intent).join(', ')}\n\n`;

  markdown += `---\n\n`;

  // Generate acts from intents
  let actNumber = 1;
  let sceneNumber = 1;

  analysis.intents.forEach((intent, i) => {
    markdown += `## ACT ${actNumber} - ${formatActTitle(intent.intent)}\n\n`;
    markdown += `**Time**: ${intent.start_time} - ${intent.end_time}\n`;
    markdown += `**Intent**: ${intent.intent}\n`;
    markdown += `**Confidence**: ${(intent.confidence * 100).toFixed(0)}%\n\n`;

    // Find key moments in this intent's timeframe
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

        if (moment.screenshot) {
          markdown += `![Screenshot](${moment.screenshot})\n\n`;
        }

        // Generate dialogue
        markdown += `**Dialogue**:\n\n`;
        markdown += `> **NARRATOR**: ${moment.description}\n`;
        markdown += `> **DEVELOPER**: "${generateThought(moment.intent)}"\n\n`;

        sceneNumber++;
      });
    }

    actNumber++;
  });

  // Save script
  const scriptPath = '.ex-g-se/session_script.md';

  // Create directory if needed
  const exGseDir = '.ex-g-se';
  if (!fs.existsSync(exGseDir)) {
    fs.mkdirSync(exGseDir, { recursive: true });
  }

  fs.writeFileSync(scriptPath, markdown);

  console.log(`${colors.green}✅ Script generated!${colors.reset}`);
  console.log(`  ${colors.gray}Location:${colors.reset} ${scriptPath}\n`);

  // Also generate timeline JSON for video generation
  generateTimeline(analysis);
}

function generateTimeline(analysis) {
  const timeline = analysis.key_moments.map((moment, i) => {
    const nextMoment = analysis.key_moments[i + 1];
    let duration = 30; // default 30 seconds

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
      screenshot: moment.screenshot || null,
      actions: [
        {
          type: 'highlight',
          target: 'current file',
          duration: 3,
        },
        {
          type: 'typewriter',
          text: moment.title,
          duration: 2,
        },
        {
          type: 'fade_out',
          duration: 1,
        },
      ],
      voiceover: `At this moment, the developer is working on: ${moment.title}. ${moment.description}`,
    };
  });

  const timelinePath = '.ex-g-se/video_assets/scenes.json';

  // Create directory
  const videoAssetsDir = '.ex-g-se/video_assets';
  if (!fs.existsSync(videoAssetsDir)) {
    fs.mkdirSync(videoAssetsDir, { recursive: true });
  }

  fs.writeFileSync(timelinePath, JSON.stringify(timeline, null, 2));

  console.log(`${colors.green}✅ Timeline generated!${colors.reset}`);
  console.log(`  ${colors.gray}Location:${colors.reset} ${timelinePath}\n`);
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

// Main
const args = process.argv.slice(2);

if (args.length === 0) {
  console.error(`${colors.red}❌ Usage: ex-g-se-script <session_analysis.json>${colors.reset}\n`);
  process.exit(1);
}

const analysisPath = args[0];

try {
  generateScript(analysisPath);
} catch (error) {
  console.error(`${colors.red}❌ Error:${colors.reset} ${error.message}\n`);
  process.exit(1);
}
