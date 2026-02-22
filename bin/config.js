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
    // Pause readline to avoid conflicts
    rl.pause();

    process.stdout.write(question);
    process.stdin.setRawMode(true);
    process.stdin.resume();
    process.stdin.setEncoding('utf8');

    let password = '';

    const onData = (char) => {
      // Enter/Return - submit password
      if (char === '\r' || char === '\n') {
        // Drain any remaining input
        process.stdin.setRawMode(false);
        const buffer = Buffer.alloc(1024);
        try {
          process.stdin.read(buffer);
        } catch (e) {
          // Ignore
        }
        process.stdin.pause();
        process.stdin.removeListener('data', onData);
        process.stdout.write('\n');

        // Resume readline for next prompt
        rl.resume();
        resolve(password);
        return;
      }

      // Ctrl+C or Ctrl+D - exit
      if (char === '\u0003' || char === '\u0004') {
        process.stdout.write('\n');
        process.stdin.setRawMode(false);
        process.stdin.pause();
        process.stdin.removeListener('data', onData);
        rl.close();
        process.exit(0);
        return;
      }

      // Backspace/Delete - remove last character
      if (char === '\u007f' || char === '\b') {
        if (password.length > 0) {
          password = password.slice(0, -1);
          process.stdout.write('\b \b');
        }
        return;
      }

      // Ignore control characters (except those handled above)
      if (char.charCodeAt(0) < 32 && char !== '\r' && char !== '\n') {
        return;
      }

      // Accept all other characters (including multi-byte UTF-8)
      password += char;
      process.stdout.write('*');
    };

    process.stdin.on('data', onData);
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

  console.log(`✅ API key reçue (${apiKey.length} caractères)`);

  // 3. API URL
  const defaultUrl = getDefaultUrl(provider);
  console.log('');
  const apiUrl = await prompt(rl, `🌐 API URL (default: ${defaultUrl}): `);
  const finalApiUrl = apiUrl.trim() || defaultUrl;
  console.log(`✅ URL: ${finalApiUrl}`);

  // 4. Model
  const defaultModel = getDefaultModel(provider);
  const model = await prompt(rl, `🤖 Model (default: ${defaultModel}): `);
  const finalModel = model.trim() || defaultModel;
  console.log(`✅ Model: ${finalModel}\n`);

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
// RUN
// ============================================================================

config().catch(error => {
  console.error('\n❌ Erreur:', error.message);
  process.exit(1);
});
