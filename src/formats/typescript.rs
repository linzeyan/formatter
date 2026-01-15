use std::path::Path;

use anyhow::Result;
use dprint_plugin_typescript as ts;
use once_cell::sync::Lazy;

use super::{FormatError, ensure_newline};

static CONF: Lazy<ts::configuration::Configuration> =
    Lazy::new(|| ts::configuration::ConfigurationBuilder::new().build());

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let ext = path.extension().map(|e| e.to_string_lossy().to_string());
    let res = ts::format_text(ts::FormatTextOptions {
        path,
        extension: ext.as_deref(),
        text: text.into(),
        config: &CONF,
        external_formatter: None,
    })
    .map_err(|e| FormatError::Message(e.to_string()))?;
    Ok(res.map(ensure_newline))
}
