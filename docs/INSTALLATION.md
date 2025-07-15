# EdisonPrompt Installation Guide

Multiple installation methods for getting EdisonPrompt up and running on your system.

## 📦 Quick Installation

### From Source (Recommended)
```bash
# Clone the repository
git clone https://github.com/prequired/edisonprompt.git
cd edisonprompt

# Build and install
cargo build --release
cargo install --path .

# Verify installation
edisonprompt --version
```

### Using Cargo
```bash
# Install directly from crates.io (when published)
cargo install edisonprompt

# Verify installation
edisonprompt --version
```

## 🖥️ Platform-Specific Instructions

### Linux
```bash
# Ubuntu/Debian - Install Rust first
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build EdisonPrompt
git clone https://github.com/prequired/edisonprompt.git
cd edisonprompt
cargo build --release

# Install to system
sudo cp target/release/edisonprompt /usr/local/bin/
```

### macOS
```bash
# Install Rust via Homebrew
brew install rust

# Or use rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install EdisonPrompt
git clone https://github.com/prequired/edisonprompt.git
cd edisonprompt
cargo build --release
cp target/release/edisonprompt /usr/local/bin/
```

### Windows
```powershell
# Install Rust via rustup
# Download from https://rustup.rs/ and run rustup-init.exe

# Build EdisonPrompt
git clone https://github.com/prequired/edisonprompt.git
cd edisonprompt
cargo build --release

# Add to PATH or copy to desired location
copy target\release\edisonprompt.exe C:\Users\%USERNAME%\bin\
```

## 🐳 Docker Installation

### Using Docker
```bash
# Create Dockerfile
cat > Dockerfile << 'EOF'
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/edisonprompt /usr/local/bin/edisonprompt
ENTRYPOINT ["edisonprompt"]
EOF

# Build image
docker build -t edisonprompt .

# Run with volume for data persistence
docker run -v ~/.local/share/edisonprompt:/root/.local/share/edisonprompt edisonprompt --help
```

### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'
services:
  edisonprompt:
    build: .
    volumes:
      - ~/.local/share/edisonprompt:/root/.local/share/edisonprompt
      - ~/.config/edisonprompt:/root/.config/edisonprompt
    environment:
      - EDITOR=nano
```

## 🔧 Post-Installation Setup

### Shell Completions
```bash
# Bash
edisonprompt completions bash > ~/.bash_completion.d/edisonprompt
source ~/.bash_completion.d/edisonprompt

# Zsh
mkdir -p ~/.zsh/completions
edisonprompt completions zsh > ~/.zsh/completions/_edisonprompt
# Add to ~/.zshrc: fpath=(~/.zsh/completions $fpath)

# Fish
edisonprompt completions fish > ~/.config/fish/completions/edisonprompt.fish

# PowerShell
edisonprompt completions powershell > edisonprompt.ps1
# Import in PowerShell profile
```

### Configuration
```bash
# Create default config
mkdir -p ~/.config/edisonprompt
edisonprompt --help  # This creates default config on first run

# Customize configuration
cat > ~/.config/edisonprompt/config.toml << 'EOF'
[database]
path = "~/.local/share/edisonprompt/prompts.db"
timeout_ms = 5000

[editor]
fallback = "nano"  # or "vim", "code", etc.

[search]
limit = 50
highlight = true

[output]
color = true
format = "table"
EOF
```

### Environment Variables
```bash
# Add to ~/.bashrc, ~/.zshrc, etc.
export EDITOR=code                    # Your preferred editor
export EDISONPROMPT_CONFIG=~/.config/edisonprompt/config.toml
export EDISONPROMPT_DATA_DIR=~/.local/share/edisonprompt
```

## 🏗️ Development Setup

### Prerequisites
- Rust 1.70 or later
- Git
- A text editor or IDE

### Development Installation
```bash
# Clone repository
git clone https://github.com/prequired/edisonprompt.git
cd edisonprompt

# Install development dependencies
cargo install cargo-watch cargo-tarpaulin

# Run in development mode
cargo run -- --help

# Run tests
cargo test

# Run with hot reloading
cargo watch -x 'run -- list'

# Check code quality
cargo clippy -- -D warnings
cargo fmt
```

### IDE Setup

#### VS Code
```bash
# Install Rust extension
code --install-extension rust-lang.rust-analyzer

# Recommended settings.json
mkdir -p .vscode
cat > .vscode/settings.json << 'EOF'
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.buildScripts.enable": true,
    "editor.formatOnSave": true
}
EOF
```

#### Vim/Neovim
```bash
# Install rust.vim and coc-rust-analyzer
# Add to .vimrc/.config/nvim/init.vim
```

## 🚀 Performance Optimization

### Binary Size Optimization
```bash
# Build with size optimizations
cargo build --release

# Strip debug symbols (Linux/macOS)
strip target/release/edisonprompt

# Use UPX compression (optional)
upx --best target/release/edisonprompt
```

### Runtime Performance
```bash
# Set optimal environment variables
export RUST_LOG=error  # Reduce logging overhead
export MALLOC_ARENA_MAX=2  # Reduce memory fragmentation

# Use performance governor (Linux)
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

## 📊 Verification

### Test Installation
```bash
# Basic functionality test
edisonprompt --version
edisonprompt --help

# Create test prompt
echo "Test prompt with {{variable}}" | edisonprompt add test --interactive

# Test template rendering
edisonprompt get test --var variable=value

# Test search
edisonprompt search "test"

# Clean up
edisonprompt delete test --force
```

### Performance Benchmarks
```bash
# Startup time (should be <100ms)
time edisonprompt --help

# Search performance (should be <50ms)
time edisonprompt search "test"

# Memory usage check
valgrind --tool=massif target/release/edisonprompt list
```

## 🐛 Troubleshooting

### Common Issues

#### "command not found: edisonprompt"
```bash
# Check if cargo bin is in PATH
echo $PATH | grep -q ~/.cargo/bin || echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc

# Or install to system location
sudo cp target/release/edisonprompt /usr/local/bin/
```

#### "permission denied" on Linux/macOS
```bash
# Make binary executable
chmod +x target/release/edisonprompt

# Or run with sudo for system installation
sudo cp target/release/edisonprompt /usr/local/bin/
sudo chmod +x /usr/local/bin/edisonprompt
```

#### Database errors
```bash
# Check permissions on data directory
ls -la ~/.local/share/edisonprompt/

# Reset database if corrupted
rm ~/.local/share/edisonprompt/prompts.db
edisonprompt list  # Creates new database
```

#### Editor not working
```bash
# Set EDITOR environment variable
export EDITOR=nano  # or vim, code, etc.

# Or configure in config file
echo 'fallback = "nano"' >> ~/.config/edisonprompt/config.toml
```

#### Clipboard not working in headless environments
```bash
# Enable fallback mode
echo 'enable_fallback = true' >> ~/.config/edisonprompt/config.toml

# Or use file input/output
echo "content" | edisonprompt add test --interactive
```

### Debug Mode
```bash
# Enable verbose logging
RUST_LOG=debug edisonprompt --verbose list

# Trace all operations
RUST_LOG=trace edisonprompt search "test"
```

### Getting Help
```bash
# Built-in help
edisonprompt --help
edisonprompt add --help

# Version information
edisonprompt --version

# Configuration check
edisonprompt list --verbose
```

## 🔄 Updates

### Updating EdisonPrompt
```bash
# From source
cd edisonprompt
git pull origin main
cargo build --release
cargo install --path .

# From crates.io (when available)
cargo install edisonprompt --force
```

### Backup Before Updates
```bash
# Export all prompts
edisonprompt export --output backup-$(date +%Y%m%d).json --pretty

# Save configuration
cp ~/.config/edisonprompt/config.toml config-backup.toml
```

## 📝 Next Steps

After installation:

1. **Set up shell completions** for better UX
2. **Configure your preferred editor** in the config
3. **Create your first templates** following the examples
4. **Set up regular backups** with export functionality
5. **Explore advanced features** in the examples documentation

For more detailed usage examples, see [EXAMPLES.md](EXAMPLES.md).

For development and contributing, see [CONTRIBUTING.md](CONTRIBUTING.md).