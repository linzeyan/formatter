use std::path::Path;

use anyhow::Result;

use super::{FormatError, ensure_newline};

/// Heuristic pretty formatter for .proto files (brace/semicolon indentation, 2 spaces)
pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let mut out = String::new();
    let mut indent: i32 = 0;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let mut current = line.to_string();
        if current.starts_with('}') {
            indent -= 1;
        }
        if indent < 0 {
            indent = 0;
        }
        // normalize spacing around = and ;
        // normalize whitespace around '=' and collapse spaces
        current = current.replace("=", " = ");
        current = current.split_whitespace().collect::<Vec<_>>().join(" ");
        let has_semicolon = current.ends_with(';');
        let has_open = current.ends_with('{');
        out.push_str(&"  ".repeat(indent as usize));
        out.push_str(&current);
        out.push('\n');
        if has_open {
            indent += 1;
        }
        if current.ends_with("};") {
            indent -= 1;
        } else if current.ends_with('}') {
            // closing brace only, indent already decreased on start of next line
        }
        if has_semicolon {
            // nothing
        }
    }

    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
