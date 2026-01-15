use std::path::Path;

use anyhow::Result;
use dprint_plugin_ruff as ruff;
use once_cell::sync::Lazy;

use super::{FormatError, ensure_newline};

static CONF: Lazy<ruff::configuration::Configuration> =
    Lazy::new(ruff::configuration::Configuration::default);

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let res =
        ruff::format_text(path, text, &CONF).map_err(|e| FormatError::Message(e.to_string()))?;
    Ok(res.map(ensure_newline))
}
