use std::path::Path;

use anyhow::Result;
use pretty_yaml::{config::*, format_text};

use super::{FormatError, ensure_newline};

fn detect_line_break(text: &str) -> LineBreak {
    #[cfg(windows)]
    {
        return LineBreak::Crlf;
    }

    if text.contains("\r\n") {
        LineBreak::Crlf
    } else {
        LineBreak::Lf
    }
}

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    // Configure pretty_yaml to mirror yamlfmt 基本預設：縮排 2、無文檔起始、保留鍵順序、
    // 不強制行寬、不修剪行尾空白、保留注釋。
    let line_break = detect_line_break(text);
    let mut options = FormatOptions::default();
    options.layout.indent_width = 2;
    options.layout.print_width = 10_000; // 模擬「無行寬限制」
    options.layout.line_break = line_break;
    options.language.trailing_comma = false;
    options.language.trim_trailing_whitespaces = false;
    options.language.prefer_single_line = false;
    options.language.flow_sequence_prefer_single_line = None;
    options.language.flow_map_prefer_single_line = None;

    let formatted = format_text(text, &options)
        .map_err(|e| FormatError::Message(format!("YAML format error: {e}")))?;

    // pretty_yaml 不自動確保尾隨換行，與現有 formatter 行為一致需要補齊。
    let output = ensure_newline(formatted);

    if output == text {
        Ok(None)
    } else {
        Ok(Some(output))
    }
}
