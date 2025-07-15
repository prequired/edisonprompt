# Shell Completions Guide

EdisonPrompt provides comprehensive shell completions for all major shells, enabling fast command-line navigation with tab completion support.

## 🚀 Quick Setup

### Bash
```bash
# Generate and install bash completions
edisonprompt completions bash > ~/.bash_completion.d/edisonprompt
source ~/.bash_completion.d/edisonprompt

# Or add to ~/.bashrc for automatic loading
echo 'source ~/.bash_completion.d/edisonprompt' >> ~/.bashrc
```

### Zsh
```bash
# Create completions directory
mkdir -p ~/.zsh/completions

# Generate and install zsh completions
edisonprompt completions zsh > ~/.zsh/completions/_edisonprompt

# Add completions directory to fpath in ~/.zshrc
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc

# Reload shell
source ~/.zshrc
```

### Fish
```bash
# Generate and install fish completions
edisonprompt completions fish > ~/.config/fish/completions/edisonprompt.fish

# Fish will automatically load completions on next shell start
```

### PowerShell
```powershell
# Generate PowerShell completions
edisonprompt completions powershell > edisonprompt_completions.ps1

# Add to PowerShell profile
Add-Content $PROFILE '. .\edisonprompt_completions.ps1'

# Or append directly to profile
edisonprompt completions powershell >> $PROFILE

# Reload profile
. $PROFILE
```

## 🔧 Advanced Installation

### System-wide Installation (Linux/macOS)

#### Bash (System-wide)
```bash
# Install for all users
sudo edisonprompt completions bash > /etc/bash_completion.d/edisonprompt

# Or for pkg-config locations
sudo edisonprompt completions bash > /usr/share/bash-completion/completions/edisonprompt
```

#### Zsh (System-wide)
```bash
# Install to zsh site-functions
sudo edisonprompt completions zsh > /usr/share/zsh/site-functions/_edisonprompt

# Or for Oh My Zsh custom completions
edisonprompt completions zsh > ~/.oh-my-zsh/custom/completions/_edisonprompt
```

#### Fish (System-wide)
```bash
# Install for all users
sudo edisonprompt completions fish > /usr/share/fish/completions/edisonprompt.fish
```

### Package Managers

#### Homebrew (macOS)
```bash
# If installed via Homebrew, completions are often handled automatically
# Manual setup if needed:
edisonprompt completions bash > $(brew --prefix)/etc/bash_completion.d/edisonprompt
edisonprompt completions zsh > $(brew --prefix)/share/zsh/site-functions/_edisonprompt
edisonprompt completions fish > $(brew --prefix)/share/fish/completions/edisonprompt.fish
```

## 📋 What Gets Completed

### Commands
All primary commands with descriptions:
- `add` - Add a new prompt from clipboard or input
- `get` - Retrieve and render a prompt with variables  
- `list` - List prompts with optional filtering
- `search` - Search prompts by content using full-text search
- `edit` - Edit an existing prompt in your editor
- `delete` - Delete a prompt permanently
- `export` - Export prompts to JSON format
- `import` - Import prompts from JSON format
- `completions` - Generate shell completions

### Options and Flags
All command-line options with intelligent completion:
- Global options: `--verbose`, `--config`, `--no-color`, `--help`, `--version`
- Command-specific options: `--interactive`, `--force`, `--copy`, `--raw`, etc.
- File paths for options like `--config`, `--output`, `--input`

### Value Completion
Smart completion for enumerated values:
- **Output formats**: `table`, `json`, `plain`
- **Sort fields**: `name`, `created`, `updated`  
- **Merge strategies**: `skip`, `overwrite`, `rename`
- **Shell types**: `bash`, `zsh`, `fish`, `powershell`

### Context-Aware Suggestions
- File path completion for `--config`, `--output`, `--input` options
- Smart defaults based on command context
- Proper handling of quoted arguments and special characters

## 🎯 Usage Examples

### Tab Completion in Action
```bash
# Type partial command and press TAB
$ edisonprompt a<TAB>
add

# Complete options
$ edisonprompt add --<TAB>
--force        --help         --interactive  --tags         --verbose

# Complete enum values
$ edisonprompt list --format <TAB>
json   plain  table

# Complete file paths
$ edisonprompt export --output <TAB>
backup.json    exports/       prompts.json

# Complete shell types
$ edisonprompt completions <TAB>
bash        fish        powershell  zsh
```

### Help Integration
Completions include help text for commands and options:
```bash
$ edisonprompt <TAB><TAB>
add          -- Add a new prompt from clipboard or input
completions  -- Generate shell completions  
delete       -- Delete a prompt permanently
edit         -- Edit an existing prompt in your editor
export       -- Export prompts to JSON format
get          -- Retrieve and render a prompt with variables
help         -- Print this message or the help of the given subcommand(s)
import       -- Import prompts from JSON format
list         -- List prompts with optional filtering
search       -- Search prompts by content using full-text search
```

## 🔍 Troubleshooting

### Completions Not Working

#### Check Installation
```bash
# Verify completion files exist
ls -la ~/.bash_completion.d/edisonprompt        # Bash
ls -la ~/.zsh/completions/_edisonprompt         # Zsh  
ls -la ~/.config/fish/completions/edisonprompt.fish  # Fish

# Check if completions are loaded
complete -p edisonprompt  # Bash
which _edisonprompt       # Zsh
complete -C edisonprompt  # Fish
```

#### Reload Shell Configuration
```bash
# Bash
source ~/.bashrc
# or
source ~/.bash_completion.d/edisonprompt

# Zsh
source ~/.zshrc
# or
autoload -U compinit && compinit

# Fish
# Restart fish or run:
fish_update_completions

# PowerShell
. $PROFILE
```

### Permission Issues
```bash
# Make completion files readable
chmod 644 ~/.bash_completion.d/edisonprompt
chmod 644 ~/.zsh/completions/_edisonprompt
chmod 644 ~/.config/fish/completions/edisonprompt.fish

# Check directory permissions
ls -la ~/.bash_completion.d/
ls -la ~/.zsh/completions/
ls -la ~/.config/fish/completions/
```

### Shell-Specific Issues

#### Bash
```bash
# Check if bash-completion is installed
command -v bash-completion

# Ubuntu/Debian
sudo apt install bash-completion

# CentOS/RHEL
sudo yum install bash-completion
```

#### Zsh
```bash
# Ensure compinit is called
grep -q "autoload.*compinit" ~/.zshrc || echo 'autoload -U compinit && compinit' >> ~/.zshrc

# Check fpath
echo $fpath | grep -q ~/.zsh/completions || echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
```

#### Fish
```bash
# Check fish version (completions require fish 2.3+)
fish --version

# Manually load completions
fish -c "complete -e edisonprompt; source ~/.config/fish/completions/edisonprompt.fish"
```

#### PowerShell
```powershell
# Check execution policy
Get-ExecutionPolicy

# Set policy if needed
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Verify profile location
echo $PROFILE

# Test completion loading
. .\edisonprompt_completions.ps1
```

### Debugging
```bash
# Test completion generation
edisonprompt completions bash | head -20

# Verify completion file syntax
bash -n ~/.bash_completion.d/edisonprompt          # Bash
zsh -n ~/.zsh/completions/_edisonprompt             # Zsh
fish -n ~/.config/fish/completions/edisonprompt.fish  # Fish

# Enable verbose completion debugging
set -x  # Bash/Zsh
set -x  # Fish
```

## 🔄 Updates and Maintenance

### Regenerating Completions
When EdisonPrompt is updated, regenerate completions to get new commands/options:

```bash
# Regenerate all shell completions
edisonprompt completions bash > ~/.bash_completion.d/edisonprompt
edisonprompt completions zsh > ~/.zsh/completions/_edisonprompt  
edisonprompt completions fish > ~/.config/fish/completions/edisonprompt.fish
edisonprompt completions powershell > edisonprompt_completions.ps1

# Reload shell configuration
source ~/.bashrc          # Bash
source ~/.zshrc           # Zsh
# Restart Fish
. $PROFILE                # PowerShell
```

### Automation Script
Create a script to automatically update completions:

```bash
#!/bin/bash
# update-completions.sh

set -e

echo "Updating EdisonPrompt shell completions..."

# Create directories if they don't exist
mkdir -p ~/.bash_completion.d
mkdir -p ~/.zsh/completions
mkdir -p ~/.config/fish/completions

# Generate completions
edisonprompt completions bash > ~/.bash_completion.d/edisonprompt
edisonprompt completions zsh > ~/.zsh/completions/_edisonprompt
edisonprompt completions fish > ~/.config/fish/completions/edisonprompt.fish

echo "Shell completions updated successfully!"
echo "Restart your shell or source your configuration files to load changes."
```

## 🌟 Advanced Features

### Custom Completion Functions
For advanced users, the generated completions can be customized:

#### Bash Custom Functions
```bash
# Add to ~/.bashrc after sourcing completions
_edisonprompt_custom() {
    # Add custom completion logic here
    local cur="${COMP_WORDS[COMP_CWORD]}"
    
    # Example: complete prompt names for get/edit/delete commands
    if [[ "${COMP_WORDS[1]}" =~ ^(get|edit|delete)$ ]]; then
        local prompts=$(edisonprompt list --names-only 2>/dev/null)
        COMPREPLY=($(compgen -W "$prompts" -- "$cur"))
        return 0
    fi
    
    # Fall back to default completion
    _edisonprompt
}

# Override default completion
complete -F _edisonprompt_custom edisonprompt
```

### Integration with Shell Frameworks

#### Oh My Zsh
```bash
# Create custom plugin
mkdir -p ~/.oh-my-zsh/custom/plugins/edisonprompt
echo 'edisonprompt completions zsh > ${0:h}/_edisonprompt' > ~/.oh-my-zsh/custom/plugins/edisonprompt/edisonprompt.plugin.zsh

# Add to plugins in ~/.zshrc
plugins=(... edisonprompt)
```

#### Fish with Fisher
```bash
# Create completion function
function __edisonprompt_install_completions
    edisonprompt completions fish > ~/.config/fish/completions/edisonprompt.fish
    echo "EdisonPrompt completions installed"
end

# Install via Fisher
fisher install edisonprompt
```

## 📚 Related Documentation

- [Installation Guide](INSTALLATION.md) - Complete installation instructions
- [Examples](EXAMPLES.md) - Usage examples and workflows  
- [Configuration](../edisonprompt.wiki/Configuration.md) - Configuration options
- [Command Reference](../edisonprompt.wiki/Command-Reference.md) - Complete command documentation

## 💡 Tips and Best Practices

1. **Keep completions updated** when installing new versions of EdisonPrompt
2. **Use descriptive naming** for prompts to make completion more effective
3. **Learn keyboard shortcuts** specific to your shell for faster completion navigation
4. **Test completions** in a new shell session after installation
5. **Backup completion configurations** before making system-wide changes

Shell completions transform EdisonPrompt from a powerful CLI tool into an intuitive, fast, and efficient prompt management experience. Once set up, you'll find yourself navigating commands and options with unprecedented speed and accuracy.