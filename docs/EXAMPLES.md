# EdisonPrompt Examples

This guide showcases real-world use cases and powerful workflows with EdisonPrompt.

## 🚀 Quick Start Examples

### Basic Workflow
```bash
# Add a prompt from clipboard
edisonprompt add email-template

# Add interactively with metadata
edisonprompt add code-review --interactive --tags work,development

# Get prompt with variables
edisonprompt get email-template --var recipient=team --var urgency=high --copy

# Search and explore
edisonprompt search "email" --highlight --limit 5
edisonprompt list --tag work --format table
```

## 📧 Email Templates

### Professional Email Template
```bash
edisonprompt add professional-email --interactive
```
**Content:**
```
Subject: {{subject}}

Dear {{recipient}},

{{greeting}}

{{main_content}}

{{closing}}

Best regards,
{{sender_name}}
{{sender_title}}
```

**Usage:**
```bash
edisonprompt get professional-email \
  --var subject="Project Update" \
  --var recipient="Team" \
  --var greeting="I hope this email finds you well." \
  --var main_content="I wanted to provide an update on our current project status..." \
  --var closing="Please let me know if you have any questions." \
  --var sender_name="John Doe" \
  --var sender_title="Project Manager"
```

### Follow-up Email
```bash
echo "Hi {{name}},

Just following up on our conversation about {{topic}}. 

{{next_steps}}

Let me know when you'd like to {{action}}.

Thanks!
{{your_name}}" | edisonprompt add follow-up --interactive --tags email,business
```

## 💻 Code Templates

### Pull Request Template
```bash
edisonprompt add pr-template --interactive --tags code,github
```
**Content:**
```
## Summary
{{summary}}

## Changes
{{changes}}

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests passing
- [ ] Manual testing completed

## Breaking Changes
{{breaking_changes}}

## Related Issues
Closes #{{issue_number}}
```

### Code Review Template
```bash
edisonprompt add code-review --interactive --tags code,review
```
**Content:**
```
## Code Review for {{author}}

### Positive Feedback
{{positive_points}}

### Suggestions for Improvement
{{suggestions}}

### Questions
{{questions}}

### Overall Assessment
{{assessment}}

**Approval Status:** {{status}}
```

## 📝 Documentation Templates

### API Documentation
```bash
edisonprompt add api-docs --interactive --tags docs,api
```
**Content:**
```
# {{endpoint_name}} API

## Overview
{{description}}

## Endpoint
`{{method}} {{endpoint_url}}`

## Parameters
{{parameters}}

## Request Example
```{{request_format}}
{{request_example}}
```

## Response Example
```{{response_format}}
{{response_example}}
```

## Error Codes
{{error_codes}}
```

### Blog Post Outline
```bash
edisonprompt add blog-outline --interactive --tags writing,blog
```
**Content:**
```
# {{title}}

## Hook
{{hook}}

## Introduction
{{introduction}}

## Main Points
1. {{point_1}}
2. {{point_2}}
3. {{point_3}}

## Supporting Details
{{details}}

## Conclusion
{{conclusion}}

## Call to Action
{{cta}}

**Target Audience:** {{audience}}
**Word Count:** {{word_count}}
**SEO Keywords:** {{keywords}}
```

## 🤖 AI Prompt Engineering

### ChatGPT System Prompt
```bash
edisonprompt add gpt-system --interactive --tags ai,chatgpt
```
**Content:**
```
You are {{role_description}}.

Your capabilities include:
{{capabilities}}

Your constraints are:
{{constraints}}

When responding:
{{response_guidelines}}

Examples of your work:
{{examples}}

Always maintain {{tone}} tone and {{style}} style.
```

### Claude Analysis Prompt
```bash
edisonprompt add claude-analysis --interactive --tags ai,claude
```
**Content:**
```
Please analyze the following {{content_type}}:

{{content}}

Focus on:
1. {{focus_area_1}}
2. {{focus_area_2}}
3. {{focus_area_3}}

Provide insights on:
- {{insight_area_1}}
- {{insight_area_2}}
- {{insight_area_3}}

Format your response as {{output_format}}.
```

## 📊 Business Templates

### Meeting Agenda
```bash
edisonprompt add meeting-agenda --interactive --tags business,meetings
```
**Content:**
```
# {{meeting_title}}

**Date:** {{date}}
**Time:** {{time}}
**Attendees:** {{attendees}}

## Agenda Items

1. **{{item_1}}** ({{duration_1}})
   - {{details_1}}

2. **{{item_2}}** ({{duration_2}})
   - {{details_2}}

3. **{{item_3}}** ({{duration_3}})
   - {{details_3}}

## Action Items
{{action_items}}

## Next Meeting
{{next_meeting}}
```

### Project Status Report
```bash
edisonprompt add status-report --interactive --tags business,project
```
**Content:**
```
# {{project_name}} Status Report

**Reporting Period:** {{period}}
**Project Manager:** {{pm_name}}

## Executive Summary
{{executive_summary}}

## Accomplishments
{{accomplishments}}

## Challenges
{{challenges}}

## Upcoming Milestones
{{milestones}}

## Budget Status
{{budget_status}}

## Risk Assessment
{{risks}}

## Next Steps
{{next_steps}}
```

## 🔧 Advanced Workflows

### Multi-Template Project Setup
```bash
# Create related templates for a complete workflow
edisonprompt add ticket-template --tags workflow,jira
edisonprompt add standup-update --tags workflow,scrum
edisonprompt add retrospective --tags workflow,agile

# Use them together
edisonprompt get ticket-template --var title="User Authentication" --copy
edisonprompt get standup-update --var yesterday="Worked on login UI" --var today="Implementing OAuth"
```

### Template Versioning
```bash
# Export current templates
edisonprompt export --output templates-v1.0.json --pretty

# Make changes and export again
edisonprompt export --output templates-v1.1.json --pretty

# Import specific versions when needed
edisonprompt import --input templates-v1.0.json --merge rename
```

### Team Collaboration
```bash
# Export team templates
edisonprompt export --tag team --output team-templates.json --pretty

# Share with team members
# Each team member imports:
edisonprompt import --input team-templates.json --merge skip

# Search across all team templates
edisonprompt search "standup" --format json
```

## 🎯 Pro Tips

### Efficient Variable Management
```bash
# Use descriptive variable names
edisonprompt add email-template --interactive
# Variables: recipient_name, email_type, urgency_level, call_to_action

# Set up default values for common variables
# During interactive creation, provide defaults like:
# - sender_name: "Your Name"
# - company: "Your Company"
# - signature: "Best regards"
```

### Organization Strategies
```bash
# Use consistent tagging
edisonprompt add template --tags category,subcategory,purpose

# Examples:
--tags email,business,external
--tags code,review,security
--tags docs,api,public
--tags ai,chatgpt,creative
```

### Search & Discovery
```bash
# Search by content
edisonprompt search "authentication" --highlight

# Filter by tags
edisonprompt list --tag ai --format table

# Find templates with specific variables
edisonprompt search "{{recipient}}" --highlight
```

### Backup & Sync
```bash
# Regular backups
edisonprompt export --output "backup-$(date +%Y-%m-%d).json" --pretty

# Sync across machines
edisonprompt export --output ~/Dropbox/edisonprompt-sync.json --pretty
# On other machine:
edisonprompt import --input ~/Dropbox/edisonprompt-sync.json --merge overwrite
```

## 🌟 Power User Examples

### Dynamic Content Generation
```bash
# Blog post with multiple sections
edisonprompt add blog-post --interactive --tags content,blog
# Use variables for each section: intro, main_point_1, main_point_2, conclusion

# Email campaign series
edisonprompt add email-day-1 --tags email,campaign
edisonprompt add email-day-3 --tags email,campaign
edisonprompt add email-day-7 --tags email,campaign
```

### Development Workflows
```bash
# Complete feature development flow
edisonprompt add feature-spec --tags dev,planning
edisonprompt add implementation-plan --tags dev,planning
edisonprompt add test-plan --tags dev,testing
edisonprompt add deployment-checklist --tags dev,deploy

# Use together:
edisonprompt get feature-spec --var feature="User Profile" --copy
edisonprompt get implementation-plan --var feature="User Profile" --var timeline="2 weeks"
```

### AI Prompt Chains
```bash
# Multi-step AI workflow
edisonprompt add analyze-requirements --tags ai,analysis
edisonprompt add generate-solution --tags ai,solution
edisonprompt add review-output --tags ai,review

# Execute in sequence with different AI models
edisonprompt get analyze-requirements --var input="user needs" --copy
# (paste to Claude)
edisonprompt get generate-solution --var analysis="..." --copy
# (paste to GPT-4)
edisonprompt get review-output --var solution="..." --copy
# (final review)
```

## 📈 Measuring Success

### Performance Tracking
```bash
# Time your workflows
time edisonprompt get complex-template --var var1=value1 --var var2=value2

# Monitor template usage
edisonprompt list --format json | jq '.[] | .name' | sort | uniq -c | sort -nr
```

### Template Analytics
```bash
# Find most used templates by searching recent exports
edisonprompt export --output analytics.json
# Analyze the JSON for usage patterns

# Identify templates that need variables
edisonprompt search "{{" --format plain | wc -l
```

This collection of examples demonstrates EdisonPrompt's versatility across different domains and use cases. Start with simple templates and gradually build more sophisticated workflows as you become comfortable with the tool.