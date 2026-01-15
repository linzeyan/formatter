use std::path::Path;

use anyhow::Result;

use super::{FormatError, ensure_newline};

/// Heuristic pretty formatter for Makefiles:
/// - trims行尾空白
/// - 壓縮連續空行為 1
/// - recipe 行強制以 tab 開頭（偵測上一個目標行）
pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let mut out = String::new();
    let mut prev_blank = false;
    let mut in_rule = false;

    for line in text.lines() {
        let trimmed_end = line.trim_end();
        let trimmed_start = trimmed_end.trim_start();

        if trimmed_start.is_empty() {
            if prev_blank {
                continue;
            }
            prev_blank = true;
            out.push('\n');
            continue;
        }
        prev_blank = false;

        if trimmed_start.starts_with('#') {
            out.push_str(trimmed_start);
            out.push('\n');
            continue;
        }

        // rule detection: line with ':' not starting with tab/space
        if !trimmed_end.starts_with('\t')
            && trimmed_end.contains(':')
            && !trimmed_end.starts_with(' ')
        {
            if let Some((targets, deps_raw)) = trimmed_end.split_once(':') {
                let deps = deps_raw.split_whitespace().collect::<Vec<_>>().join(" ");
                if deps.is_empty() {
                    out.push_str(targets.trim_end());
                    out.push(':');
                } else {
                    out.push_str(targets.trim_end());
                    out.push_str(": ");
                    out.push_str(&deps);
                }
            } else {
                out.push_str(trimmed_end);
            }
            out.push('\n');
            in_rule = true;
            continue;
        }

        if in_rule && !trimmed_end.starts_with('\t') && !trimmed_end.is_empty() {
            // recipe line: enforce leading tab
            out.push('\t');
            out.push_str(trimmed_start);
            out.push('\n');
            continue;
        }

        out.push_str(trimmed_end);
        out.push('\n');
    }

    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
