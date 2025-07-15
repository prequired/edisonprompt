# Troubleshooting

Common issues, solutions, and diagnostic procedures for EdisonPrompt. This guide helps you quickly resolve problems and optimize your experience.

## 🚨 Quick Diagnostics

### Health Check Commands
```bash
# Test basic functionality
edisonprompt --version

# Verify database connectivity
edisonprompt list

# Test configuration
edisonprompt --verbose list

# Check permissions
ls -la ~/.config/edisonprompt/
ls -la ~/.local/share/edisonprompt/
```

### Common Status Indicators
| Issue | Command | Expected Result |
|-------|---------|-----------------|
| Installation | `edisonprompt --version` | `edisonprompt 0.1.0` |
| Database | `edisonprompt list` | Prompt list or empty |
| Config | `edisonprompt --help` | Help text displays |
| Permissions | `echo "test" \| edisonprompt add test --interactive` | Prompt created |

## 🔧 Installation Issues

### "Command Not Found"

**Problem:** `edisonprompt: command not found`

**Solutions:**
```bash
# Check if binary exists
ls ~/.cargo/bin/edisonprompt

# Add cargo bin to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or install to system location
sudo cp ~/.cargo/bin/edisonprompt /usr/local/bin/

# Verify installation
which edisonprompt
```

### Installation from Source Fails

**Problem:** `cargo install edisonprompt` fails

**Solutions:**
```bash
# Update Rust toolchain
rustup update stable

# Clear cargo cache
cargo clean
rm -rf ~/.cargo/registry/cache

# Install with verbose output
cargo install edisonprompt --verbose

# Check system requirements
rustc --version  # Should be 1.70+
```

### Permission Denied

**Problem:** `Permission denied` when running commands

**Solutions:**
```bash
# Make binary executable
chmod +x ~/.cargo/bin/edisonprompt

# Check file ownership
ls -la ~/.cargo/bin/edisonprompt

# Fix ownership if needed
sudo chown $USER:$USER ~/.cargo/bin/edisonprompt
```

## 💾 Database Issues

### Database Connection Failed

**Problem:** `Error: Database connection failed`

**Solutions:**
```bash
# Check database file exists
ls -la ~/.local/share/edisonprompt/prompts.db

# Verify directory permissions
chmod 755 ~/.local/share/edisonprompt/

# Create directory if missing
mkdir -p ~/.local/share/edisonprompt/

# Test with temporary database
edisonprompt --config <(echo '[database]
path = "/tmp/test.db"') list
```

### Database Corruption

**Problem:** `Error: Database file is corrupted`

**Solutions:**
```bash
# Backup existing database
cp ~/.local/share/edisonprompt/prompts.db ~/.local/share/edisonprompt/prompts.db.backup

# Try SQLite repair
sqlite3 ~/.local/share/edisonprompt/prompts.db ".recover" | sqlite3 recovered.db
mv recovered.db ~/.local/share/edisonprompt/prompts.db

# If repair fails, restore from export
if [ -f backup.json ]; then
    rm ~/.local/share/edisonprompt/prompts.db
    edisonprompt import --input backup.json
fi

# Last resort: start fresh
rm ~/.local/share/edisonprompt/prompts.db
edisonprompt list  # Creates new database
```

### Permission Denied on Database

**Problem:** `Permission denied: prompts.db`

**Solutions:**
```bash
# Check database file permissions
ls -la ~/.local/share/edisonprompt/prompts.db

# Fix permissions
chmod 644 ~/.local/share/edisonprompt/prompts.db
chmod 755 ~/.local/share/edisonprompt/

# Check if file is locked
lsof ~/.local/share/edisonprompt/prompts.db

# Change database location temporarily
export EDISONPROMPT_DATA_DIR=/tmp/edisonprompt
edisonprompt list
```

## 📝 Configuration Issues

### Config File Not Found

**Problem:** `Configuration file not found`

**Solutions:**
```bash
# Create config directory
mkdir -p ~/.config/edisonprompt/

# Create minimal config
cat > ~/.config/edisonprompt/config.toml << 'EOF'
[database]
path = "~/.local/share/edisonprompt/prompts.db"

[editor]
fallback = "nano"
EOF

# Test configuration
edisonprompt --config ~/.config/edisonprompt/config.toml list
```

### Invalid Configuration

**Problem:** `Error parsing configuration file`

**Solutions:**
```bash
# Validate TOML syntax
python3 -c "import toml; print(toml.load('~/.config/edisonprompt/config.toml'))"

# Check for common errors
grep -n '=' ~/.config/edisonprompt/config.toml
grep -n '\[' ~/.config/edisonprompt/config.toml

# Use minimal working config
cat > ~/.config/edisonprompt/config.toml << 'EOF'
[database]
path = "~/.local/share/edisonprompt/prompts.db"
EOF

# Test with verbose output
edisonprompt --verbose list
```

### Environment Variable Issues

**Problem:** Environment variables not recognized

**Solutions:**
```bash
# Check current environment
env | grep EDISONPROMPT
env | grep EDITOR

# Set variables correctly
export EDISONPROMPT_CONFIG=~/.config/edisonprompt/config.toml
export EDISONPROMPT_DATA_DIR=~/.local/share/edisonprompt
export EDITOR=nano

# Add to shell profile
echo 'export EDITOR=nano' >> ~/.bashrc
source ~/.bashrc
```

## 📋 Clipboard Issues

### Clipboard Not Working

**Problem:** `Clipboard operation failed`

**Solutions:**
```bash
# Test system clipboard
echo "test" | xclip -selection clipboard  # Linux
echo "test" | pbcopy                      # macOS

# Enable fallback mode
cat >> ~/.config/edisonprompt/config.toml << 'EOF'
[clipboard]
enable_fallback = true
EOF

# Use file I/O instead of clipboard
edisonprompt get template > output.txt
edisonprompt add new-template < input.txt --interactive
```

### Headless Environment Issues

**Problem:** Clipboard fails in SSH/Docker

**Solutions:**
```bash
# Configure fallback mode
[clipboard]
enable_fallback = true
timeout_ms = 1000

# Use file-based workflow
edisonprompt export --output templates.json
edisonprompt get template --var name="value" > output.txt

# Set up X11 forwarding for SSH
ssh -X user@server
export DISPLAY=:0
```

## ✏️ Editor Issues

### Editor Not Found

**Problem:** `Editor command not found: code`

**Solutions:**
```bash
# Check if editor exists
which code
which vim
which nano

# Set working editor
export EDITOR=nano
# Or configure in config file
[editor]
command = "nano"
fallback = "nano"

# Test editor
echo "test content" | edisonprompt add test-editor --interactive
```

### Editor Doesn't Wait

**Problem:** Editor returns immediately, changes not saved

**Solutions:**
```bash
# Use --wait flag for GUI editors
[editor]
command = "code"
args = ["--wait"]

# Alternative editors that wait by default
[editor]
command = "vim"     # Terminal editors wait automatically
# command = "nano"
# command = "emacs"

# Test editor behavior
edisonprompt edit test-prompt
```

### Editor Permission Issues

**Problem:** `Permission denied when opening editor`

**Solutions:**
```bash
# Check editor permissions
ls -la $(which code)

# Use different editor temporarily
EDITOR=nano edisonprompt edit prompt-name

# Fix PATH issues
export PATH="/usr/local/bin:/usr/bin:/bin:$PATH"
hash -r  # Refresh command cache
```

## 🔍 Search Issues

### Slow Search Performance

**Problem:** Search takes longer than expected

**Solutions:**
```bash
# Check database size
ls -lh ~/.local/share/edisonprompt/prompts.db

# Analyze slow queries
EDISONPROMPT_LOG=debug edisonprompt search "slow query"

# Optimize database
sqlite3 ~/.local/share/edisonprompt/prompts.db "VACUUM;"
sqlite3 ~/.local/share/edisonprompt/prompts.db "ANALYZE;"

# Reduce search scope
edisonprompt search "term" --limit 10
edisonprompt list --tag specific-tag
```

### Search Returns No Results

**Problem:** Expected results not found

**Solutions:**
```bash
# Check if prompts exist
edisonprompt list

# Test different search terms
edisonprompt search "{{variable}}"  # Find templates with variables
edisonprompt search "email"         # Single word
edisonprompt search "email template" # Multiple words

# Check FTS5 index
sqlite3 ~/.local/share/edisonprompt/prompts.db "SELECT count(*) FROM prompts_fts;"

# Rebuild search index
sqlite3 ~/.local/share/edisonprompt/prompts.db "INSERT INTO prompts_fts(prompts_fts) VALUES('rebuild');"
```

### Special Character Issues

**Problem:** Search fails with special characters

**Solutions:**
```bash
# Quote search terms with special characters
edisonprompt search '"hello@world.com"'
edisonprompt search "'user's template'"

# Escape special characters
edisonprompt search "user\'s template"

# Use partial matching
edisonprompt search "hello world"  # Instead of "hello@world.com"
```

## 🎨 Template Issues

### Variable Not Found

**Problem:** `Variable 'name' not found in template`

**Solutions:**
```bash
# Check template content
edisonprompt get template-name --raw

# List available variables
edisonprompt search "{{" --highlight | grep -o "{{[^}]*}}"

# Use correct variable names
edisonprompt get template --var correct_name="value"

# Skip variable substitution
edisonprompt get template --raw
```

### Template Rendering Fails

**Problem:** `Template rendering error`

**Solutions:**
```bash
# Check for malformed template syntax
edisonprompt get template --raw | grep -E "{{.*}}"

# Test with simple variables
edisonprompt add test-template --interactive
# Enter: "Hello {{name}}!"

# Validate Handlebars syntax
echo "{{name}}" | handlebars  # If handlebars CLI is available

# Escape problematic content
# Use \{{literal}} for literal braces
```

### Invalid Variable Names

**Problem:** `Invalid variable name 'user-name'`

**Solutions:**
```bash
# Use valid variable naming
{{user_name}}     # Use underscores
{{userName}}      # Use camelCase
{{USER_NAME}}     # Use uppercase

# Avoid invalid patterns
{{user-name}}     # Hyphens not allowed
{{2nd_item}}      # Can't start with number
{{user name}}     # Spaces not allowed
```

## 🌐 Network and Sync Issues

### Export/Import Failures

**Problem:** Export/import operations fail

**Solutions:**
```bash
# Test file permissions
touch /tmp/test-export.json
edisonprompt export --output /tmp/test-export.json

# Check disk space
df -h ~/.local/share/edisonprompt/

# Use different location
edisonprompt export --output ~/Documents/backup.json

# Test with small dataset
edisonprompt export --tag test --output small-export.json
```

### Sync Conflicts

**Problem:** Import creates conflicting prompts

**Solutions:**
```bash
# Use different merge strategies
edisonprompt import --input backup.json --merge skip
edisonprompt import --input backup.json --merge overwrite
edisonprompt import --input backup.json --merge rename

# Preview import changes
edisonprompt import --input backup.json --dry-run

# Manual conflict resolution
edisonprompt export --output current.json
# Compare files manually
# Import selectively
```

## 🐛 Debug Mode

### Enable Verbose Logging
```bash
# Basic verbose output
edisonprompt --verbose list

# Debug logging
EDISONPROMPT_LOG=debug edisonprompt search "test"

# Trace all operations
EDISONPROMPT_LOG=trace edisonprompt get template

# Log to file
EDISONPROMPT_LOG=debug edisonprompt list 2> debug.log
```

### Collecting Debug Information
```bash
# System information
uname -a
rustc --version
edisonprompt --version

# Configuration dump
edisonprompt --verbose list | head -20

# Database statistics
sqlite3 ~/.local/share/edisonprompt/prompts.db ".schema"
sqlite3 ~/.local/share/edisonprompt/prompts.db "SELECT count(*) FROM prompts;"

# File system information
ls -la ~/.config/edisonprompt/
ls -la ~/.local/share/edisonprompt/
df -h ~/.local/share/edisonprompt/
```

## 🔍 Performance Issues

### Slow Startup

**Problem:** EdisonPrompt takes too long to start

**Solutions:**
```bash
# Profile startup time
time edisonprompt --help

# Check for large database
ls -lh ~/.local/share/edisonprompt/prompts.db

# Optimize database
sqlite3 ~/.local/share/edisonprompt/prompts.db "VACUUM;"

# Use faster storage
# Move database to SSD if on HDD
```

### High Memory Usage

**Problem:** EdisonPrompt uses too much memory

**Solutions:**
```bash
# Monitor memory usage
ps aux | grep edisonprompt

# Check database size
ls -lh ~/.local/share/edisonprompt/prompts.db

# Reduce search limits
[search]
limit = 25

# Clear large caches
sqlite3 ~/.local/share/edisonprompt/prompts.db "PRAGMA cache_size = 100;"
```

## 🆘 Getting Help

### Before Reporting Issues

1. **Check this troubleshooting guide**
2. **Run health check commands**
3. **Collect debug information**
4. **Try minimal reproduction**

### Reporting Bugs

**Include this information:**
```bash
# System information
uname -a
edisonprompt --version
rustc --version

# Error reproduction
edisonprompt --verbose [failing-command] 2>&1

# Configuration (remove sensitive data)
cat ~/.config/edisonprompt/config.toml

# Database info (if relevant)
sqlite3 ~/.local/share/edisonprompt/prompts.db ".schema"
```

### Community Resources

- **GitHub Issues**: [Report bugs and feature requests](https://github.com/prequired/edisonprompt/issues)
- **Discussions**: [Community discussions](https://github.com/prequired/edisonprompt/discussions)
- **Wiki**: [This documentation](https://github.com/prequired/edisonprompt/wiki)

### Emergency Recovery

**Complete Reset (Last Resort):**
```bash
# Backup everything first
cp -r ~/.config/edisonprompt/ ~/edisonprompt-config-backup/
cp -r ~/.local/share/edisonprompt/ ~/edisonprompt-data-backup/

# Export prompts if possible
edisonprompt export --output ~/edisonprompt-emergency-backup.json

# Remove all EdisonPrompt data
rm -rf ~/.config/edisonprompt/
rm -rf ~/.local/share/edisonprompt/

# Reinstall
cargo install edisonprompt --force

# Import prompts back
edisonprompt import --input ~/edisonprompt-emergency-backup.json
```

---

## See Also

- [Getting Started](Getting-Started) - Basic setup and configuration
- [Configuration](Configuration) - Detailed configuration options
- [Performance & Benchmarks](Performance-Benchmarks) - Performance optimization
- [Command Reference](Command-Reference) - Complete command documentation