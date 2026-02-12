# Provider Documentation Generator

This script automates the creation of provider documentation files for AISDK.

## Usage

### Interactive Mode

From the `apps/docs` directory:

```bash
pnpm gen:provider
```

The script will interactively prompt you for the following information:

1. **Provider name** (e.g., `Cohere`, `Anthropic`, `Google`)
2. **Provider lowercase** (e.g., `cohere`) - auto-suggested
3. **Environment variable name** (e.g., `COHERE_API_KEY`) - auto-suggested
4. **Example model method** (e.g., `command_r_plus`)
5. **Model string** (e.g., `command-r-plus`) - auto-suggested
6. **Model type example** (e.g., `CommandRPlus`)
7. **Default base URL** (e.g., `https://api.cohere.ai`)
8. **Output filename** (e.g., `cohere.mdx`) - auto-suggested

### Non-Interactive Mode (Batch Generation)

For batch generation or CI/CD pipelines, you can pass all options as command line arguments:

```bash
pnpm gen:provider --provider-name "Groq" --provider-lowercase "groq" --env-var-name "GROQ_API_KEY" --example-model-method "llama_3_1_8b_instruct" --example-model-string "llama-3.1-8b-instruct" --model-type-example "MetaLlamaLlama318bInstructV10" --default-base-url "https://api.groq.com/openai/" --output-filename "groq.mdx"
```

**Available Arguments:**

| Short | Long | Description | Required |
|-------|------|-------------|----------|
| `-n` | `--provider-name` | Provider name (e.g., "Groq") | Yes |
| `-l` | `--provider-lowercase` | Lowercase provider name | No (derived from name) |
| `-e` | `--env-var-name` | Environment variable name | No (derived from name) |
| `-m` | `--example-model-method` | Example model method (snake_case) | Yes |
| `-s` | `--example-model-string` | Model string (kebab-case) | No (derived from method) |
| `-t` | `--model-type-example` | Model type example (PascalCase) | Yes |
| `-b` | `--default-base-url` | Default base URL | Yes |
| `-o` | `--output-filename` | Output filename | No (derived from lowercase) |

**Example with short arguments:**

```bash
pnpm gen:provider -n "DeepSeek" -l "deepseek" -e "DEEPSEEK_API_KEY" -m "deepseek_chat" -s "deepseek-chat" -t "DeepseekChat" -b "https://api.deepseek.com/" -o "deepseek.mdx"
```

## Features

- **Auto-suggestions**: The script intelligently suggests values based on your input
- **Confirmation prompts**: Press Enter to accept suggestions or type a custom value
- **Overwrite protection**: Warns if a file already exists before overwriting
- **Template-based**: Uses a consistent template to ensure documentation uniformity

## Example Session

```
🚀 Provider Documentation Generator

This will create a new provider documentation file.
Press Enter to accept suggested values.

Provider name (e.g., Anthropic, Google, OpenAI): Cohere
Provider lowercase [cohere]: 
Environment variable name [COHERE_API_KEY]: 
Example model method (e.g., gpt_5, claude_3_5_sonnet): command_r_plus
Model string (kebab-case) [command-r-plus]: 
Model type example (PascalCase, e.g., Gpt5, Claude35Sonnet): CommandRPlus
Default base URL (e.g., https://api.openai.com): https://api.cohere.ai
Output filename [cohere.mdx]: 

✅ Successfully created: /path/to/content/docs/providers/cohere.mdx

Next steps:
  1. Review the generated file
  2. Customize if needed
  3. Add entry to meta.json if required
```

## Post-Generation Steps

After generating a new provider documentation file, you **must** manually add it to two locations:

### 1. Add to Sidebar Navigation

Edit `apps/docs/content/docs/providers/meta.json` and add your provider to the `pages` array in **alphabetical order**:

```json
{
  "title": "Providers",
  "icon": "Workflow",
  "pages": ["index", "antropic", "cohere", "google"]
}
```

### 2. Add to Get Started Page

Edit `apps/docs/content/docs/(get-started)/index.mdx` and add a link in the **Model Providers** section in **alphabetical order**:

```mdx
## Model Providers

AISDK supports a wide range of AI providers, including:

- <Link href="/docs/providers/anthropic">Anthropic</Link>
- <Link href="/docs/providers/cohere">Cohere</Link>
- <Link href="/docs/providers/google">Google</Link>
- <Link href="/docs/providers">OpenAI</Link>
- more to come
```

> **Note**: Both files require entries to be in **alphabetical order** for consistency.

## Files

- `scripts/generate-provider-doc.ts` - The generator script
- `scripts/templates/provider.mdx.template` - The MDX template with placeholders
- `scripts/README.md` - This documentation file

## Customization

To modify the template, edit `scripts/templates/provider.mdx.template`. The following placeholders are available:

- `{{PROVIDER_NAME}}` - Provider name (e.g., "Cohere")
- `{{PROVIDER_LOWERCASE}}` - Lowercase provider name (e.g., "cohere")
- `{{ENV_VAR_NAME}}` - Environment variable name (e.g., "COHERE_API_KEY")
- `{{EXAMPLE_MODEL_METHOD}}` - Example model method (e.g., "command_r_plus")
- `{{EXAMPLE_MODEL_STRING}}` - Example model string (e.g., "command-r-plus")
- `{{MODEL_TYPE_EXAMPLE}}` - Model type example (e.g., "CommandRPlus")
- `{{DEFAULT_BASE_URL}}` - Default base URL (e.g., "https://api.cohere.ai")
