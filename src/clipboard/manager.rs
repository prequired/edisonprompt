use crate::error::Result;
use arboard::Clipboard;
use std::time::Duration;

pub struct ClipboardManager {
    clipboard: Option<Clipboard>,
    timeout: Duration,
    enable_fallback: bool,
}

impl ClipboardManager {
    pub fn new(timeout_ms: u64, enable_fallback: bool) -> Self {
        let clipboard = match Clipboard::new() {
            Ok(clipboard) => Some(clipboard),
            Err(_) => None,
        };

        Self {
            clipboard,
            timeout: Duration::from_millis(timeout_ms),
            enable_fallback,
        }
    }

    pub fn get_text(&mut self) -> Result<String> {
        if let Some(ref mut clipboard) = self.clipboard {
            match clipboard.get_text() {
                Ok(text) => Ok(text),
                Err(e) => {
                    if self.enable_fallback {
                        self.fallback_get_text()
                    } else {
                        Err(e.into())
                    }
                }
            }
        } else if self.enable_fallback {
            self.fallback_get_text()
        } else {
            Err(crate::error::PromptedsError::Clipboard(
                arboard::Error::ContentNotAvailable
            ))
        }
    }

    pub fn set_text(&mut self, text: &str) -> Result<()> {
        if let Some(ref mut clipboard) = self.clipboard {
            match clipboard.set_text(text) {
                Ok(()) => Ok(()),
                Err(e) => {
                    if self.enable_fallback {
                        self.fallback_set_text(text)
                    } else {
                        Err(e.into())
                    }
                }
            }
        } else if self.enable_fallback {
            self.fallback_set_text(text)
        } else {
            Err(crate::error::PromptedsError::Clipboard(
                arboard::Error::ContentNotAvailable
            ))
        }
    }

    pub fn is_available(&self) -> bool {
        self.clipboard.is_some() || self.enable_fallback
    }

    fn fallback_get_text(&self) -> Result<String> {
        // In headless environments, try reading from a temporary file
        let temp_file = std::env::temp_dir().join("edisonprompt_clipboard.txt");
        
        if temp_file.exists() {
            let content = std::fs::read_to_string(temp_file)?;
            Ok(content)
        } else {
            Err(crate::error::PromptedsError::Clipboard(
                arboard::Error::ContentNotAvailable
            ))
        }
    }

    fn fallback_set_text(&self, text: &str) -> Result<()> {
        // In headless environments, write to a temporary file
        let temp_file = std::env::temp_dir().join("edisonprompt_clipboard.txt");
        std::fs::write(temp_file, text)?;
        Ok(())
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new(5000, true)
    }
}