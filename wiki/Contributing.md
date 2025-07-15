# Contributing to EdisonPrompt

Help make EdisonPrompt even better! This guide covers everything from bug reports to code contributions, ensuring a smooth collaboration process.

## 🎯 Ways to Contribute

### Bug Reports & Feature Requests
- **Report bugs** with detailed reproduction steps
- **Suggest features** that enhance prompt management workflows
- **Improve documentation** for clarity and completeness
- **Share use cases** to inspire new features

### Code Contributions
- **Fix bugs** and improve reliability
- **Add new features** following the project vision
- **Optimize performance** to maintain speed standards
- **Write tests** to improve code coverage

### Community Contributions
- **Help other users** in discussions and issues
- **Create examples** and tutorials
- **Write blog posts** about EdisonPrompt
- **Spread the word** in the developer community

## 🚀 Getting Started

### Prerequisites
- **Rust 1.70+** - Latest stable toolchain
- **Git** - Version control
- **Basic CLI experience** - Understanding of command-line tools
- **SQLite knowledge** (helpful for database features)

### Development Setup
```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/edisonprompt.git
cd edisonprompt

# Add upstream remote
git remote add upstream https://github.com/prequired/edisonprompt.git

# Install development tools
cargo install cargo-watch cargo-tarpaulin cargo-audit

# Run tests to verify setup
cargo test
```

### First Build
```bash
# Build in development mode
cargo build

# Run EdisonPrompt locally
./target/debug/edisonprompt --version

# Build optimized release
cargo build --release

# Run comprehensive tests
cargo test --all
cargo clippy -- -D warnings
cargo fmt --check
```

## 🔄 Development Workflow

### Before Starting Work

1. **Check existing issues** to avoid duplicated effort
2. **Create an issue** for significant changes
3. **Discuss approach** with maintainers
4. **Create feature branch** from latest main

```bash
# Update your fork
git checkout main
git pull upstream main
git push origin main

# Create feature branch
git checkout -b feature/amazing-feature
```

### During Development

**Write clean, performant code:**
```rust
// ✅ Good: Clear, efficient implementation
pub fn search_prompts(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
    let mut stmt = self.connection.prepare(SEARCH_QUERY)?;
    let results = stmt.query_map(params![query, limit], |row| {
        Ok(SearchResult {
            id: row.get("id")?,
            name: row.get("name")?,
            content: row.get("content")?,
            snippet: row.get("snippet")?,
        })
    })?;
    
    results.collect()
}

// ❌ Avoid: Unclear, inefficient code
pub fn find_stuff(q: &str) -> Vec<String> {
    // Unclear purpose, inefficient implementation
    let mut results = Vec::new();
    // ... problematic implementation
    results
}
```

**Follow project conventions:**
```rust
// Error handling with our custom error types
use crate::error::{PromptedsError, Result};

// Public API documentation
/// Search for prompts using full-text search.
/// 
/// # Arguments
/// * `query` - Search term or phrase
/// * `limit` - Maximum number of results
/// 
/// # Returns
/// Vector of search results ordered by relevance
pub fn search_prompts(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
    // Implementation
}

// Performance considerations
#[inline]
pub fn validate_prompt_name(name: &str) -> Result<()> {
    // Hot path - inline for performance
}
```

### Testing Requirements

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_search_functionality() {
        let temp_dir = TempDir::new().unwrap();
        let mut db = Database::new(temp_dir.path().join("test.db")).unwrap();
        
        // Create test data
        let prompt = Prompt::new("test", "hello {{name}}");
        db.create_prompt(&prompt).unwrap();
        
        // Test search
        let results = db.search_prompts("hello", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test");
    }
}
```

**Integration Tests:**
```rust
// tests/integration_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_add_and_get_workflow() {
    let mut cmd = Command::cargo_bin("edisonprompt").unwrap();
    cmd.args(&["add", "test-prompt", "--interactive"])
        .write_stdin("Hello {{name}}!")
        .assert()
        .success();
    
    let mut cmd = Command::cargo_bin("edisonprompt").unwrap();
    cmd.args(&["get", "test-prompt", "--var", "name=World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World!"));
}
```

**Performance Tests:**
```rust
// benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_search(c: &mut Criterion) {
    c.bench_function("search_1000_prompts", |b| {
        b.iter(|| {
            // Benchmark implementation
            search_prompts(black_box("test"), black_box(50))
        });
    });
}

criterion_group!(benches, benchmark_search);
criterion_main!(benches);
```

### Quality Checks

**Run before committing:**
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run all tests
cargo test

# Check test coverage
cargo tarpaulin --out Html

# Security audit
cargo audit

# Performance benchmarks
cargo bench
```

### Commit Guidelines

**Commit Message Format:**
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `test` - Adding or modifying tests
- `perf` - Performance improvements
- `chore` - Maintenance tasks

**Examples:**
```bash
git commit -m "feat(search): add highlighting for search results"
git commit -m "fix(database): handle database connection timeout gracefully"
git commit -m "docs(wiki): add troubleshooting guide for common issues"
git commit -m "perf(template): optimize variable extraction regex"
```

### Pull Request Process

1. **Update your branch:**
```bash
git checkout main
git pull upstream main
git checkout feature/amazing-feature
git rebase main
```

2. **Create Pull Request:**
- Clear title describing the change
- Detailed description of what and why
- Reference related issues
- Include screenshots for UI changes
- List any breaking changes

3. **PR Template:**
```markdown
## Summary
Brief description of the changes.

## Changes
- [ ] Added new feature X
- [ ] Fixed bug Y
- [ ] Updated documentation

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Performance benchmarks run
- [ ] Manual testing completed

## Breaking Changes
List any breaking changes.

## Related Issues
Closes #123
```

## 📋 Code Standards

### Rust Conventions

**Naming:**
```rust
// Modules: snake_case
mod template_engine;

// Structs: PascalCase
pub struct TemplateEngine;

// Functions: snake_case
pub fn extract_variables() -> Result<Vec<String>>;

// Constants: SCREAMING_SNAKE_CASE
const MAX_PROMPT_LENGTH: usize = 100_000;

// Enums: PascalCase with PascalCase variants
pub enum OutputFormat {
    Table,
    Json,
    Plain,
}
```

**Error Handling:**
```rust
// Use ? operator for error propagation
pub fn create_prompt(&mut self, prompt: &Prompt) -> Result<()> {
    let mut stmt = self.connection.prepare(INSERT_PROMPT)?;
    stmt.execute(params![prompt.id, prompt.name, prompt.content])?;
    Ok(())
}

// Custom error types with thiserror
#[derive(Error, Debug)]
pub enum PromptedsError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Prompt '{name}' not found")]
    PromptNotFound { name: String },
}
```

**Documentation:**
```rust
/// Search for prompts using full-text search.
/// 
/// This function uses SQLite's FTS5 extension for fast, relevant search
/// results. Results are ranked by relevance using BM25 algorithm.
/// 
/// # Arguments
/// * `query` - Search term or phrase. Supports FTS5 query syntax.
/// * `limit` - Maximum number of results to return (1-1000).
/// 
/// # Returns
/// Vector of `SearchResult` structs ordered by relevance score.
/// 
/// # Errors
/// Returns `PromptedsError::Database` if the search query fails.
/// 
/// # Examples
/// ```
/// let results = db.search_prompts("email template", 10)?;
/// assert!(results.len() <= 10);
/// ```
pub fn search_prompts(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
    // Implementation
}
```

### Performance Guidelines

**Memory Efficiency:**
```rust
// ✅ Use references when possible
pub fn format_prompt(prompt: &Prompt) -> String {
    format!("{}: {}", prompt.name, prompt.content)
}

// ✅ Iterator chains for efficiency
pub fn filter_prompts(prompts: &[Prompt], tag: &str) -> impl Iterator<Item = &Prompt> {
    prompts.iter().filter(move |p| p.tags.contains(tag))
}

// ❌ Avoid unnecessary allocations
pub fn bad_format(prompt: Prompt) -> String {
    let name = prompt.name.clone();  // Unnecessary clone
    format!("{}: {}", name, prompt.content)
}
```

**Database Performance:**
```rust
// ✅ Prepared statements for repeated queries
pub struct Database {
    connection: Connection,
    search_stmt: Option<Statement>,
}

impl Database {
    pub fn search_prompts(&mut self, query: &str) -> Result<Vec<SearchResult>> {
        let stmt = self.search_stmt.get_or_insert_with(|| {
            self.connection.prepare(SEARCH_QUERY).unwrap()
        });
        // Use prepared statement
    }
}
```

### Database Conventions

**Schema Changes:**
```rust
// Always include migration path
pub fn migrate_to_v2(conn: &Connection) -> Result<()> {
    conn.execute(
        "ALTER TABLE prompts ADD COLUMN priority INTEGER DEFAULT 0",
        [],
    )?;
    conn.execute(
        "INSERT INTO schema_version (version) VALUES (2)",
        [],
    )?;
    Ok(())
}
```

**Query Performance:**
```sql
-- Use indexes for common queries
CREATE INDEX IF NOT EXISTS idx_prompts_updated ON prompts(updated_at);

-- Optimize FTS5 queries
SELECT name, content, bm25(prompts_fts) as rank
FROM prompts_fts 
WHERE prompts_fts MATCH ?
ORDER BY rank
LIMIT ?;
```

## 🔍 Feature Development

### Adding New Commands

1. **Define command structure:**
```rust
// In src/cli/args.rs
#[derive(Subcommand)]
pub enum Commands {
    // ... existing commands
    
    /// New awesome command
    Awesome {
        /// Description of the argument
        #[arg(short, long)]
        option: String,
    },
}
```

2. **Implement command logic:**
```rust
// In src/cli/commands/awesome.rs
use crate::{Database, Result};

pub struct AwesomeCommand;

impl AwesomeCommand {
    pub fn execute(database: &mut Database, option: String) -> Result<()> {
        // Command implementation
        println!("Executing awesome command with: {}", option);
        Ok(())
    }
}
```

3. **Add to command router:**
```rust
// In src/main.rs
match args.command {
    // ... existing commands
    Commands::Awesome { option } => {
        AwesomeCommand::execute(&mut database, option)?;
    }
}
```

4. **Write tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_awesome_command() {
        // Test implementation
    }
}
```

### Database Schema Changes

**Always provide migration path:**
```rust
pub fn migrate_database(conn: &Connection, from_version: u32, to_version: u32) -> Result<()> {
    match (from_version, to_version) {
        (1, 2) => migrate_v1_to_v2(conn),
        (2, 3) => migrate_v2_to_v3(conn),
        _ => Err(PromptedsError::UnsupportedMigration { from_version, to_version }),
    }
}
```

### Performance Requirements

**All new features must:**
- Maintain <100ms startup time
- Keep search operations <50ms for 1000+ prompts
- Use <50MB memory during normal operation
- Include performance benchmarks

## 🐛 Bug Fix Process

### Bug Report Analysis
1. **Reproduce the issue** locally
2. **Write a failing test** that demonstrates the bug
3. **Fix the issue** with minimal changes
4. **Verify the fix** with comprehensive testing
5. **Add regression tests** to prevent recurrence

### Bug Fix Example:
```rust
// Before: Bug in search highlighting
pub fn highlight_search_terms(content: &str, query: &str) -> String {
    content.replace(query, &format!("<mark>{}</mark>", query))  // Simple, buggy
}

// After: Proper case-insensitive highlighting
pub fn highlight_search_terms(content: &str, query: &str) -> String {
    let regex = RegexBuilder::new(&regex::escape(query))
        .case_insensitive(true)
        .build()
        .unwrap();
    
    regex.replace_all(content, "<mark>$0</mark>").to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_insensitive_highlighting() {
        let result = highlight_search_terms("Hello World", "hello");
        assert_eq!(result, "<mark>Hello</mark> World");
    }
}
```

## 📚 Documentation

### Code Documentation
```rust
// Public APIs require documentation
/// Database operations for prompt management.
/// 
/// This struct manages the SQLite database connection and provides
/// methods for CRUD operations on prompts, variables, and tags.
/// 
/// # Examples
/// ```
/// use tempfile::TempDir;
/// let temp_dir = TempDir::new()?;
/// let mut db = Database::new(temp_dir.path().join("test.db"))?;
/// ```
pub struct Database {
    connection: Connection,
}
```

### Wiki Documentation
- Update relevant wiki pages for new features
- Add examples and use cases
- Include troubleshooting information
- Update command reference

## 🚦 CI/CD and Automation

### GitHub Actions Workflow
All contributions trigger automated checks:
- **Rust formatting** with `cargo fmt`
- **Linting** with `cargo clippy`
- **Unit tests** with `cargo test`
- **Integration tests** across platforms
- **Performance benchmarks** with regression detection
- **Security audit** with `cargo audit`

### Local Pre-commit Hooks
```bash
# Install pre-commit hooks
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Format check
cargo fmt --check

# Lint check
cargo clippy -- -D warnings

# Test check
cargo test

echo "All checks passed!"
EOF

chmod +x .git/hooks/pre-commit
```

## 🎯 Release Process

### Version Management
- **Semantic Versioning**: MAJOR.MINOR.PATCH
- **Breaking changes**: Increment MAJOR
- **New features**: Increment MINOR  
- **Bug fixes**: Increment PATCH

### Release Checklist
- [ ] All tests passing
- [ ] Performance benchmarks meet requirements
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created
- [ ] Crates.io publication
- [ ] GitHub release with assets

## 🤝 Community Guidelines

### Code of Conduct
- **Be respectful** and constructive in all interactions
- **Help others** learn and improve
- **Focus on technical merit** rather than personal preferences
- **Assume good intentions** in discussions

### Communication
- **Use GitHub issues** for bug reports and feature requests
- **Use discussions** for questions and general feedback
- **Be clear and specific** in all communications
- **Provide context** for your suggestions and reports

### Recognition
Contributors are recognized in:
- **Git commit history** with proper attribution
- **CHANGELOG.md** for significant contributions
- **GitHub contributors** page
- **Release notes** for major features

## 📞 Getting Help

### Development Questions
- **Create a discussion** for general development questions
- **Open an issue** for specific bugs or feature requests
- **Check existing issues** before creating new ones

### Mentorship
New contributors are welcome! Core maintainers provide guidance for:
- First-time contributors
- Complex feature development
- Performance optimization
- Best practices and conventions

### Resources
- **Rust Book**: https://doc.rust-lang.org/book/
- **Cargo Guide**: https://doc.rust-lang.org/cargo/
- **SQLite Documentation**: https://sqlite.org/docs.html
- **EdisonPrompt Wiki**: Comprehensive project documentation

---

**Thank you for contributing to EdisonPrompt!** Your efforts help create better tools for the AI development community. Together, we're building something exceptional. 🚀

## See Also

- [Getting Started](Getting-Started) - Project setup and basic usage
- [Command Reference](Command-Reference) - Complete API documentation
- [Performance & Benchmarks](Performance-Benchmarks) - Performance standards and testing
- [Troubleshooting](Troubleshooting) - Common issues and solutions