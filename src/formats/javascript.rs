use std::path::Path;

use anyhow::Result;

use super::{FormatError, ensure_newline};
use crate::formats::typescript;

pub fn format(path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    typescript::format(path, text).map(|opt| opt.map(ensure_newline))
}
