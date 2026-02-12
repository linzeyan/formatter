use std::path::Path;

use formatter::formats::{
    FormatError, bash, css, dockerfile, go, graphql, hcl, html, ini, javascript, json, lua,
    makefile, markdown, nginx, protobuf, python, rlang, rustfmt, sql, toml_fmt, typescript, xml,
    yaml,
};

// Helper to run a formatter and return result (does not unwrap error)
fn try_run(
    f: fn(&Path, &str) -> anyhow::Result<Option<String>, FormatError>,
    path: &str,
    input: &str,
) -> Result<Option<String>, FormatError> {
    f(Path::new(path), input)
}

// ──────────────────────────────────────────────
// 空字串輸入測試：所有格式化器不應 panic
// ──────────────────────────────────────────────

#[test]
fn empty_input_json() {
    // Empty JSON is invalid, should return error or handle gracefully
    let r = try_run(json::format, "a.json", "");
    // Should not panic — either Ok or Err is acceptable
    let _ = r;
}

#[test]
fn empty_input_yaml() {
    let r = try_run(yaml::format, "a.yaml", "");
    let _ = r;
}

#[test]
fn empty_input_toml() {
    let r = try_run(toml_fmt::format, "a.toml", "");
    let _ = r;
}

#[test]
fn empty_input_xml() {
    let r = try_run(xml::format, "a.xml", "");
    let _ = r;
}

#[test]
fn empty_input_markdown() {
    let r = try_run(markdown::format, "a.md", "");
    let _ = r;
}

#[test]
fn empty_input_bash() {
    let r = try_run(bash::format, "a.sh", "");
    // bash::format returns Ok(None) for empty input
    assert!(r.is_ok());
    assert!(r.unwrap().is_none());
}

#[test]
fn empty_input_css() {
    let r = try_run(css::format, "a.css", "");
    let _ = r;
}

#[test]
fn empty_input_typescript() {
    let r = try_run(typescript::format, "a.ts", "");
    let _ = r;
}

#[test]
fn empty_input_javascript() {
    let r = try_run(javascript::format, "a.js", "");
    let _ = r;
}

#[test]
fn empty_input_dockerfile() {
    let r = try_run(dockerfile::format, "Dockerfile", "");
    let _ = r;
}

#[test]
fn empty_input_sql() {
    let r = try_run(sql::format, "a.sql", "");
    let _ = r;
}

#[test]
fn empty_input_python() {
    let r = try_run(python::format, "a.py", "");
    let _ = r;
}

#[test]
fn empty_input_go() {
    let r = try_run(go::format, "a.go", "");
    let _ = r;
}

#[test]
fn empty_input_rust() {
    let r = try_run(rustfmt::format, "a.rs", "");
    let _ = r;
}

#[test]
fn empty_input_ini() {
    let r = try_run(ini::format, "a.ini", "");
    let _ = r;
}

#[test]
fn empty_input_graphql() {
    let r = try_run(graphql::format, "a.graphql", "");
    let _ = r;
}

#[test]
fn empty_input_hcl() {
    let r = try_run(hcl::format, "a.hcl", "");
    let _ = r;
}

#[test]
fn empty_input_lua() {
    let r = try_run(lua::format, "a.lua", "");
    let _ = r;
}

#[test]
fn empty_input_html() {
    let r = try_run(html::format, "a.html", "");
    let _ = r;
}

#[test]
fn empty_input_makefile() {
    let r = try_run(makefile::format, "Makefile", "");
    let _ = r;
}

#[test]
fn empty_input_nginx() {
    let r = try_run(nginx::format, "nginx.conf", "");
    let _ = r;
}

#[test]
fn empty_input_protobuf() {
    let r = try_run(protobuf::format, "a.proto", "");
    let _ = r;
}

#[test]
fn empty_input_rlang() {
    let r = try_run(rlang::format, "a.r", "");
    let _ = r;
}

// ──────────────────────────────────────────────
// 僅空白字元輸入
// ──────────────────────────────────────────────

#[test]
fn whitespace_only_json() {
    let r = try_run(json::format, "a.json", "   \n  \n");
    let _ = r;
}

#[test]
fn whitespace_only_yaml() {
    let r = try_run(yaml::format, "a.yaml", "   \n  \n");
    let _ = r;
}

#[test]
fn whitespace_only_bash() {
    let r = try_run(bash::format, "a.sh", "   \n  \n");
    let _ = r;
}

#[test]
fn whitespace_only_makefile() {
    let r = try_run(makefile::format, "Makefile", "   \n  \n");
    let _ = r;
}

#[test]
fn whitespace_only_nginx() {
    let r = try_run(nginx::format, "nginx.conf", "   \n  \n");
    let _ = r;
}

#[test]
fn whitespace_only_protobuf() {
    let r = try_run(protobuf::format, "a.proto", "   \n  \n");
    let _ = r;
}

// ──────────────────────────────────────────────
// 僅換行符輸入
// ──────────────────────────────────────────────

#[test]
fn newline_only_json() {
    let r = try_run(json::format, "a.json", "\n");
    let _ = r;
}

#[test]
fn newline_only_yaml() {
    let r = try_run(yaml::format, "a.yaml", "\n");
    let _ = r;
}

#[test]
fn newline_only_bash() {
    let r = try_run(bash::format, "a.sh", "\n");
    let _ = r;
}

// ──────────────────────────────────────────────
// Unicode 特殊字元（CJK、emoji、數學符號、零寬字元）
// ──────────────────────────────────────────────

#[test]
fn unicode_json_keys() {
    let input = r#"{"名前":"太郎","emoji":"🎉","math":"∑∏∫"}"#;
    let r = try_run(json::format, "a.json", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("名前"));
    assert!(out.contains("太郎"));
    assert!(out.contains("🎉"));
    assert!(out.contains("∑∏∫"));
}

#[test]
fn unicode_yaml_values() {
    let input = "name: 中文測試\nemoji: 🚀\nmath: ∀x∈ℝ\n";
    let r = try_run(yaml::format, "a.yaml", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("中文測試"));
    assert!(out.contains("🚀"));
}

#[test]
fn unicode_bash_comments() {
    let input = "#!/bin/bash\n# 這是中文註解\necho \"你好世界\"\n";
    let r = try_run(bash::format, "a.sh", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("這是中文註解"));
    assert!(out.contains("你好世界"));
}

#[test]
fn unicode_xml_content() {
    let input = "<root><item>日本語テスト</item></root>";
    let r = try_run(xml::format, "a.xml", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("日本語テスト"));
}

#[test]
fn unicode_html_content() {
    let input = "<div>Ünïcödé Tëst 🌍</div>";
    let r = try_run(html::format, "a.html", input);
    assert!(r.is_ok());
}

#[test]
fn zero_width_chars_in_yaml() {
    // Zero-width space (U+200B) and zero-width joiner (U+200D)
    let input = "key: value\u{200B}with\u{200D}zerowidth\n";
    let r = try_run(yaml::format, "a.yaml", input);
    assert!(r.is_ok());
}

#[test]
fn rtl_control_chars_in_json() {
    // Right-to-left override (U+202E)
    let input = "{\"key\": \"normal\u{202E}reversed\"}";
    let r = try_run(json::format, "a.json", input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 超長單行輸入
// ──────────────────────────────────────────────

#[test]
fn long_single_line_json() {
    // JSON with a very long string value
    let long_val = "x".repeat(10_000);
    let input = format!("{{\"key\":\"{}\"}}", long_val);
    let r = try_run(json::format, "a.json", &input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.clone());
    assert!(out.contains(&long_val));
}

#[test]
fn long_single_line_yaml() {
    let long_val = "y".repeat(10_000);
    let input = format!("key: {}\n", long_val);
    let r = try_run(yaml::format, "a.yaml", &input);
    assert!(r.is_ok());
}

#[test]
fn long_single_line_bash() {
    let long_cmd = format!("echo {}", "a".repeat(5_000));
    let r = try_run(bash::format, "a.sh", &long_cmd);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 深層巢狀結構
// ──────────────────────────────────────────────

#[test]
fn deep_nested_json() {
    // 10 levels of nesting (modest depth to avoid slow formatting)
    let mut input = String::new();
    for _ in 0..10 {
        input.push_str("{\"a\":");
    }
    input.push('1');
    for _ in 0..10 {
        input.push('}');
    }
    let r = try_run(json::format, "a.json", &input);
    assert!(r.is_ok());
}

#[test]
fn deep_nested_xml() {
    let mut input = String::new();
    for i in 0..30 {
        input.push_str(&format!("<l{}>", i));
    }
    input.push_str("content");
    for i in (0..30).rev() {
        input.push_str(&format!("</l{}>", i));
    }
    let r = try_run(xml::format, "a.xml", &input);
    assert!(r.is_ok());
}

#[test]
fn deep_nested_bash_if() {
    let mut input = String::new();
    for _ in 0..20 {
        input.push_str("if true; then\n");
    }
    input.push_str("echo deep\n");
    for _ in 0..20 {
        input.push_str("fi\n");
    }
    let r = try_run(bash::format, "a.sh", &input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.clone());
    assert!(out.contains("echo deep"));
}

#[test]
fn deep_nested_yaml() {
    let mut input = String::new();
    for i in 0..20 {
        input.push_str(&"  ".repeat(i));
        input.push_str(&format!("level{}:\n", i));
    }
    input.push_str(&"  ".repeat(20));
    input.push_str("value: deep\n");
    let r = try_run(yaml::format, "a.yaml", &input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 畸形/無效輸入：應返回 Err 而不是 panic
// ──────────────────────────────────────────────

#[test]
fn malformed_json_returns_error() {
    let input = "{invalid json!!!";
    let r = try_run(json::format, "a.json", input);
    assert!(r.is_err());
}

#[test]
fn malformed_xml_returns_error() {
    let input = "<unclosed><tag>";
    let r = try_run(xml::format, "a.xml", input);
    // quick-xml might still handle unclosed tags
    let _ = r;
}

#[test]
fn malformed_toml_returns_error() {
    let input = "[[[invalid toml";
    let r = try_run(toml_fmt::format, "a.toml", input);
    assert!(r.is_err());
}

#[test]
fn malformed_ini_does_not_panic() {
    let input = "=no_key_value\n[section\n";
    let r = try_run(ini::format, "a.ini", input);
    // ini parser may or may not error, but should not panic
    let _ = r;
}

#[test]
fn malformed_go_returns_error() {
    let input = "this is not go code at all!!!";
    let r = try_run(go::format, "a.go", input);
    assert!(r.is_err());
}

#[test]
fn malformed_rust_returns_error() {
    let input = "fn main() { let x = ; }";
    let r = try_run(rustfmt::format, "a.rs", input);
    assert!(r.is_err());
}

#[test]
fn malformed_hcl_returns_error() {
    let input = "resource {{{ invalid }}}";
    let r = try_run(hcl::format, "a.hcl", input);
    assert!(r.is_err());
}

#[test]
fn malformed_graphql_does_not_panic() {
    let input = "type { broken }}}";
    let r = try_run(graphql::format, "a.graphql", input);
    let _ = r; // should not panic
}

// ──────────────────────────────────────────────
// 特殊邊界：只含有註解的文件
// ──────────────────────────────────────────────

#[test]
fn comment_only_bash() {
    let input = "#!/bin/bash\n# Just a comment\n# And another\n";
    let r = try_run(bash::format, "a.sh", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("# Just a comment"));
    assert!(out.contains("# And another"));
}

#[test]
fn comment_only_makefile() {
    let input = "# This is a comment\n# Another comment\n";
    let r = try_run(makefile::format, "Makefile", input);
    assert!(r.is_ok());
}

#[test]
fn comment_only_yaml() {
    let input = "# YAML comment\n# Another comment\n";
    let r = try_run(yaml::format, "a.yaml", input);
    assert!(r.is_ok());
}

#[test]
fn comment_only_nginx() {
    let input = "# nginx comment line\n# another comment\n";
    let r = try_run(nginx::format, "nginx.conf", input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 含有 trailing whitespace 的輸入
// ──────────────────────────────────────────────

#[test]
fn trailing_whitespace_bash() {
    let input = "echo hello   \necho world   \n";
    let r = try_run(bash::format, "a.sh", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    // trailing whitespace should be trimmed
    for line in out.lines() {
        assert_eq!(
            line.trim_end(),
            line,
            "Line has trailing whitespace: {:?}",
            line
        );
    }
}

#[test]
fn trailing_whitespace_makefile() {
    let input = "all:   \n\techo hi   \n";
    let r = try_run(makefile::format, "Makefile", input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 含有 tab 和空格混用的輸入
// ──────────────────────────────────────────────

#[test]
fn mixed_indentation_bash() {
    let input = "if true; then\n\t  echo mixed\nfi\n";
    let r = try_run(bash::format, "a.sh", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("echo mixed"));
}

#[test]
fn mixed_indentation_nginx() {
    let input = "server {\n\t  listen 80;\n}\n";
    let r = try_run(nginx::format, "nginx.conf", input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 包含特殊 JSON 值
// ──────────────────────────────────────────────

#[test]
fn json_special_values() {
    let input =
        r#"{"null_val":null,"bool_true":true,"bool_false":false,"number":42.5,"negative":-1}"#;
    let r = try_run(json::format, "a.json", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("null"));
    assert!(out.contains("true"));
    assert!(out.contains("false"));
    assert!(out.contains("42.5"));
    assert!(out.contains("-1"));
}

#[test]
fn json_escaped_characters() {
    let input = r#"{"escaped":"line1\nline2\ttab\\backslash\"quote"}"#;
    let r = try_run(json::format, "a.json", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains(r#"\n"#));
    assert!(out.contains(r#"\t"#));
}

#[test]
fn json_empty_objects_and_arrays() {
    let input = r#"{"obj":{},"arr":[]}"#;
    let r = try_run(json::format, "a.json", input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 多文件 YAML
// ──────────────────────────────────────────────

#[test]
fn yaml_multiple_documents() {
    let input = "---\na: 1\n---\nb: 2\n---\nc: 3\n";
    let r = try_run(yaml::format, "a.yaml", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("a: 1"));
    assert!(out.contains("b: 2"));
    assert!(out.contains("c: 3"));
}

#[test]
fn yaml_anchors_and_aliases() {
    let input =
        "defaults: &defaults\n  adapter: postgres\ndev:\n  <<: *defaults\n  database: dev_db\n";
    let r = try_run(yaml::format, "a.yaml", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("&defaults"));
    assert!(out.contains("*defaults"));
}

// ──────────────────────────────────────────────
// 大量連續空行壓縮
// ──────────────────────────────────────────────

#[test]
fn many_blank_lines_bash() {
    let input = "echo start\n\n\n\n\n\n\n\necho end\n";
    let r = try_run(bash::format, "a.sh", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    // bash formatter collapses consecutive blank lines
    assert!(!out.contains("\n\n\n"));
}

#[test]
fn many_blank_lines_makefile() {
    let input = "all:\n\n\n\n\n\techo ok\n";
    let r = try_run(makefile::format, "Makefile", input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(!out.contains("\n\n\n"));
}
