use std::path::Path;

use anyhow::Result;

use super::{FormatError, ensure_newline};

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let body: hcl::Body =
        hcl::from_str(text).map_err(|e| FormatError::Message(format!("hcl parse error: {e}")))?;
    let out = hcl::format::to_string(&body)
        .map_err(|e| FormatError::Message(format!("hcl format error: {e}")))?;
    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
