use std::path::Path;

use anyhow::Result;
use dprint_plugin_dockerfile as docker;
use once_cell::sync::Lazy;

use super::{FormatError, ensure_newline};
use crate::formats::bash;

static CONF: Lazy<docker::configuration::Configuration> =
    Lazy::new(|| docker::configuration::ConfigurationBuilder::new().build());

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    // First run dockerfile formatter
    let primary =
        docker::format_text(path, text, &CONF).map_err(|e| FormatError::Message(e.to_string()))?;
    let mut content = primary.unwrap_or_else(|| text.to_string());

    // Then normalize embedded RUN shell lines for better alignment (like bash formatter).
    if content.contains("RUN ") {
        let mut out_lines = Vec::new();
        let mut lines_iter = content.lines().peekable();
        while let Some(line) = lines_iter.next() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("RUN ") {
                // collect continuation lines (ending with '\' or indented lines following)
                let mut collected = vec![trimmed.trim_start_matches("RUN ").to_string()];
                while let Some(next) = lines_iter.peek() {
                    let nt = next.trim_end();
                    if nt.ends_with('\\') || next.starts_with(' ') || next.starts_with('\t') {
                        collected.push(nt.trim_start().to_string());
                        lines_iter.next();
                    } else {
                        break;
                    }
                }
                let shell_src = collected.join("\n");
                match bash::format(Path::new("inline.sh"), &shell_src) {
                    Ok(Some(formatted)) => {
                        let blines: Vec<&str> = formatted.trim_end().lines().collect();
                        if let Some(first) = blines.first() {
                            out_lines.push(format!("RUN {}", first));
                            for extra in blines.iter().skip(1) {
                                out_lines.push(format!("  {}", extra));
                            }
                        } else {
                            out_lines.push(line.to_string());
                        }
                    }
                    _ => out_lines.push(line.to_string()),
                }
            } else {
                out_lines.push(line.to_string());
            }
        }
        content = out_lines.join("\n");
    }

    let out = if content == text {
        None
    } else {
        Some(ensure_newline(content))
    };
    Ok(out)
}
