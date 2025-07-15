# Command Reference

Complete documentation for all EdisonPrompt commands, flags, and options.

## Global Options

These options are available for all commands:

| Flag | Description |
|------|-------------|
| `-v, --verbose` | Enable verbose output |
| `-c, --config <FILE>` | Specify custom config file path |
| `--no-color` | Disable colored output |
| `-h, --help` | Show help information |
| `-V, --version` | Show version information |

## Commands Overview

| Command | Purpose | Key Features |
|---------|---------|--------------|
| [`add`](#add) | Add new prompts | Clipboard input, interactive mode, tagging |
| [`get`](#get) | Retrieve prompts | Variable substitution, clipboard output |
| [`list`](#list) | List prompts | Filtering, sorting, multiple formats |
| [`search`](#search) | Search prompts | Full-text search, highlighting |
| [`edit`](#edit) | Edit prompts | Editor integration, auto-save |
| [`delete`](#delete) | Delete prompts | Safe removal, confirmation |
| [`export`](#export) | Export prompts | JSON format, selective export |
| [`import`](#import) | Import prompts | Merge strategies, validation |
| [`completions`](#completions) | Generate shell completions | All major shells |

---

## add

Add a new prompt from clipboard or interactive input.

### Syntax
```bash
edisonprompt add <NAME> [OPTIONS]
```

### Arguments
- `<NAME>` - Prompt name (alphanumeric, hyphens, underscores only)

### Options
| Flag | Description |
|------|-------------|
| `-i, --interactive` | Use interactive mode for content input |
| `-t, --tags <TAGS>` | Add comma-separated tags |
| `-f, --force` | Overwrite existing prompt |

### Examples

**Add from clipboard:**
```bash
# Copy content to clipboard first, then:
edisonprompt add email-template
```

**Interactive mode:**
```bash
edisonprompt add code-review --interactive
# Prompts for content input
```

**With tags:**
```bash
edisonprompt add meeting-notes --tags business,templates --interactive
```

**Force overwrite:**
```bash
edisonprompt add existing-prompt --force --interactive
```

### Variable Detection
EdisonPrompt automatically detects `{{variable}}` patterns and prompts for descriptions during interactive creation.

---

## get

Retrieve and render a prompt with variable substitution.

### Syntax
```bash
edisonprompt get <NAME> [OPTIONS]
```

### Arguments
- `<NAME>` - Name of prompt to retrieve

### Options
| Flag | Description |
|------|-------------|
| `--var <KEY=VALUE>` | Set variable values (can be used multiple times) |
| `-c, --copy` | Copy result to clipboard |
| `-r, --raw` | Output raw content without rendering variables |

### Examples

**Basic retrieval:**
```bash
edisonprompt get welcome-email
```

**With variables:**
```bash
edisonprompt get welcome-email --var name="John" --var company="Acme"
```

**Copy to clipboard:**
```bash
edisonprompt get api-docs --var endpoint="/users" --var method="GET" --copy
```

**Raw content (no variable substitution):**
```bash
edisonprompt get template-source --raw
```

### Interactive Variables
If variables are missing, EdisonPrompt will prompt for values interactively.

---

## list

List prompts with optional filtering and formatting.

### Syntax
```bash
edisonprompt list [OPTIONS]
```

### Options
| Flag | Description |
|------|-------------|
| `-t, --tag <TAG>` | Filter by tag |
| `-f, --format <FORMAT>` | Output format: `table`, `json`, `plain` |
| `-l, --limit <NUM>` | Limit number of results |
| `-s, --sort <FIELD>` | Sort by: `name`, `created`, `updated` |
| `--names-only` | Show only prompt names |

### Examples

**Default table view:**
```bash
edisonprompt list
```

**Filter by tag:**
```bash
edisonprompt list --tag email
```

**JSON output:**
```bash
edisonprompt list --format json
```

**Limited results:**
```bash
edisonprompt list --limit 10 --sort updated
```

**Names only (useful for scripts):**
```bash
edisonprompt list --names-only
```

### Output Formats

**Table Format (Default):**
```
┌─────────────────┬───────────┬─────────────┬─────────────────────┐
│ Name            │ Variables │ Tags        │ Updated             │
├─────────────────┼───────────┼─────────────┼─────────────────────┤
│ email-template  │ 3         │ email, work │ 2025-07-15 10:30:00 │
│ code-review     │ 2         │ code, dev   │ 2025-07-15 09:15:00 │
└─────────────────┴───────────┴─────────────┴─────────────────────┘
```

**JSON Format:**
```json
[
  {
    "id": "uuid-here",
    "name": "email-template",
    "variables": ["name", "subject", "content"],
    "tags": ["email", "work"],
    "created_at": "2025-07-15T10:30:00Z",
    "updated_at": "2025-07-15T10:30:00Z"
  }
]
```

---

## search

Search prompts using full-text search with highlighting.

### Syntax
```bash
edisonprompt search <QUERY> [OPTIONS]
```

### Arguments
- `<QUERY>` - Search term or phrase

### Options
| Flag | Description |
|------|-------------|
| `--highlight` | Highlight search terms in results |
| `-l, --limit <NUM>` | Limit number of results (default: 50) |
| `-f, --format <FORMAT>` | Output format: `table`, `json`, `plain` |

### Examples

**Basic search:**
```bash
edisonprompt search "email template"
```

**With highlighting:**
```bash
edisonprompt search "{{name}}" --highlight
```

**Limited results:**
```bash
edisonprompt search "documentation" --limit 5
```

**JSON output:**
```bash
edisonprompt search "API" --format json
```

### Search Features
- **Full-text search** across prompt names and content
- **FTS5-powered** for instant results (~7ms)
- **Phrase matching** with quotes: `"exact phrase"`
- **Variable search** to find templates with specific variables
- **Highlighting** shows matched terms with `<mark>` tags

---

## edit

Edit an existing prompt in your preferred editor.

### Syntax
```bash
edisonprompt edit <NAME> [OPTIONS]
```

### Arguments
- `<NAME>` - Name of prompt to edit

### Options
| Flag | Description |
|------|-------------|
| `-y, --yes` | Skip confirmation prompt |

### Examples

**Edit with confirmation:**
```bash
edisonprompt edit email-template
```

**Skip confirmation:**
```bash
edisonprompt edit code-review --yes
```

### Editor Selection
EdisonPrompt uses editors in this priority:
1. `$EDITOR` environment variable
2. Configured editor in `config.toml`
3. Fallback editor (default: `nano`)

### Supported Editors
- **nano**, **vim**, **emacs** - Terminal editors
- **code** (VS Code), **subl** (Sublime Text) - GUI editors
- **gedit**, **kate** - Desktop editors

---

## delete

Delete a prompt permanently with safety confirmations.

### Syntax
```bash
edisonprompt delete <NAME> [OPTIONS]
```

### Arguments
- `<NAME>` - Name of prompt to delete

### Options
| Flag | Description |
|------|-------------|
| `-y, --yes` | Skip confirmation prompt |
| `-f, --force` | Force delete without any confirmation |

### Examples

**Delete with confirmation:**
```bash
edisonprompt delete old-template
# Prompts: "Are you sure you want to delete 'old-template'? (y/N)"
```

**Skip confirmation:**
```bash
edisonprompt delete test-prompt --yes
```

**Force delete:**
```bash
edisonprompt delete temp-prompt --force
```

### Safety Features
- Confirmation prompts by default
- Shows prompt details before deletion
- Cannot be undone (use export for backups)

---

## export

Export prompts to JSON format for backup or sharing.

### Syntax
```bash
edisonprompt export [OPTIONS]
```

### Options
| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Output file path (stdout if not specified) |
| `-t, --tag <TAG>` | Export only prompts with specific tag |
| `-p, --pretty` | Pretty-print JSON output |

### Examples

**Export all prompts:**
```bash
edisonprompt export --output backup.json --pretty
```

**Export to stdout:**
```bash
edisonprompt export --pretty
```

**Export by tag:**
```bash
edisonprompt export --tag work --output work-prompts.json
```

**Pipe to other tools:**
```bash
edisonprompt export | jq '.[] | .name'
```

### Export Format
```json
{
  "schema_version": "1.0",
  "exported_at": "2025-07-15T10:30:00Z",
  "prompts": [
    {
      "id": "uuid-here",
      "name": "email-template",
      "content": "Hello {{name}}...",
      "variables": [
        {
          "name": "name",
          "description": "Recipient name",
          "default_value": null
        }
      ],
      "tags": ["email", "business"],
      "created_at": "2025-07-15T10:30:00Z",
      "updated_at": "2025-07-15T10:30:00Z"
    }
  ]
}
```

---

## import

Import prompts from JSON format with conflict resolution.

### Syntax
```bash
edisonprompt import [OPTIONS]
```

### Options
| Flag | Description |
|------|-------------|
| `-i, --input <FILE>` | Input file path (stdin if not specified) |
| `-m, --merge <STRATEGY>` | Merge strategy: `skip`, `overwrite`, `rename` |
| `--dry-run` | Show what would be imported without making changes |

### Examples

**Import from file:**
```bash
edisonprompt import --input backup.json --merge skip
```

**Import from stdin:**
```bash
cat shared-prompts.json | edisonprompt import --merge overwrite
```

**Dry run (preview):**
```bash
edisonprompt import --input new-prompts.json --dry-run
```

### Merge Strategies

| Strategy | Behavior |
|----------|----------|
| `skip` | Skip prompts that already exist (default) |
| `overwrite` | Replace existing prompts with imported versions |
| `rename` | Import with modified names (e.g., `prompt-name-2`) |

### Import Validation
- Schema version compatibility checking
- Prompt name validation
- Variable format validation
- Detailed conflict reporting

---

## completions

Generate shell completion scripts for enhanced productivity.

### Syntax
```bash
edisonprompt completions <SHELL>
```

### Arguments
- `<SHELL>` - Shell type: `bash`, `zsh`, `fish`, `powershell`

### Examples

**Generate and install completions:**

**Bash:**
```bash
edisonprompt completions bash > ~/.bash_completion.d/edisonprompt
source ~/.bash_completion.d/edisonprompt
```

**Zsh:**
```bash
mkdir -p ~/.zsh/completions
edisonprompt completions zsh > ~/.zsh/completions/_edisonprompt
# Add to ~/.zshrc: fpath=(~/.zsh/completions $fpath)
```

**Fish:**
```bash
edisonprompt completions fish > ~/.config/fish/completions/edisonprompt.fish
```

**PowerShell:**
```bash
edisonprompt completions powershell > edisonprompt.ps1
# Import in PowerShell profile
```

### Completion Features
- **Command completion** - All subcommands and flags
- **Prompt name completion** - Available prompt names for `get`, `edit`, `delete`
- **Tag completion** - Existing tags for filtering
- **File path completion** - For import/export operations

---

## Exit Codes

EdisonPrompt uses standard exit codes for script integration:

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Command parse error |
| `3` | Database error |
| `4` | Template error |
| `5` | IO error |
| `6` | Prompt not found |
| `7` | Configuration error |

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `EDITOR` | Preferred text editor | System default |
| `EDISONPROMPT_CONFIG` | Custom config file path | `~/.config/edisonprompt/config.toml` |
| `EDISONPROMPT_DATA_DIR` | Custom data directory | `~/.local/share/edisonprompt` |
| `NO_COLOR` | Disable colored output | - |

## Performance Notes

- **Startup time**: ~4ms (includes config loading and database connection)
- **Search time**: ~7ms for 1000+ prompts (SQLite FTS5)
- **Memory usage**: <20MB typical, <50MB with large datasets
- **Database size**: ~1KB per prompt (varies with content length)

---

## See Also

- [Template Variables](Template-Variables) - Advanced variable usage
- [Configuration](Configuration) - Customization options
- [Performance & Benchmarks](Performance-Benchmarks) - Technical details
- [Troubleshooting](Troubleshooting) - Common issues and solutions