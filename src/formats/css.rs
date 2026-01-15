use std::path::Path;

use anyhow::Result;
use dprint_plugin_css as css;
use once_cell::sync::Lazy;

use super::{FormatError, ensure_newline};

static CONF: Lazy<css::configuration::Configuration> =
    Lazy::new(|| css::configuration::ConfigurationBuilder::new().build());

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let formatted =
        css::format_text(path, text, &CONF).map_err(|e| FormatError::Message(e.to_string()))?;
    if formatted == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(formatted)))
    }
}
