use std::path::Path;

use anyhow::Result;
use dprint_plugin_json as json;
use once_cell::sync::Lazy;

use super::FormatError;
use super::ensure_newline;

static CONF: Lazy<json::configuration::Configuration> =
    Lazy::new(|| json::configuration::ConfigurationBuilder::new().build());

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let res =
        json::format_text(path, text, &CONF).map_err(|e| FormatError::Message(e.to_string()))?;
    Ok(res.map(ensure_newline))
}
