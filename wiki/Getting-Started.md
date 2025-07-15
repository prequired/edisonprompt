# Getting Started with EdisonPrompt

Welcome to EdisonPrompt! This guide will get you up and running in minutes with the world's fastest AI prompt management CLI.

## 🚀 Installation

### Quick Install (Recommended)
```bash
cargo install edisonprompt
```

### Verify Installation
```bash
edisonprompt --version
# Should output: edisonprompt 0.1.0
```

### First Run Setup
```bash
# This automatically creates config directories and database
edisonprompt --help
```

## 📝 Your First Prompt

Let's create your first prompt template:

### 1. Add a Simple Prompt
```bash
edisonprompt add hello-world --interactive
```

When prompted, enter:
```
Hello {{name}}!

Welcome to EdisonPrompt. This is your first template with a variable.

Hope you enjoy the {{speed}} performance!
```

### 2. Use Your Prompt
```bash
edisonprompt get hello-world --var name="Alice" --var speed="lightning-fast"
```

**Output:**
```
Hello Alice!

Welcome to EdisonPrompt. This is your first template with a variable.

Hope you enjoy the lightning-fast performance!
```

### 3. Copy to Clipboard
```bash
edisonprompt get hello-world --var name="Bob" --var speed="incredible" --copy
```

## 🎯 Essential Workflows

### Workflow 1: Email Templates

**Create an email template:**
```bash
edisonprompt add client-email --interactive --tags email,business
```

**Template content:**
```
Subject: {{subject}}

Dear {{client_name}},

{{greeting_message}}

{{main_content}}

{{closing_message}}

Best regards,
{{your_name}}
{{your_title}}
```

**Use the template:**
```bash
edisonprompt get client-email \
  --var subject="Project Update" \
  --var client_name="John" \
  --var greeting_message="I hope this email finds you well." \
  --var main_content="I wanted to provide an update on the current project status..." \
  --var closing_message="Please let me know if you have any questions." \
  --var your_name="Alice" \
  --var your_title="Project Manager" \
  --copy
```

### Workflow 2: Code Documentation

**Create a function doc template:**
```bash
edisonprompt add function-doc --interactive --tags code,docs
```

**Template content:**
```
/**
 * {{function_description}}
 * 
 * @param {{{param_type}}} {{param_name}} - {{param_description}}
 * @returns {{{return_type}}} {{return_description}}
 * 
 * @example
 * {{example_usage}}
 */
```

**Use it:**
```bash
edisonprompt get function-doc \
  --var function_description="Calculates the total price including tax" \
  --var param_type="number" \
  --var param_name="basePrice" \
  --var param_description="The base price before tax" \
  --var return_type="number" \
  --var return_description="The total price including tax" \
  --var example_usage="calculateTotal(100) // returns 108"
```

### Workflow 3: AI Prompt Engineering

**Create a ChatGPT system prompt:**
```bash
edisonprompt add chatgpt-system --interactive --tags ai,chatgpt
```

**Template content:**
```
You are a {{role}} with expertise in {{domain}}.

Your capabilities include:
{{capabilities}}

Your constraints:
{{constraints}}

When responding:
- {{response_style}}
- {{tone_instruction}}
- {{format_requirement}}

Always maintain {{personality}} personality.
```

## 🔍 Exploring Your Prompts

### List All Prompts
```bash
edisonprompt list
```

### Search by Content
```bash
edisonprompt search "email" --highlight
```

### Filter by Tags
```bash
edisonprompt list --tag email
```

### Different Output Formats
```bash
# Table format (default)
edisonprompt list --format table

# JSON format
edisonprompt list --format json

# Plain text (names only)
edisonprompt list --names-only
```

## ⚡ Performance Features

### Lightning-Fast Search
```bash
# Search across thousands of prompts in ~7ms
edisonprompt search "{{variable}}" --highlight --limit 10
```

### Instant Startup
```bash
# Measures startup performance
time edisonprompt --help
# Typical result: ~4ms
```

### Efficient Memory Usage
EdisonPrompt uses <20MB of memory even with thousands of prompts.

## 🛠️ Shell Integration

### Enable Completions

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

### With Completions Enabled
```bash
edisonprompt get <TAB>  # Shows available prompt names
edisonprompt list --<TAB>  # Shows available flags
```

## 📊 Organizing Your Prompts

### Use Descriptive Names
```bash
# Good naming
edisonprompt add client-follow-up-email
edisonprompt add api-error-documentation
edisonprompt add standup-meeting-template

# Less descriptive
edisonprompt add email1
edisonprompt add docs
edisonprompt add template
```

### Tag Strategically
```bash
# Multiple tags for better organization
edisonprompt add bug-report --tags code,github,templates
edisonprompt add sales-email --tags email,business,external
edisonprompt add ai-prompt --tags ai,chatgpt,creative
```

### Use Consistent Variable Names
Common variable patterns:
- `{{name}}`, `{{company}}`, `{{date}}`
- `{{subject}}`, `{{recipient}}`, `{{sender}}`
- `{{title}}`, `{{description}}`, `{{content}}`

## 🔄 Backup and Sync

### Export Your Prompts
```bash
# Export all prompts
edisonprompt export --output my-prompts.json --pretty

# Export specific tags
edisonprompt export --tag work --output work-prompts.json
```

### Import Prompts
```bash
# Import with different merge strategies
edisonprompt import --input shared-prompts.json --merge skip
edisonprompt import --input updated-prompts.json --merge overwrite
```

## 🎯 Next Steps

Now that you're up and running:

1. **Explore Advanced Features**: Check out [Template Variables](Template-Variables) for advanced templating
2. **Customize Your Setup**: Review [Configuration](Configuration) for personalization options
3. **Learn All Commands**: Browse [Command Reference](Command-Reference) for complete documentation
4. **Optimize Performance**: Read [Performance & Benchmarks](Performance-Benchmarks) for technical details

## 💡 Pro Tips

### Productivity Hacks
```bash
# Set up shell aliases for common commands
echo 'alias pa="edisonprompt add"' >> ~/.bashrc
echo 'alias pg="edisonprompt get"' >> ~/.bashrc
echo 'alias ps="edisonprompt search"' >> ~/.bashrc
```

### Template Best Practices
- Use descriptive variable names
- Provide default values when possible
- Include examples in template descriptions
- Group related templates with consistent tags

### Performance Optimization
- Use specific search terms for faster results
- Leverage tag filtering instead of content search when possible
- Keep prompt names concise but descriptive

## ❓ Need Help?

- **Common Issues**: Check [Troubleshooting](Troubleshooting)
- **Advanced Usage**: See [Template Variables](Template-Variables)
- **Configuration**: Review [Configuration](Configuration)
- **Bug Reports**: Open an issue on [GitHub](https://github.com/prequired/edisonprompt/issues)

---

**You're ready to supercharge your prompt management workflow!** 🚀

Continue to [Command Reference](Command-Reference) for detailed command documentation.