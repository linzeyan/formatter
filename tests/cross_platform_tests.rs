use std::path::Path;

use formatter::formats::{
    FormatError, FormatKind, bash, css, detect_kind, dockerfile, ini, javascript, json, makefile,
    markdown, nginx, protobuf, rlang, sql, toml_fmt, typescript, xml, yaml,
};

fn run(
    f: fn(&Path, &str) -> anyhow::Result<Option<String>, FormatError>,
    path: &str,
    input: &str,
) -> String {
    f(Path::new(path), input)
        .unwrap()
        .unwrap_or_else(|| input.to_string())
}

// ──────────────────────────────────────────────
// CRLF (\r\n) vs LF (\n) 處理
// ──────────────────────────────────────────────

#[test]
fn yaml_crlf_detection_and_preservation() {
    let input = "foo: 1\r\nbar: 2\r\n";
    let out = run(yaml::format, "a.yaml", input);
    // YAML formatter should detect CRLF and preserve it
    assert!(
        out.contains("\r\n"),
        "CRLF should be preserved in YAML output"
    );
}

#[test]
fn yaml_lf_stays_lf() {
    let input = "foo: 1\nbar: 2\n";
    let out = run(yaml::format, "a.yaml", input);
    assert!(!out.contains("\r\n"), "LF-only input should not gain CRLF");
}

#[test]
fn json_crlf_input() {
    let input = "{\r\n\"a\": 1\r\n}";
    let r = json::format(Path::new("a.json"), input);
    assert!(r.is_ok());
}

#[test]
fn bash_crlf_input() {
    let input = "echo hello\r\necho world\r\n";
    let r = bash::format(Path::new("a.sh"), input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("echo hello"));
    assert!(out.contains("echo world"));
}

#[test]
fn xml_crlf_input() {
    let input = "<root>\r\n  <item>1</item>\r\n</root>";
    let r = xml::format(Path::new("a.xml"), input);
    assert!(r.is_ok());
}

#[test]
fn toml_crlf_input() {
    let input = "[section]\r\nkey = \"value\"\r\n";
    let r = toml_fmt::format(Path::new("a.toml"), input);
    assert!(r.is_ok());
}

#[test]
fn makefile_crlf_input() {
    let input = "all:\r\n\techo hi\r\n";
    let r = makefile::format(Path::new("Makefile"), input);
    assert!(r.is_ok());
}

#[test]
fn nginx_crlf_input() {
    let input = "server {\r\n    listen 80;\r\n}\r\n";
    let r = nginx::format(Path::new("nginx.conf"), input);
    assert!(r.is_ok());
}

#[test]
fn protobuf_crlf_input() {
    let input = "message A {\r\n  int32 x = 1;\r\n}\r\n";
    let r = protobuf::format(Path::new("a.proto"), input);
    assert!(r.is_ok());
}

#[test]
fn ini_crlf_input() {
    let input = "[section]\r\nkey=value\r\n";
    let r = ini::format(Path::new("a.ini"), input);
    assert!(r.is_ok());
}

#[test]
fn css_crlf_input() {
    let input = "h1 {\r\n  color: red;\r\n}\r\n";
    let r = css::format(Path::new("a.css"), input);
    assert!(r.is_ok());
}

#[test]
fn typescript_crlf_input() {
    let input = "const  x = 1;\r\n";
    let r = typescript::format(Path::new("a.ts"), input);
    assert!(r.is_ok());
}

#[test]
fn javascript_crlf_input() {
    let input = "const  a = 1;\r\n";
    let r = javascript::format(Path::new("a.js"), input);
    assert!(r.is_ok());
}

#[test]
fn rlang_crlf_input() {
    let input = "x <- 1\r\nprint(x)\r\n";
    let r = rlang::format(Path::new("a.r"), input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 混合換行符處理
// ──────────────────────────────────────────────

#[test]
fn mixed_line_endings_yaml() {
    let input = "a: 1\r\nb: 2\nc: 3\r\n";
    let r = yaml::format(Path::new("a.yaml"), input);
    assert!(r.is_ok());
}

#[test]
fn mixed_line_endings_bash() {
    let input = "echo 1\r\necho 2\necho 3\r\n";
    let r = bash::format(Path::new("a.sh"), input);
    assert!(r.is_ok());
}

#[test]
fn mixed_line_endings_json() {
    let input = "{\r\n\"a\": 1,\n\"b\": 2\r\n}";
    let r = json::format(Path::new("a.json"), input);
    assert!(r.is_ok());
}

// ──────────────────────────────────────────────
// 路徑格式偵測（跨平台）
// ──────────────────────────────────────────────

#[test]
fn detect_kind_unix_paths() {
    assert_eq!(
        detect_kind(Path::new("/home/user/project/file.json")),
        Some(FormatKind::Json)
    );
    assert_eq!(
        detect_kind(Path::new("/etc/nginx/nginx.conf")),
        Some(FormatKind::Nginx)
    );
    assert_eq!(
        detect_kind(Path::new("/app/Dockerfile")),
        Some(FormatKind::Dockerfile)
    );
}

#[test]
fn detect_kind_relative_paths() {
    assert_eq!(
        detect_kind(Path::new("./src/main.rs")),
        Some(FormatKind::Rust)
    );
    assert_eq!(
        detect_kind(Path::new("../config.yaml")),
        Some(FormatKind::Yaml)
    );
}

#[test]
fn detect_kind_deep_nested_path() {
    assert_eq!(
        detect_kind(Path::new("a/b/c/d/e/f/g/h/i/j/file.py")),
        Some(FormatKind::Python)
    );
}

// ──────────────────────────────────────────────
// 文件結尾換行一致性（跨平台關鍵）
// ──────────────────────────────────────────────

#[test]
fn output_ends_with_lf_json() {
    let out = run(json::format, "a.json", "{\"a\":1}");
    assert!(out.ends_with('\n'), "JSON output should end with LF");
}

#[test]
fn output_ends_with_lf_toml() {
    let out = run(toml_fmt::format, "a.toml", "[a]\nb=1");
    assert!(out.ends_with('\n'), "TOML output should end with LF");
}

#[test]
fn output_ends_with_lf_xml() {
    let out = run(xml::format, "a.xml", "<a><b>1</b></a>");
    assert!(out.ends_with('\n'), "XML output should end with LF");
}

#[test]
fn output_ends_with_lf_bash() {
    let out = run(bash::format, "a.sh", "echo hi");
    assert!(out.ends_with('\n'), "Bash output should end with LF");
}

#[test]
fn output_ends_with_lf_css() {
    let out = run(css::format, "a.css", "h1{color:red;}");
    assert!(out.ends_with('\n'), "CSS output should end with LF");
}

#[test]
fn output_ends_with_lf_typescript() {
    let out = run(typescript::format, "a.ts", "const x=1");
    assert!(out.ends_with('\n'), "TypeScript output should end with LF");
}

#[test]
fn output_ends_with_lf_javascript() {
    let out = run(javascript::format, "a.js", "const a=1");
    assert!(out.ends_with('\n'), "JavaScript output should end with LF");
}

#[test]
fn output_ends_with_lf_sql() {
    let out = run(sql::format, "a.sql", "select 1");
    assert!(out.ends_with('\n'), "SQL output should end with LF");
}

#[test]
fn output_ends_with_lf_markdown() {
    let out = run(markdown::format, "a.md", "# Title\ntext");
    assert!(out.ends_with('\n'), "Markdown output should end with LF");
}

#[test]
fn output_ends_with_lf_dockerfile() {
    let out = run(
        dockerfile::format,
        "Dockerfile",
        "FROM alpine\nRUN echo hi\n",
    );
    assert!(out.ends_with('\n'), "Dockerfile output should end with LF");
}

#[test]
fn output_ends_with_lf_ini() {
    let out = run(ini::format, "a.ini", "[s]\nk=v");
    assert!(out.ends_with('\n'), "INI output should end with LF");
}

#[test]
fn output_ends_with_lf_protobuf() {
    let out = run(
        protobuf::format,
        "a.proto",
        "message A {\n  int32 x = 1;\n}",
    );
    assert!(out.ends_with('\n'), "Protobuf output should end with LF");
}

#[test]
fn output_ends_with_lf_makefile() {
    let out = run(makefile::format, "Makefile", "all:\n\techo hi");
    assert!(out.ends_with('\n'), "Makefile output should end with LF");
}

#[test]
fn output_ends_with_lf_nginx() {
    let out = run(nginx::format, "nginx.conf", "server {\n  listen 80;\n}");
    assert!(out.ends_with('\n'), "Nginx output should end with LF");
}

#[test]
fn output_ends_with_lf_rlang() {
    let out = run(rlang::format, "a.r", "x <- 1");
    assert!(out.ends_with('\n'), "R output should end with LF");
}
