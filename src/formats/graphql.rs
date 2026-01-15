use std::path::Path;

use anyhow::Result;

use super::{FormatError, ensure_newline};

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let opts = pretty_graphql::config::FormatOptions::default();
    let out = pretty_graphql::format_text(text, &opts)
        .map_err(|e| FormatError::Message(format!("graphql format error: {e}")))?;
    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
