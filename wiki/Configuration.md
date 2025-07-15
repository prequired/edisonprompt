# Configuration

Customize EdisonPrompt to match your workflow with comprehensive configuration options for database, editor, clipboard, search, and output settings.

## 📍 Configuration File Location

EdisonPrompt follows XDG Base Directory specification:

| Platform | Default Location |
|----------|------------------|
| **Linux** | `~/.config/edisonprompt/config.toml` |
| **macOS** | `~/Library/Application Support/edisonprompt/config.toml` |
| **Windows** | `%APPDATA%\edisonprompt\config.toml` |

### Custom Config Location
```bash
# Use custom config file
edisonprompt --config /path/to/custom/config.toml

# Set via environment variable
export EDISONPROMPT_CONFIG=~/.my-custom-config.toml
edisonprompt list
```

## 🔧 Complete Configuration Reference

### Default Configuration
```toml
[database]
path = "~/.local/share/edisonprompt/prompts.db"
timeout_ms = 5000
wal_mode = true

[editor]
# Uses $EDITOR environment variable if not set
command = ""  
fallback = "nano"
args = []

[clipboard]
timeout_ms = 5000
enable_fallback = true

[search]
limit = 50
highlight = true
ranking = "bm25"

[output]
color = true
format = "table"

[output.table_widths]
name = 30
variables = 20
tags = 20

[template]
strict_variables = false
syntax = "handlebars"
```

## Database Configuration

### Database Path
```toml
[database]
# Supports ~ expansion
path = "~/.local/share/edisonprompt/prompts.db"

# Alternative locations
# path = "/tmp/edisonprompt.db"                    # Temporary storage
# path = "~/Dropbox/edisonprompt/prompts.db"       # Cloud sync
# path = "./project-prompts.db"                    # Project-specific
```

### Database Performance
```toml
[database]
# Connection timeout in milliseconds
timeout_ms = 5000

# Enable WAL mode for better performance
wal_mode = true

# Additional SQLite optimizations
pragma_cache_size = 1000
pragma_temp_store = "memory"
```

### Environment Variables
```bash
# Override database location
export EDISONPROMPT_DATA_DIR=~/custom/data/dir
```

## Editor Configuration

### Editor Selection Priority
1. `editor.command` in config file
2. `$EDITOR` environment variable  
3. `editor.fallback` in config file
4. System default (`nano`)

### Editor Examples
```toml
[editor]
# VS Code
command = "code"
args = ["--wait"]

# Vim with specific options
command = "vim"
args = ["+set number", "+set syntax=markdown"]

# Sublime Text
command = "subl"
args = ["--wait"]

# Nano with line numbers
fallback = "nano"
args = ["-l"]
```

### GUI Editors
```toml
[editor]
# These require --wait flag to block until editor closes
command = "code"        # VS Code
# command = "subl"      # Sublime Text  
# command = "atom"      # Atom
# command = "gedit"     # GNOME Text Editor
```

### Terminal Editors
```toml
[editor]
# No --wait needed for terminal editors
command = "vim"         # Vim
# command = "nvim"      # Neovim
# command = "emacs"     # Emacs
# command = "nano"      # Nano
# command = "micro"     # Micro
```

## Clipboard Configuration

### Timeout Settings
```toml
[clipboard]
# Clipboard operation timeout
timeout_ms = 5000

# Enable fallback to file I/O in headless environments
enable_fallback = true
```

### Headless Environment Support
When running in headless environments (SSH, containers):

```toml
[clipboard]
enable_fallback = true  # Uses temporary files instead
```

Fallback behavior:
- Writes to `$TMPDIR/edisonprompt_clipboard.txt`
- Reads from same file for subsequent operations
- Automatically cleans up on exit

## Search Configuration

### Search Defaults
```toml
[search]
# Default result limit
limit = 50

# Enable highlighting by default
highlight = true

# FTS5 ranking algorithm
ranking = "bm25"  # Options: bm25, rank
```

### Advanced Search Settings
```toml
[search]
# Custom FTS5 configuration
fts_tokenizer = "unicode61"
fts_remove_diacritics = true

# Performance tuning
cache_search_results = true
max_cached_results = 1000
```

## Output Configuration

### Colors and Formatting
```toml
[output]
# Enable colored output
color = true

# Default output format for list command
format = "table"  # Options: table, json, plain

# Force colors even when piping
force_color = false
```

### Table Customization
```toml
[output.table_widths]
name = 30        # Prompt name column width
variables = 20   # Variables column width  
tags = 20        # Tags column width
content = 40     # Content preview width
```

### Color Scheme
```toml
[output.colors]
header = "bright_blue"
prompt_name = "green"
variables = "yellow"
tags = "cyan"
dates = "dim"
error = "red"
warning = "yellow"
success = "green"
```

## Template Configuration

### Variable Handling
```toml
[template]
# Error on undefined variables (strict mode)
strict_variables = false

# Template syntax (currently only handlebars)
syntax = "handlebars"

# Variable naming validation
allow_underscore_prefix = true
max_variable_name_length = 50
```

### Template Defaults
```toml
[template.defaults]
# Default values for common variables
author = "Your Name"
company = "Your Company"
email = "your.email@company.com"
date_format = "%Y-%m-%d"
```

## Environment Variables

Override configuration with environment variables:

| Variable | Purpose | Example |
|----------|---------|---------|
| `EDISONPROMPT_CONFIG` | Config file path | `~/.config/edisonprompt/config.toml` |
| `EDISONPROMPT_DATA_DIR` | Data directory | `~/.local/share/edisonprompt` |
| `EDITOR` | Preferred editor | `code`, `vim`, `nano` |
| `NO_COLOR` | Disable colors | `1` (any value) |
| `EDISONPROMPT_LOG` | Log level | `debug`, `info`, `warn`, `error` |

## Platform-Specific Configuration

### Linux
```toml
# ~/.config/edisonprompt/config.toml
[database]
path = "~/.local/share/edisonprompt/prompts.db"

[editor]
fallback = "nano"  # Usually available

[clipboard]
enable_fallback = true  # For SSH sessions
```

### macOS
```toml
# ~/Library/Application Support/edisonprompt/config.toml
[database]
path = "~/Library/Application Support/edisonprompt/prompts.db"

[editor]
fallback = "nano"  # Pre-installed

# Optional: Use pbcopy/pbpaste integration
[clipboard]
use_system_clipboard = true
```

### Windows
```toml
# %APPDATA%\edisonprompt\config.toml
[database]
path = "%APPDATA%\\edisonprompt\\prompts.db"

[editor]
fallback = "notepad"

[clipboard]
# Windows has reliable clipboard access
enable_fallback = false
```

## Configuration Examples

### Minimal Configuration
```toml
# Only override what you need
[editor]
command = "code"
args = ["--wait"]

[search]
limit = 100
```

### Developer Configuration
```toml
[database]
path = "~/dev/edisonprompt/prompts.db"

[editor]
command = "code"
args = ["--wait", "--new-window"]

[search]
limit = 100
highlight = true

[output]
format = "table"
color = true

[template]
strict_variables = true
```

### Team/Shared Configuration
```toml
[database]
# Shared network location
path = "/shared/team/edisonprompt/prompts.db"

[template.defaults]
company = "Acme Corporation"
support_email = "support@acme.com"

[output.table_widths]
name = 40      # Longer names for detailed templates
tags = 30      # More tag space
```

### Performance-Optimized Configuration
```toml
[database]
timeout_ms = 2000      # Faster timeout
wal_mode = true        # Better concurrency
pragma_cache_size = 2000

[search]
limit = 25             # Fewer results
cache_search_results = true

[output]
format = "plain"       # Faster rendering
color = false          # Minimal formatting
```

## Configuration Validation

### Check Current Configuration
```bash
# View effective configuration
edisonprompt list --verbose

# Test configuration
edisonprompt --config /path/to/config.toml --help
```

### Common Configuration Errors
```toml
# ❌ Invalid TOML syntax
[database
path = "~/prompts.db"  # Missing closing bracket

# ❌ Invalid path
[database]
path = "invalid/absolute/path"

# ❌ Invalid timeout
[clipboard]
timeout_ms = "5000"    # Should be number, not string

# ❌ Invalid color setting
[output]
color = "true"         # Should be boolean, not string
```

### Validation Messages
```bash
# Config validation output
$ edisonprompt list
Error: Invalid configuration at database.timeout_ms
Expected: positive integer
Found: string "5000"
```

## Configuration Migration

### Upgrading Configuration
When upgrading EdisonPrompt, configuration files are automatically migrated:

```bash
# Backup current config before upgrade
cp ~/.config/edisonprompt/config.toml ~/.config/edisonprompt/config.toml.backup

# After upgrade, EdisonPrompt will migrate automatically
edisonprompt list  # Triggers migration if needed
```

### Migration Log
```
Config migration from v0.1.0 to v0.2.0:
✓ Added new field: template.strict_variables = false
✓ Deprecated field: old_setting (using new_setting instead)
✓ Migration complete
```

## Troubleshooting Configuration

### Debug Configuration Issues
```bash
# Enable verbose output
edisonprompt --verbose list

# Check configuration parsing
EDISONPROMPT_LOG=debug edisonprompt list

# Test with minimal config
echo '[database]
path = "/tmp/test.db"' > /tmp/test-config.toml
edisonprompt --config /tmp/test-config.toml list
```

### Reset to Defaults
```bash
# Backup current config
mv ~/.config/edisonprompt/config.toml ~/.config/edisonprompt/config.toml.backup

# EdisonPrompt will create default config on next run
edisonprompt --help
```

### Common Issues

**Config file not found:**
```bash
# Create config directory
mkdir -p ~/.config/edisonprompt

# EdisonPrompt will create default config
edisonprompt list
```

**Permission denied:**
```bash
# Check permissions
ls -la ~/.config/edisonprompt/
chmod 644 ~/.config/edisonprompt/config.toml
```

**Invalid TOML syntax:**
```bash
# Validate TOML syntax
python3 -c "import toml; toml.load('~/.config/edisonprompt/config.toml')"
```

## Security Considerations

### File Permissions
```bash
# Secure config file permissions
chmod 600 ~/.config/edisonprompt/config.toml

# Secure data directory
chmod 700 ~/.local/share/edisonprompt/
```

### Sensitive Data
```toml
# ❌ Don't put secrets in config
[template.defaults]
api_key = "secret-key-here"  # BAD

# ✅ Use environment variables instead
[template.defaults]
api_key = "${API_KEY}"       # GOOD
```

### Network Paths
```toml
# Be cautious with network database paths
[database]
# path = "smb://server/share/prompts.db"  # Consider security implications
```

---

## See Also

- [Getting Started](Getting-Started) - Initial setup and configuration
- [Command Reference](Command-Reference) - Configuration-related commands
- [Troubleshooting](Troubleshooting) - Configuration problem solutions
- [Performance & Benchmarks](Performance-Benchmarks) - Performance-related settings