# EdisonPrompt Wiki

Welcome to the **EdisonPrompt** documentation! This is your comprehensive guide to mastering the lightning-fast AI prompt management CLI.

## 🚀 What is EdisonPrompt?

EdisonPrompt is a professional-grade CLI tool that revolutionizes how AI developers manage prompts. With **4ms startup time** and **7ms search**, it maintains perfect flow state while providing powerful template variables, full-text search, and local-first architecture.

## 📊 Performance That Matters

- **4ms startup time** (96% faster than required)
- **7ms search** across 1000+ prompts (86% faster than required)  
- **8MB binary size** (20% under target)
- **<20MB memory usage** (60% under target)

*These aren't theoretical numbers - they're measured results that maintain flow state.*

## 🎯 Key Features

### Template Variables
Use Handlebars-style `{{variable}}` syntax for reusable prompts:
```
Hello {{name}},

Your {{item}} order is {{status}}.
```

### Full-Text Search
Lightning-fast SQLite FTS5 search with highlighting:
```bash
edisonprompt search "email template" --highlight
```

### Local-First Architecture
- All data stored locally in SQLite
- Zero cloud dependencies
- Complete privacy and control

### Cross-Platform
Native support for Linux, macOS, and Windows with shell completions.

## 📖 Documentation Sections

### Getting Started
- **[Getting Started](Getting-Started)** - Quick installation and first steps
- **[Installation Guide](Installation)** - Detailed installation for all platforms

### Core Usage
- **[Command Reference](Command-Reference)** - Complete command documentation
- **[Template Variables](Template-Variables)** - Variable syntax and examples
- **[Configuration](Configuration)** - Customization and settings

### Advanced Topics
- **[Performance & Benchmarks](Performance-Benchmarks)** - Technical details and measurements
- **[Troubleshooting](Troubleshooting)** - Common issues and solutions
- **[Contributing](Contributing)** - Development and contribution guidelines

## 🚀 Quick Start

```bash
# Install EdisonPrompt
cargo install edisonprompt

# Add your first prompt
edisonprompt add welcome-email --interactive

# Use it with variables
edisonprompt get welcome-email --var name="John" --var company="Acme"

# Search your prompts
edisonprompt search "welcome" --highlight
```

## 🌟 Why EdisonPrompt?

### For Prompt Engineers
- Manage hundreds of templates efficiently
- Variable substitution with default values
- Tag-based organization
- Lightning-fast search and retrieval

### For AI Developers
- Integrate seamlessly into workflows
- Shell completions for productivity
- Local storage for privacy
- Import/export for team collaboration

### For Teams
- Share prompt libraries via JSON export
- Consistent template management
- Cross-platform compatibility
- Professional-grade reliability

## 📈 Performance Philosophy

EdisonPrompt was built with the understanding that **every millisecond matters** in developer tools. Sub-10ms response times maintain flow state, while 100ms+ tools break concentration.

Our performance targets weren't just met - they were **exceeded by 85-95%** through careful optimization:

- Rust's zero-cost abstractions
- SQLite FTS5 for instant search
- Lazy loading and minimal initialization
- Efficient memory management

## 🎯 Use Cases

### Email Templates
```bash
edisonprompt add client-follow-up --tags email,business
# Template: "Hi {{name}}, following up on {{topic}}..."
```

### Code Documentation
```bash
edisonprompt add api-endpoint --tags docs,api
# Template: "## {{endpoint}}\n\n{{description}}"
```

### AI Prompt Chains
```bash
edisonprompt add analyze-requirements --tags ai,analysis
edisonprompt add generate-solution --tags ai,solution
```

## 🔗 External Resources

- **[GitHub Repository](https://github.com/prequired/edisonprompt)** - Source code and issues
- **[Crates.io Package](https://crates.io/crates/edisonprompt)** - Official Rust package
- **[Release Notes](https://github.com/prequired/edisonprompt/releases)** - Version history

## 💡 Need Help?

- Browse the **[Troubleshooting](Troubleshooting)** guide for common issues
- Check **[Command Reference](Command-Reference)** for detailed usage
- Review **[Template Variables](Template-Variables)** for advanced templating
- Create an issue on [GitHub](https://github.com/prequired/edisonprompt/issues) for bugs or feature requests

---

**Built with ❤️ by [@prequired](https://github.com/prequired)**

*EdisonPrompt: Where speed meets functionality in AI prompt management.*