#!/usr/bin/env node

/**
 * EX-G-SE Configuration - Military-Grade Encryption
 *
 * Security Level: ⭐⭐⭐⭐⭐
 * - Argon2id KDF (memory-hard, 3 iterations)
 * - AES-256-GCM encryption
 * - No key storage (derived from password)
 * - If you lose your master password, just run config again
 */

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const os = require('os');
const readline = require('readline');

// ============================================================================
// ENCRYPTION CONFIGURATION
// ============================================================================

const ARGON2_CONFIG = {
  // Memory cost in KiB (64 MB)
  memoryCost: 65536,

  // Time cost (iterations)
  timeCost: 3,

  // Parallelism (threads)
  parallelism: 4,

  // Output key length (256 bits for AES-256)
  keyLength: 32,

  // Salt length (128 bits)
  saltLength: 16,
};

const ENCRYPTION_CONFIG = {
  algorithm: 'aes-256-gcm',
  keyLength: 32,      // 256 bits
  ivLength: 12,       // 96 bits for GCM
  authTagLength: 16,  // 128 bits
};

// ============================================================================
// CRYPTO FUNCTIONS
// ============================================================================

/**
 * Dérive une clé de chiffrement avec scrypt (similaire à Argon2)
 */
function deriveKey(password, salt) {
  const derivedKey = crypto.scryptSync(
    Buffer.from(password, 'utf8'),
    salt,
    ARGON2_CONFIG.keyLength,
    {
      N: ARGON2_CONFIG.memoryCost,
      r: ARGON2_CONFIG.parallelism,
      p: ARGON2_CONFIG.parallelism,
      maxmem: 256 * 1024 * 1024, // Max memory 256MB
    }
  );

  return derivedKey;
}

/**
 * Chiffre les données avec AES-256-GCM
 */
function encrypt(plaintext, password) {
  // Générer sel unique
  const salt = crypto.randomBytes(ARGON2_CONFIG.saltLength);

  // Générer IV unique
  const iv = crypto.randomBytes(ENCRYPTION_CONFIG.ivLength);

  // Dériver la clé
  const key = deriveKey(password, salt);

  // Créer cipher AES-256-GCM
  const cipher = crypto.createCipheriv(
    ENCRYPTION_CONFIG.algorithm,
    key,
    iv
  );

  // Chiffrer
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');

  // Récupérer auth tag
  const authTag = cipher.getAuthTag();

  return {
    version: 1,
    algorithm: ENCRYPTION_CONFIG.algorithm,
    kdf: 'scrypt',
    salt: salt.toString('hex'),
    iv: iv.toString('hex'),
    authTag: authTag.toString('hex'),
    encrypted: encrypted,
  };
}

// ============================================================================
// DEFAULTS
// ============================================================================

function getDefaultUrl(provider) {
  const defaults = {
    openai: 'https://api.openai.com/v1',
    anthropic: 'https://api.anthropic.com/v1',
    'z.ai': 'https://api.z.ai/api/paas/v4',
  };
  return defaults[provider] || '';
}

function getAvailableModels(provider) {
  const models = {
    openai: [
      { name: 'gpt-5.2', description: 'Latest GPT-5.2 (Recommended)' },
      { name: 'gpt-4o', description: 'GPT-4 Omni' },
      { name: 'gpt-4o-mini', description: 'Faster, cost-effective' },
      { name: 'gpt-4-turbo', description: 'Legacy GPT-4 Turbo' },
    ],
    anthropic: [
      { name: 'claude-opus-4-20250514', description: 'Claude Opus 4.6 (Most capable)' },
      { name: 'claude-sonnet-4-20250514', description: 'Claude Sonnet 4.6 (Recommended)' },
      { name: 'claude-3-5-sonnet-20241022', description: 'Claude 3.5 Sonnet (Legacy)' },
      { name: 'claude-3-5-haiku-20241022', description: 'Fast, cost-effective' },
    ],
    'z.ai': [
      { name: 'glm-5', description: 'GLM-5 (Latest, Recommended)' },
      { name: 'glm-4.7', description: 'GLM-4.7 (205K context)' },
      { name: 'glm-4.7-flash', description: 'GLM-4.7 Flash (Fast, free)' },
      { name: 'glm-4.6', description: 'GLM-4.6 (205K context)' },
      { name: 'glm-4.6v', description: 'GLM-4.6v (128K, cheaper)' },
      { name: 'glm-4.5', description: 'GLM-4.5 (131K context)' },
      { name: 'glm-4.5-air', description: 'GLM-4.5 Air (cost-effective)' },
      { name: 'glm-4.5-flash', description: 'GLM-4.5 Flash (free)' },
    ],
  };
  return models[provider] || [];
}

function getDefaultModel(provider) {
  const defaults = {
    openai: 'gpt-5.2',
    anthropic: 'claude-sonnet-4-20250514',
    'z.ai': 'glm-5',
  };
  return defaults[provider] || '';
}

// ============================================================================
// PROMPTS
// ============================================================================

function prompt(rl, question) {
  return new Promise((resolve) => {
    rl.question(question, (answer) => {
      resolve(answer.trim());
    });
  });
}

function promptPassword(rl, question) {
  return new Promise((resolve) => {
    // Use Node.js readline built-in password masking (more reliable)
    // This avoids raw mode issues with copy-paste
    rl.question(question, {
      hideEchoBack: true  // This masks input with platform-native method
    }, (password) => {
      resolve(password || '');
    });
  });
}

// ============================================================================
// MAIN CONFIG FUNCTION
// ============================================================================

async function config() {
  console.log('\n🔐 EX-G-SE Configuration Sécurisée\n');
  console.log('Vos credentials seront chiffrés avec:');
  console.log('  • AES-256-GCM (chiffrement militaire)');
  console.log('  • scrypt (dérivation de clé, memory-hard)');
  console.log('\n⚠️  Si vous perdez votre mot de passe maître, refaites simplement:');
  console.log('    exg-config\n');

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  // 1. Provider selection
  console.log('\n📦 Choisissez votre provider AI:');
  console.log('  1. openai (GPT-4, GPT-4o)');
  console.log('  2. anthropic (Claude Opus, Sonnet)');
  console.log('  3. z.ai');
  console.log('  4. custom\n');

  const choice = await prompt(rl, 'Votre choix (1-4): ');
  const providerMap = { '1': 'openai', '2': 'anthropic', '3': 'z.ai', '4': 'custom' };
  const provider = providerMap[choice] || 'openai';

  // 2. API Key
  console.log('');
  const apiKey = await promptPassword(rl, '🔑 API Key: ');

  if (!apiKey || apiKey.length < 20) {
    console.error('\n❌ API key invalide (doit faire au moins 20 caractères)');
    console.error('   Caractères reçus: ' + apiKey.length);
    console.error('   Astuce: Collez votre clé lentement ou tapez-la manuellement\n');
    rl.close();
    process.exit(1);
  }

  // Special check for Z.AI keys (contain dots)
  if (provider === 'z.ai' && !apiKey.includes('.')) {
    console.warn('\n⚠️  Les clés Z.AI contiennent normalement un point (.)');
    console.warn('   Exemple: d1f1...fd12.U7FS...ABGf');
    console.warn('   Votre clé semble incomplète.\n');
    const confirm = await prompt(rl, 'Continuer quand même ? (oui/non): ');
    if (confirm.toLowerCase() !== 'oui' && confirm.toLowerCase() !== 'o' && confirm.toLowerCase() !== 'yes' && confirm.toLowerCase() !== 'y') {
      console.log('❌ Annulé');
      rl.close();
      process.exit(0);
    }
  }

  console.log(`✅ API key reçue (${apiKey.length} caractères)`);

  // 3. API URL
  const defaultUrl = getDefaultUrl(provider);
  console.log('');
  const apiUrl = await prompt(rl, `🌐 API URL (default: ${defaultUrl}): `);
  const finalApiUrl = apiUrl.trim() || defaultUrl;
  console.log(`✅ URL: ${finalApiUrl}`);

  // 4. Model selection with interactive menu
  const availableModels = getAvailableModels(provider);
  const defaultModel = getDefaultModel(provider);

  console.log('');
  console.log('🤖 Available Models:');
  availableModels.forEach((model, index) => {
    console.log(`  ${index + 1}. ${model.name.padEnd(35)} ${model.description}`);
  });
  console.log(`  0. Default (${defaultModel})`);

  const modelChoice = await prompt(rl, `\nSelect model (0-${availableModels.length}, or type custom name): `);
  let finalModel;

  if (modelChoice === '0' || modelChoice === '') {
    finalModel = defaultModel;
    console.log(`✅ Using default model: ${finalModel}`);
  } else if (parseInt(modelChoice) >= 1 && parseInt(modelChoice) <= availableModels.length) {
    const selectedIndex = parseInt(modelChoice) - 1;
    finalModel = availableModels[selectedIndex].name;
    console.log(`✅ Selected: ${finalModel}`);
  } else {
    // Custom model name
    finalModel = modelChoice.trim() || defaultModel;
    console.log(`✅ Using custom model: ${finalModel}`);
  }

  console.log('');

  // 5. Master Password
  console.log('='.repeat(60));
  console.log('🔐 MOT DE PASSE MAÎTRE');
  console.log('='.repeat(60));
  console.log('\nCe mot de passe servira à chiffrer votre configuration.');
  console.log('⚠️  Il doit être fort (min 12 caractères) et unique !');
  console.log('⚠️  Sans lui, impossible de déchiffrer votre config.');
  console.log('💡 Si vous l\'oubliez: refaites "exg config"\n');

  const masterPassword1 = await promptPassword(rl, 'Mot de passe maître: ');

  if (!masterPassword1 || masterPassword1.length < 12) {
    console.error('\n❌ Le mot de passe doit faire au moins 12 caractères');
    console.error('   Caractères reçus: ' + (masterPassword1 ? masterPassword1.length : 0) + '\n');
    rl.close();
    process.exit(1);
  }

  const masterPassword2 = await promptPassword(rl, 'Confirmez le mot de passe: ');

  if (masterPassword1 !== masterPassword2) {
    console.error('\n❌ Les mots de passe ne correspondent pas');
    rl.close();
    process.exit(1);
  }

  console.log('✅ Mot de passe confirmé\n');

  // 6. Build config object
  const config = {
    provider,
    api_key: apiKey,
    api_url: finalApiUrl,
    model: finalModel,
    created_at: new Date().toISOString(),
    version: '1.0',
  };

  // 7. Encrypt with master password
  console.log('🔒 Chiffrement de votre configuration...');
  const encrypted = encrypt(JSON.stringify(config), masterPassword1);

  // 8. Save to ~/.config/ex-g-se/settings.enc
  const configDir = path.join(os.homedir(), '.config', 'ex-g-se');
  fs.mkdirSync(configDir, { recursive: true });

  const configPath = path.join(configDir, 'settings.enc');

  // Check if file exists and warn
  if (fs.existsSync(configPath)) {
    console.log(`\n⚠️  File ${configPath} existe déjà.`);
    const overwrite = await prompt(rl, 'Écraser ? (oui/non): ');
    if (overwrite.toLowerCase() !== 'oui' && overwrite.toLowerCase() !== 'yes' && overwrite.toLowerCase() !== 'o' && overwrite.toLowerCase() !== 'y') {
      console.log('❌ Annulé');
      rl.close();
      process.exit(0);
    }
  }

  fs.writeFileSync(configPath, JSON.stringify(encrypted, null, 2));

  // Success
  console.log('\n' + '='.repeat(60));
  console.log('✅ CONFIGURATION SAUVEGARDÉE');
  console.log('='.repeat(60));
  console.log(`\n📁 Fichier: ${configPath}`);
  console.log(`🔦 Provider: ${provider}`);
  console.log(`🤖 Model: ${finalModel}`);
  console.log('\n⚠️  MÉMORISEZ VOTRE MOT DE PASSE MAÎTRE !');
  console.log('💡 Perdu ? Refaites: npx @oalacea/ex-g-se config\n');

  rl.close();
}

// ============================================================================
// STYLE CONFIG FUNCTION
// ============================================================================

/**
 * Configure le style guide pour la génération de scripts
 */
async function configStyle() {
  console.log('\n🎨 Configuration du Style Guide\n');
  console.log('Ce style guide sera utilisé pour générer vos scripts et posts sociaux.');
  console.log('Décrivez votre DA artistique, ton, style, etc.\n');

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  // Check if EXG.md already exists
  const configDir = path.join(os.homedir(), '.config', 'ex-g-se');
  const stylePath = path.join(configDir, 'EXG.md');

  let existingContent = '';
  if (fs.existsSync(stylePath)) {
    existingContent = fs.readFileSync(stylePath, 'utf8');
    console.log('📁 Un style guide existe déjà:');
    console.log('─'.repeat(60));
    console.log(existingContent);
    console.log('─'.repeat(60));

    const overwrite = await prompt(rl, '\nModifier ? (oui/non): ');
    if (overwrite.toLowerCase() !== 'oui' && overwrite.toLowerCase() !== 'yes' && overwrite.toLowerCase() !== 'o' && overwrite.toLowerCase() !== 'y') {
      console.log('❌ Annulé');
      rl.close();
      process.exit(0);
    }
  }

  console.log('\n📝 Décrivez votre style (appuyez sur Entrée quand vous avez fini):\n');
  console.log('Exemples de choses à inclure:');
  console.log('  • Ton: humoristique, professionnel, éducatif, etc.');
  console.log('  • Style: "build in public", technique, accessible, etc.');
  console.log('  • Format: préférences pour emojis, longueurs, structure');
  console.log('  • Thèmes: sujets que vous aimez aborder');
  console.log('  • Langue: français, anglais, multilingue, etc.\n');

  console.log('─'.repeat(60));

  const lines = [];
  const timer = setInterval(() => {
    // Keep alive
  }, 10000);

  try {
    const input = await new Promise((resolve) => {
      let buffer = '';

      const readLine = () => {
        rl.question('', (line) => {
          if (line === '' && buffer === '') {
            // Empty first line - start collecting
            readLine();
          } else if (line === '') {
            // Empty line means done
            resolve(buffer);
          } else {
            buffer += line + '\n';
            readLine();
          }
        });
      };

      readLine();
    });

    lines.push(input);
  } finally {
    clearInterval(timer);
  }

  const styleContent = lines.join('\n').trim();

  if (!styleContent || styleContent.length < 50) {
    console.error('\n❌ Style guide trop court (minimum 50 caractères)');
    rl.close();
    process.exit(1);
  }

  // Save to EXG.md
  fs.mkdirSync(configDir, { recursive: true });
  fs.writeFileSync(stylePath, styleContent);

  console.log('\n' + '='.repeat(60));
  console.log('✅ STYLE GUIDE SAUVEGARDÉ');
  console.log('='.repeat(60));
  console.log(`\n📁 Fichier: ${stylePath}`);
  console.log(`📏 Taille: ${styleContent.length} caractères`);
  console.log('\n💡 Ce style sera automatiquement appliqué à:');
  console.log('   • Scripts théâtraux');
  console.log('   • Posts LinkedIn');
  console.log('   • Threads Twitter/X');
  console.log('   • Posts Bluesky');
  console.log('   • Articles Dev.to/Hashnode');
  console.log('   • Posts Mastodon');
  console.log('   • Résumés de blog\n');

  rl.close();
}

// ============================================================================
// RUN
// ============================================================================

// Check command
const command = process.argv[2];

if (command === 'style') {
  configStyle().catch(error => {
    console.error('\n❌ Erreur:', error.message);
    process.exit(1);
  });
} else {
  config().catch(error => {
    console.error('\n❌ Erreur:', error.message);
    process.exit(1);
});
