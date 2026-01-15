use std::path::Path;

use anyhow::Result;
use ini::Ini;

use super::{FormatError, ensure_newline};

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let parsed = Ini::load_from_str(text)
        .map_err(|e| FormatError::Message(format!("ini parse error: {e}")))?;
    let mut buf = Vec::new();
    parsed
        .write_to(&mut buf)
        .map_err(|e| FormatError::Message(format!("ini write error: {e}")))?;
    let out = String::from_utf8(buf).map_err(|e| FormatError::Message(e.to_string()))?;
    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
