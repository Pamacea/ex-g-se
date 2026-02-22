# EX-G-SE Config Command

Interactive CLI configuration tool for AI provider setup.

## Usage

```bash
# Run the config command
npx @oalacea/ex-g-se-config

# Or after installation
ex-g-se-config
```

## Configuration Flow

The config command will guide you through:

1. **Provider Selection** - Choose from OpenAI, Anthropic, z.ai, or Custom
2. **API Key** - Enter your provider's API key (masked input)
3. **API URL** - Optional, with smart defaults for each provider
4. **Model Name** - Optional, with smart defaults for each provider
5. **Max Tokens** - Default: 4096
6. **Temperature** - Default: 0.7 (range: 0.0-1.0)

## Config File Location

Configuration is saved to: `~/.ex-g-se/config.json`

## Config Structure

```json
{
  "provider": "openai",
  "apiKey": "sk-...",
  "apiUrl": "https://api.openai.com/v1",
  "model": "gpt-4o",
  "maxTokens": 4096,
  "temperature": 0.7
}
```

## Provider Defaults

### OpenAI
- URL: `https://api.openai.com/v1`
- Model: `gpt-4o`

### Anthropic
- URL: `https://api.anthropic.com`
- Model: `claude-3-5-sonnet-20241022`

### z.ai
- URL: `https://api.z.ai/v1`
- Model: `zai-1`

### Custom
- URL: (required)
- Model: (required)

## Features

- ✅ Interactive prompts with defaults
- ✅ Existing config detection with overwrite prompt
- ✅ Configuration validation
- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Colored output for better UX
- ✅ Error handling and validation

## Example Session

```
🔧 EX-G-SE Configuration

? Which AI provider do you want to use?
  ❯ 1. OpenAI (GPT-4, GPT-4o)
    2. Anthropic (Claude Opus, Claude Sonnet)
    3. z.ai
    4. Custom URL

  Enter choice (1-4): 1
? Enter API key: sk-...
? Enter API URL (optional, default: https://api.openai.com/v1):
? Model name (optional, default: gpt-4o):
? Max tokens (default: 4096):
? Temperature 0.0-1.0 (default: 0.7):

✅ Configuration saved successfully!
   Location: C:\Users\YourName\.ex-g-se\config.json
```
