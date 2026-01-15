use std::path::Path;

use anyhow::Result;
use dprint_plugin_sql as sql;
use once_cell::sync::Lazy;

use super::{FormatError, ensure_newline};

static CONF: Lazy<sql::configuration::Configuration> =
    Lazy::new(|| sql::configuration::ConfigurationBuilder::new().build());

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let res =
        sql::format_text(path, text, &CONF).map_err(|e| FormatError::Message(e.to_string()))?;
    Ok(res.map(ensure_newline))
}
