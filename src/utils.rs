use crate::error::Result;
use colored::*;
use std::io::{self, Write};

pub fn print_success(message: &str) {
    println!("{} {}", "✅".green(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "❌".red(), message.red());
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️".yellow(), message.yellow());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message);
}

pub fn confirm(message: &str) -> Result<bool> {
    print!("{} (y/N): ", message);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}

pub fn format_duration(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{:.1}ms", seconds * 1000.0)
    } else if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else {
        let minutes = (seconds / 60.0) as u32;
        let remaining_seconds = seconds % 60.0;
        format!("{}m {:.1}s", minutes, remaining_seconds)
    }
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn strip_ansi(s: &str) -> String {
    // Simple ANSI escape sequence removal
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}