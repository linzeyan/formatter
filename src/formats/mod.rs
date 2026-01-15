use std::path::Path;

use anyhow::Result;

pub mod bash;
pub mod css;
pub mod dockerfile;
pub mod go;
pub mod graphql;
pub mod hcl;
pub mod html;
pub mod ini;
pub mod javascript;
pub mod json;
pub mod lua;
pub mod makefile;
pub mod markdown;
pub mod nginx;
pub mod protobuf;
pub mod python;
pub mod rlang;
pub mod rustfmt;
pub mod sql;
pub mod toml_fmt;
pub mod typescript;
pub mod xml;
pub mod yaml;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatKind {
    Json,
    Yaml,
    Toml,
    Xml,
    Markdown,
    Bash,
    Dockerfile,
    Makefile,
    Ini,
    Nginx,
    Html,
    Css,
    TypeScript,
    JavaScript,
    Golang,
    Rust,
    Python,
    Protobuf,
    Graphql,
    Hcl,
    Lua,
    R,
    Sql,
}

#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    #[error("{0}")]
    Message(String),
}

impl From<anyhow::Error> for FormatError {
    fn from(value: anyhow::Error) -> Self {
        FormatError::Message(value.to_string())
    }
}

impl From<std::io::Error> for FormatError {
    fn from(value: std::io::Error) -> Self {
        FormatError::Message(value.to_string())
    }
}

pub fn format_dispatch(
    kind: FormatKind,
    path: &Path,
    text: &str,
) -> Result<Option<String>, FormatError> {
    let out = match kind {
        FormatKind::Json => json::format(path, text),
        FormatKind::Yaml => yaml::format(path, text),
        FormatKind::Toml => toml_fmt::format(path, text),
        FormatKind::Xml => xml::format(path, text),
        FormatKind::Markdown => markdown::format(path, text),
        FormatKind::Bash => bash::format(path, text),
        FormatKind::Css => css::format(path, text),
        FormatKind::TypeScript => typescript::format(path, text),
        FormatKind::JavaScript => javascript::format(path, text),
        FormatKind::Dockerfile => dockerfile::format(path, text),
        FormatKind::Sql => sql::format(path, text),
        FormatKind::Python => python::format(path, text),
        FormatKind::Golang => go::format(path, text),
        FormatKind::Rust => rustfmt::format(path, text),
        FormatKind::Ini => ini::format(path, text),
        FormatKind::Graphql => graphql::format(path, text),
        FormatKind::Hcl => hcl::format(path, text),
        FormatKind::Lua => lua::format(path, text),
        FormatKind::Html => html::format(path, text),
        FormatKind::Makefile => makefile::format(path, text),
        FormatKind::Nginx => nginx::format(path, text),
        FormatKind::Protobuf => protobuf::format(path, text),
        FormatKind::R => rlang::format(path, text),
    }?;
    Ok(out)
}

pub fn detect_kind_from_label(label: &str) -> Option<FormatKind> {
    let l = label.to_lowercase();
    match l.as_str() {
        "json" => Some(FormatKind::Json),
        "yaml" | "yml" => Some(FormatKind::Yaml),
        "toml" => Some(FormatKind::Toml),
        "xml" => Some(FormatKind::Xml),
        "md" | "markdown" => Some(FormatKind::Markdown),
        "bash" | "sh" | "shell" => Some(FormatKind::Bash),
        "docker" | "dockerfile" => Some(FormatKind::Dockerfile),
        "makefile" | "mk" => Some(FormatKind::Makefile),
        "ini" => Some(FormatKind::Ini),
        "nginx" => Some(FormatKind::Nginx),
        "html" | "htm" => Some(FormatKind::Html),
        "css" => Some(FormatKind::Css),
        "ts" | "tsx" | "typescript" => Some(FormatKind::TypeScript),
        "js" | "jsx" | "javascript" => Some(FormatKind::JavaScript),
        "go" | "golang" => Some(FormatKind::Golang),
        "rs" | "rust" => Some(FormatKind::Rust),
        "py" | "python" => Some(FormatKind::Python),
        "proto" | "protobuf" => Some(FormatKind::Protobuf),
        "gql" | "graphql" => Some(FormatKind::Graphql),
        "hcl" | "tf" => Some(FormatKind::Hcl),
        "lua" => Some(FormatKind::Lua),
        "r" => Some(FormatKind::R),
        "sql" => Some(FormatKind::Sql),
        _ => None,
    }
}

pub fn detect_kind(path: &Path) -> Option<FormatKind> {
    let file_name = path.file_name()?.to_string_lossy().to_lowercase();
    if file_name == "dockerfile" {
        return Some(FormatKind::Dockerfile);
    }
    if file_name == "makefile" {
        return Some(FormatKind::Makefile);
    }
    if file_name == "nginx.conf" || file_name.ends_with(".nginx") {
        return Some(FormatKind::Nginx);
    }

    let ext = path
        .extension()
        .map(|s| s.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "json" | "jsonc" => Some(FormatKind::Json),
        "yaml" | "yml" => Some(FormatKind::Yaml),
        "toml" => Some(FormatKind::Toml),
        "xml" => Some(FormatKind::Xml),
        "md" | "markdown" => Some(FormatKind::Markdown),
        "sh" | "bash" => Some(FormatKind::Bash),
        "dockerfile" => Some(FormatKind::Dockerfile),
        "mk" => Some(FormatKind::Makefile),
        "ini" => Some(FormatKind::Ini),
        "conf" => Some(FormatKind::Nginx),
        "html" | "htm" => Some(FormatKind::Html),
        "css" => Some(FormatKind::Css),
        "ts" | "tsx" => Some(FormatKind::TypeScript),
        "js" | "jsx" | "mjs" | "cjs" => Some(FormatKind::JavaScript),
        "go" => Some(FormatKind::Golang),
        "rs" => Some(FormatKind::Rust),
        "py" => Some(FormatKind::Python),
        "proto" => Some(FormatKind::Protobuf),
        "graphql" | "gql" => Some(FormatKind::Graphql),
        "hcl" | "tf" => Some(FormatKind::Hcl),
        "lua" => Some(FormatKind::Lua),
        "r" => Some(FormatKind::R),
        "sql" => Some(FormatKind::Sql),
        _ => None,
    }
}

// helper shared
pub fn ensure_newline(mut text: String) -> String {
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text
}
