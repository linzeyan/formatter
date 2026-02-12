use std::path::Path;

use formatter::formats::{
    FormatError, bash, css, dockerfile, go, graphql, hcl, html, ini, javascript, json, lua,
    makefile, markdown, nginx, protobuf, python, rlang, rustfmt, sql, toml_fmt, typescript, xml,
    yaml,
};

/// Helper: format once, then format the result again.
/// Second format should return None (no change) proving idempotency.
fn assert_idempotent(
    f: fn(&Path, &str) -> anyhow::Result<Option<String>, FormatError>,
    path: &str,
    input: &str,
) {
    let first = f(Path::new(path), input)
        .unwrap()
        .unwrap_or_else(|| input.to_string());

    let second = f(Path::new(path), &first).unwrap();

    assert!(
        second.is_none(),
        "Formatter for {} is not idempotent.\nFirst output:\n{}\nSecond output:\n{:?}",
        path,
        first,
        second
    );
}

// ──────────────────────────────────────────────
// 冪等性測試：格式化兩次結果應相同
// ──────────────────────────────────────────────

#[test]
fn idempotent_json() {
    assert_idempotent(json::format, "a.json", "{\"a\":1,\"b\":[2,3]}");
}

#[test]
fn idempotent_json_nested() {
    assert_idempotent(
        json::format,
        "a.json",
        r#"{"a":{"b":{"c":{"d":1}}},"e":[1,2,3]}"#,
    );
}

#[test]
fn idempotent_yaml() {
    assert_idempotent(yaml::format, "a.yaml", "foo: 1\nbar: [2,3]\n");
}

#[test]
fn idempotent_yaml_with_comments() {
    assert_idempotent(yaml::format, "a.yaml", "# comment\nfoo: 1\n");
}

#[test]
fn idempotent_toml() {
    assert_idempotent(
        toml_fmt::format,
        "a.toml",
        "[package]\nname=\"test\"\nversion=\"0.1.0\"\n",
    );
}

#[test]
fn idempotent_xml() {
    assert_idempotent(xml::format, "a.xml", "<root><child>text</child></root>");
}

#[test]
fn idempotent_bash_simple() {
    assert_idempotent(bash::format, "a.sh", "#!/bin/bash\necho hello\n");
}

#[test]
fn idempotent_bash_if_else() {
    assert_idempotent(
        bash::format,
        "a.sh",
        "if [ \"$a\" -gt 0 ]; then\n  echo yes\nelse\n  echo no\nfi\n",
    );
}

#[test]
fn idempotent_bash_case() {
    assert_idempotent(
        bash::format,
        "a.sh",
        "case \"$1\" in\n  a)\n    echo a\n  ;;\n  *)\n    echo other\n  ;;\nesac\n",
    );
}

#[test]
fn idempotent_bash_function() {
    assert_idempotent(
        bash::format,
        "a.sh",
        "my_func() {\n  echo hello\n  echo world\n}\n",
    );
}

#[test]
fn idempotent_css() {
    assert_idempotent(css::format, "a.css", "h1 {\n  color: red;\n}\n");
}

#[test]
fn idempotent_typescript() {
    assert_idempotent(typescript::format, "a.ts", "const  x=1");
}

#[test]
fn idempotent_javascript() {
    assert_idempotent(javascript::format, "a.js", "const  a=1");
}

#[test]
fn idempotent_dockerfile() {
    assert_idempotent(
        dockerfile::format,
        "Dockerfile",
        "FROM alpine\nRUN echo hello\n",
    );
}

#[test]
fn idempotent_sql() {
    assert_idempotent(sql::format, "a.sql", "select * from users where id=1");
}

#[test]
fn idempotent_python() {
    assert_idempotent(python::format, "a.py", "x=1\ny=2\n");
}

#[test]
fn idempotent_go() {
    assert_idempotent(
        go::format,
        "a.go",
        "package main\n\nfunc main() {\n\tprintln(\"hi\")\n}\n",
    );
}

#[test]
fn idempotent_rust() {
    assert_idempotent(
        rustfmt::format,
        "a.rs",
        "fn main() {\n    println!(\"hello\");\n}\n",
    );
}

#[test]
fn idempotent_ini() {
    assert_idempotent(ini::format, "a.ini", "[section]\nkey=value\n");
}

#[test]
fn idempotent_graphql() {
    assert_idempotent(
        graphql::format,
        "a.graphql",
        "type Query {\n  hello: String\n}\n",
    );
}

#[test]
fn idempotent_hcl() {
    assert_idempotent(
        hcl::format,
        "a.hcl",
        "resource \"aws_instance\" \"example\" {\n  ami = \"abc\"\n}\n",
    );
}

#[test]
fn idempotent_lua() {
    assert_idempotent(lua::format, "a.lua", "local t = { 1, 2, 3 }\nprint(t)\n");
}

#[test]
fn idempotent_html() {
    assert_idempotent(
        html::format,
        "a.html",
        "<html><head></head><body><p>hi</p></body></html>",
    );
}

#[test]
fn idempotent_makefile() {
    assert_idempotent(
        makefile::format,
        "Makefile",
        "all: dep1 dep2\n\techo hello\n",
    );
}

#[test]
fn idempotent_nginx() {
    assert_idempotent(nginx::format, "nginx.conf", "server {\nlisten 80;\n}\n");
}

#[test]
fn idempotent_nginx_nested() {
    assert_idempotent(
        nginx::format,
        "nginx.conf",
        "server {\nlisten 80;\nlocation / {\nreturn 200;\n}\n}\n",
    );
}

#[test]
fn idempotent_protobuf() {
    assert_idempotent(
        protobuf::format,
        "a.proto",
        "syntax = \"proto3\";\nmessage A {\n  int32 x = 1;\n}\n",
    );
}

#[test]
fn idempotent_rlang() {
    assert_idempotent(rlang::format, "a.r", "x <- 1\nif (TRUE) {\n  print(x)\n}\n");
}

#[test]
fn idempotent_markdown() {
    assert_idempotent(markdown::format, "a.md", "# Title\n\nSome text here.\n");
}

#[test]
fn idempotent_markdown_with_code_block() {
    assert_idempotent(
        markdown::format,
        "a.md",
        "# Title\n\n```json\n{\n  \"a\": 1\n}\n```\n",
    );
}

// ──────────────────────────────────────────────
// 已格式化的輸入應回傳 None
// ──────────────────────────────────────────────

#[test]
fn already_formatted_json_returns_none() {
    let formatted = "{\n  \"a\": 1\n}\n";
    let result = json::format(Path::new("a.json"), formatted).unwrap();
    assert!(
        result.is_none(),
        "Already formatted JSON should return None, got: {:?}",
        result
    );
}

#[test]
fn already_formatted_yaml_returns_none() {
    let formatted = "foo: 1\nbar: 2\n";
    let result = yaml::format(Path::new("a.yaml"), formatted).unwrap();
    assert!(
        result.is_none(),
        "Already formatted YAML should return None, got: {:?}",
        result
    );
}

#[test]
fn already_formatted_bash_returns_none() {
    let formatted = "#!/bin/bash\necho hello\n";
    let result = bash::format(Path::new("a.sh"), formatted).unwrap();
    assert!(
        result.is_none(),
        "Already formatted bash should return None, got: {:?}",
        result
    );
}

#[test]
fn already_formatted_toml_returns_none() {
    let formatted = "[package]\nname = \"test\"\n";
    let result = toml_fmt::format(Path::new("a.toml"), formatted).unwrap();
    assert!(
        result.is_none(),
        "Already formatted TOML should return None, got: {:?}",
        result
    );
}
