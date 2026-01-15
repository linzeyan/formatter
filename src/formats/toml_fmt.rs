use std::path::Path;

use anyhow::Result;
use dprint_plugin_toml as toml;
use once_cell::sync::Lazy;

use super::FormatError;
use super::ensure_newline;

static CONF: Lazy<toml::configuration::Configuration> =
    Lazy::new(|| toml::configuration::ConfigurationBuilder::new().build());

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let res =
        toml::format_text(path, text, &CONF).map_err(|e| FormatError::Message(e.to_string()))?;
    Ok(res.map(ensure_newline))
}
