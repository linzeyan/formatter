use std::path::Path;

use anyhow::Result;

use super::{FormatError, ensure_newline};

/// Simple brace/semicolon-based formatter for nginx.conf
pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let mut out = String::new();
    let mut indent: i32 = 0;

    for raw_line in text.lines() {
        let mut line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        // comments keep alignment
        if line.starts_with('#') {
            out.push_str(&"    ".repeat(indent.max(0) as usize));
            out.push_str(line);
            out.push('\n');
            continue;
        }

        // decrease indent if line starts with }
        if line.starts_with('}') {
            indent -= 1;
        }
        if indent < 0 {
            indent = 0;
        }
        out.push_str(&"    ".repeat(indent as usize));
        // ensure space before { and ; spacing trimmed
        line = line.trim_end_matches(';');
        let mut to_write = line.trim().replace('{', " {");
        to_write = to_write.split_whitespace().collect::<Vec<_>>().join(" ");
        if raw_line.trim_end().ends_with('{') {
            to_write.push_str(" {");
        }
        if raw_line.trim_end().ends_with(';') {
            to_write.push(';');
        }
        out.push_str(&to_write);
        out.push('\n');

        if raw_line.contains('{') {
            indent += 1;
        }
        if raw_line.contains('}') {
            // handled at start
        }
    }

    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
