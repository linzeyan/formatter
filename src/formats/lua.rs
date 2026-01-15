use std::path::Path;

use anyhow::Result;
use stylua_lib::{Config as StyluaConfig, OutputVerification, format_code};

use super::FormatError;

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let cfg = StyluaConfig::default();
    match format_code(text, cfg, None, OutputVerification::None) {
        Ok(out) => {
            if out == text {
                Ok(None)
            } else {
                Ok(Some(out))
            }
        }
        Err(e) => Err(FormatError::Message(format!("lua format error: {e}"))),
    }
}
