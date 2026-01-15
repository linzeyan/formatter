use std::path::Path;

use anyhow::Result;

use super::FormatError;

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    match gofmt::formatter::format(text) {
        Ok(bytes) => {
            let out = String::from_utf8_lossy(&bytes).to_string();
            if out == text { Ok(None) } else { Ok(Some(out)) }
        }
        Err(e) => Err(FormatError::Message(format!("gofmt error: {e}"))),
    }
}
