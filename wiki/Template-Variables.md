# Template Variables

Master the power of dynamic prompts with EdisonPrompt's template variable system. Transform static text into reusable, configurable templates.

## 🎯 Overview

Template variables use Handlebars-style `{{variable}}` syntax to create dynamic, reusable prompts. Variables are automatically detected, can have descriptions and default values, and support interactive prompting.

## Basic Syntax

### Simple Variables
```
Hello {{name}}, welcome to {{company}}!
```

### Usage
```bash
edisonprompt get greeting --var name="Alice" --var company="Acme Corp"
# Output: Hello Alice, welcome to Acme Corp!
```

## Variable Features

### Automatic Detection
EdisonPrompt automatically extracts variables from templates:

```bash
edisonprompt add email-template --interactive
# Enter: "Dear {{recipient}}, your {{item}} order {{status}}."
# EdisonPrompt detects: recipient, item, status
```

### Interactive Prompting
Missing variables trigger interactive prompts:

```bash
edisonprompt get email-template
# Prompts: "Enter value for 'recipient':"
# Prompts: "Enter value for 'item':"
# Prompts: "Enter value for 'status':"
```

### Variable Descriptions
During template creation, provide helpful descriptions:

```
Variable 'recipient': Recipient name
Variable 'item': Product or service name  
Variable 'status': Order status (e.g., confirmed, shipped, delivered)
```

## Advanced Patterns

### Conditional Content
```
{{#if urgent}}
🚨 URGENT: {{message}}
{{else}}
ℹ️  Notice: {{message}}
{{/if}}
```

### Lists and Iteration
```
Meeting Agenda:
{{#each items}}
- {{this}}
{{/each}}
```

### Nested Objects
```
Customer: {{customer.name}}
Email: {{customer.email}}
Company: {{customer.company}}
```

## Real-World Examples

### 1. Email Templates

**Professional Email:**
```
Subject: {{subject}}

Dear {{recipient_name}},

{{greeting_message}}

{{main_content}}

{{#if action_required}}
Action Required: {{action_details}}
{{/if}}

{{closing_message}}

Best regards,
{{sender_name}}
{{sender_title}}
{{sender_company}}
```

**Usage:**
```bash
edisonprompt get professional-email \
  --var subject="Project Update" \
  --var recipient_name="John Smith" \
  --var greeting_message="I hope this email finds you well." \
  --var main_content="I wanted to provide an update on our current project status..." \
  --var action_required="true" \
  --var action_details="Please review the attached documents by Friday." \
  --var closing_message="Thank you for your continued collaboration." \
  --var sender_name="Alice Johnson" \
  --var sender_title="Project Manager" \
  --var sender_company="Tech Solutions Inc."
```

### 2. Code Documentation

**Function Documentation:**
```
/**
 * {{function_description}}
 * 
 * @param {{{param1_type}}} {{param1_name}} - {{param1_description}}
 * @param {{{param2_type}}} {{param2_name}} - {{param2_description}}
 * @returns {{{return_type}}} {{return_description}}
 * 
 * @example
 * {{example_code}}
 * 
 * @throws {{{error_type}}} {{error_description}}
 * @since {{version}}
 */
function {{function_name}}({{param1_name}}, {{param2_name}}) {
  // Implementation here
}
```

**API Endpoint Documentation:**
```
## {{method}} {{endpoint}}

{{description}}

### Parameters

{{#each parameters}}
- **{{name}}** ({{type}}) - {{description}}
{{/each}}

### Request Example
```{{request_format}}
{{request_example}}
```

### Response Example
```{{response_format}}
{{response_example}}
```

### Error Codes
{{#each errors}}
- **{{code}}**: {{message}}
{{/each}}
```

### 3. AI Prompt Engineering

**ChatGPT System Prompt:**
```
You are a {{role}} with {{experience_level}} experience in {{domain}}.

Your primary capabilities include:
{{#each capabilities}}
- {{this}}
{{/each}}

Your constraints and guidelines:
{{#each constraints}}
- {{this}}
{{/each}}

When responding to users:
1. {{response_style}}
2. {{tone_guidance}}
3. {{format_requirements}}

{{#if special_instructions}}
Special Instructions:
{{special_instructions}}
{{/if}}

Always maintain a {{personality_type}} personality while being {{communication_style}}.
```

**Claude Analysis Prompt:**
```
Please analyze the following {{content_type}}:

```
{{content}}
```

Analysis Framework:
{{#each analysis_dimensions}}
{{@index}}. **{{name}}**: {{description}}
{{/each}}

Focus Areas:
{{#each focus_areas}}
- {{area}}: {{specific_questions}}
{{/each}}

Output Format:
{{output_requirements}}

{{#if additional_context}}
Additional Context:
{{additional_context}}
{{/if}}
```

### 4. Meeting Templates

**Meeting Agenda:**
```
# {{meeting_title}}

**Date:** {{date}}
**Time:** {{start_time}} - {{end_time}}
**Location:** {{location}}
**Facilitator:** {{facilitator}}

## Attendees
{{#each attendees}}
- {{name}} ({{role}})
{{/each}}

## Agenda Items

{{#each agenda_items}}
### {{@index}}. {{title}} ({{duration}})
**Owner:** {{owner}}
**Type:** {{type}}

{{description}}

{{#if preparation_needed}}
**Preparation:** {{preparation_details}}
{{/if}}
{{/each}}

## Pre-Meeting Actions
{{#each pre_actions}}
- [ ] {{action}} ({{responsible}})
{{/each}}

## Next Meeting
**Date:** {{next_meeting_date}}
**Focus:** {{next_meeting_focus}}
```

### 5. Project Templates

**User Story Template:**
```
## User Story: {{story_title}}

**As a** {{user_type}}
**I want** {{user_goal}}
**So that** {{user_benefit}}

### Acceptance Criteria
{{#each acceptance_criteria}}
- [ ] {{this}}
{{/each}}

### Technical Notes
{{technical_requirements}}

### Definition of Done
{{#each dod_items}}
- [ ] {{this}}
{{/each}}

**Story Points:** {{story_points}}
**Priority:** {{priority}}
**Epic:** {{epic_name}}
```

## Variable Best Practices

### 1. Naming Conventions

**Good Variable Names:**
```
{{recipient_name}}     # Clear and specific
{{start_date}}         # Descriptive
{{error_message}}      # Self-documenting
{{api_endpoint_url}}   # Fully qualified
```

**Avoid:**
```
{{name}}              # Too generic
{{x}}                 # Meaningless
{{data}}              # Vague
{{temp}}              # Unclear purpose
```

### 2. Default Values

Provide sensible defaults during creation:
```
{{sender_name}} -> Default: "Your Name"
{{company}} -> Default: "Your Company"
{{date}} -> Default: "Today's Date"
```

### 3. Variable Descriptions

Write helpful descriptions:
```
{{recipient_name}} -> "Full name of the email recipient"
{{urgency_level}} -> "Priority level: low, medium, high, urgent"
{{deadline}} -> "Project deadline in YYYY-MM-DD format"
```

### 4. Consistent Patterns

Use consistent naming across templates:
```
# Email templates
{{recipient_name}}, {{sender_name}}, {{subject_line}}

# Code templates  
{{function_name}}, {{param_type}}, {{return_type}}

# Meeting templates
{{meeting_title}}, {{start_time}}, {{attendee_list}}
```

## Interactive Variable Management

### Setting Multiple Variables
```bash
# Method 1: Multiple --var flags
edisonprompt get template \
  --var name="Alice" \
  --var role="Developer" \
  --var project="WebApp"

# Method 2: Interactive prompting
edisonprompt get template
# Prompts for each missing variable
```

### Variable Validation

EdisonPrompt validates variable names:
- Must start with letter or underscore
- Can contain letters, numbers, underscores
- Case-sensitive
- No spaces or special characters

**Valid:**
```
{{user_name}}
{{_private_var}}
{{API_KEY}}
{{version2}}
```

**Invalid:**
```
{{user-name}}        # Hyphens not allowed
{{2nd_item}}         # Can't start with number
{{user name}}        # Spaces not allowed
{{user@email}}       # Special chars not allowed
```

## Advanced Features

### Variable Substitution Order
Variables are processed in template order:
```
{{greeting}} {{name}}! Today is {{date}}.
# Processes: greeting → name → date
```

### Nested Variable Support
```
{{user.name}}
{{config.database.host}}
{{items.0.title}}
```

### Escaping Variables
To include literal `{{}}` in output:
```
\{{not_a_variable}}
# Outputs: {{not_a_variable}}
```

## Performance Considerations

### Variable Extraction Speed
- Variable detection: <1ms for typical templates
- Template rendering: <2ms with 10+ variables
- Database lookup: Cached for repeated access

### Memory Usage
- Variables stored efficiently in SQLite
- Minimal memory overhead per variable
- Batch processing for multiple templates

## Troubleshooting Variables

### Common Issues

**Variable Not Found:**
```bash
edisonprompt get template --var nonexistent="value"
# Error: Variable 'nonexistent' not found in template
```

**Missing Required Variable:**
```bash
edisonprompt get template --raw  # Skip variable substitution
```

**Malformed Variable Name:**
```
# Template: "Hello {{user-name}}"
# Error: Invalid variable name 'user-name'
```

### Debug Variable Issues
```bash
# Show template with variables highlighted
edisonprompt search "{{" --highlight

# List templates with specific variables
edisonprompt search "{{name}}" --format json

# Export template for inspection
edisonprompt export --tag debug --pretty
```

## Integration Examples

### Shell Scripts
```bash
#!/bin/bash
NAME="John Doe"
ROLE="Developer"

# Generate email from template
EMAIL=$(edisonprompt get welcome-email --var name="$NAME" --var role="$ROLE")
echo "$EMAIL" | mail -s "Welcome" user@example.com
```

### CI/CD Pipelines
```yaml
- name: Generate Release Notes
  run: |
    edisonprompt get release-notes \
      --var version="${{ github.event.release.tag_name }}" \
      --var date="$(date +%Y-%m-%d)" \
      --copy
```

### Text Editors
Many editors can integrate with command-line tools:

**VS Code snippet:**
```json
{
  "Insert EdisonPrompt": {
    "prefix": "ep",
    "body": [
      "${1:$(edisonprompt get ${2:template-name} --var ${3:var}='${4:value}')}"
    ]
  }
}
```

---

## See Also

- [Command Reference](Command-Reference) - Complete command documentation
- [Configuration](Configuration) - Customizing variable behavior
- [Getting Started](Getting-Started) - Basic template creation
- [Performance & Benchmarks](Performance-Benchmarks) - Variable processing performance