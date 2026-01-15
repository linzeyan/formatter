use std::path::Path;

use anyhow::Result;

use super::FormatError;

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let syntax = syn::parse_file(text)
        .map_err(|e| FormatError::Message(format!("rust parse error: {e}")))?;
    let out = prettyplease::unparse(&syntax);
    if out == text { Ok(None) } else { Ok(Some(out)) }
}
