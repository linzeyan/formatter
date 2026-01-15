use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use dprint_plugin_markdown as md;
use once_cell::sync::Lazy;

use super::{FormatError, FormatKind, detect_kind_from_label, ensure_newline, format_dispatch};

static CONF: Lazy<md::configuration::Configuration> =
    Lazy::new(|| md::configuration::ConfigurationBuilder::new().build());

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let mut code_cb = |lang: &str, code: &str, _line: u32| -> anyhow::Result<Option<String>> {
        let Some(kind) = detect_kind_from_label(lang) else {
            return Ok(None);
        };
        if matches!(kind, FormatKind::Markdown) {
            return Ok(None);
        }
        let fake_path = fake_path_for_kind(kind);
        match format_dispatch(kind, &fake_path, code) {
            Ok(Some(out)) => Ok(Some(out)),
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    };
    let res = md::format_text(text, &CONF, &mut code_cb)
        .map_err(|e| FormatError::Message(e.to_string()))?;
    Ok(res.map(ensure_newline))
}

fn fake_path_for_kind(kind: FormatKind) -> PathBuf {
    match kind {
        FormatKind::Json => PathBuf::from("code.json"),
        FormatKind::Yaml => PathBuf::from("code.yaml"),
        FormatKind::Toml => PathBuf::from("code.toml"),
        FormatKind::Xml => PathBuf::from("code.xml"),
        FormatKind::Markdown => PathBuf::from("code.md"),
        FormatKind::Bash => PathBuf::from("code.sh"),
        FormatKind::Dockerfile => PathBuf::from("Dockerfile"),
        FormatKind::Makefile => PathBuf::from("Makefile"),
        FormatKind::Ini => PathBuf::from("code.ini"),
        FormatKind::Nginx => PathBuf::from("code.conf"),
        FormatKind::Html => PathBuf::from("code.html"),
        FormatKind::Css => PathBuf::from("code.css"),
        FormatKind::TypeScript => PathBuf::from("code.ts"),
        FormatKind::JavaScript => PathBuf::from("code.js"),
        FormatKind::Golang => PathBuf::from("code.go"),
        FormatKind::Rust => PathBuf::from("code.rs"),
        FormatKind::Python => PathBuf::from("code.py"),
        FormatKind::Protobuf => PathBuf::from("code.proto"),
        FormatKind::Graphql => PathBuf::from("code.graphql"),
        FormatKind::Hcl => PathBuf::from("code.hcl"),
        FormatKind::Lua => PathBuf::from("code.lua"),
        FormatKind::R => PathBuf::from("code.r"),
        FormatKind::Sql => PathBuf::from("code.sql"),
    }
}
