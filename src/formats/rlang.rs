use std::path::Path;

use anyhow::Result;

use super::{FormatError, ensure_newline};

/// Minimal R formatter using brace indentation and trimming.
pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let mut out = String::new();
    let mut indent: i32 = 0;

    for raw in text.lines() {
        let line = raw.trim_end();
        if line.is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        if trimmed.starts_with('}') {
            indent -= 1;
        }
        if indent < 0 {
            indent = 0;
        }
        out.push_str(&"  ".repeat(indent as usize));
        out.push_str(trimmed);
        out.push('\n');
        if trimmed.ends_with('{') {
            indent += 1;
        }
    }

    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
