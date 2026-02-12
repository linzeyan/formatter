use std::path::Path;

use formatter::formats::{
    FormatError, FormatKind, bash, css, dockerfile, format_dispatch, ini, javascript, json,
    makefile, markdown, nginx, protobuf, toml_fmt, typescript, xml, yaml,
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

// ══════════════════════════════════════════════
// format_dispatch 路由完整性：每種 FormatKind 都正確路由
// ══════════════════════════════════════════════

#[test]
fn dispatch_json_matches_direct_call() {
    let input = "{\"a\":1}";
    let direct = json::format(Path::new("a.json"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::Json, Path::new("a.json"), input).unwrap();
    assert_eq!(direct, dispatched);
}

#[test]
fn dispatch_yaml_matches_direct_call() {
    let input = "a: 1\n";
    let direct = yaml::format(Path::new("a.yaml"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::Yaml, Path::new("a.yaml"), input).unwrap();
    assert_eq!(direct, dispatched);
}

#[test]
fn dispatch_toml_matches_direct_call() {
    let input = "[a]\nb=1\n";
    let direct = toml_fmt::format(Path::new("a.toml"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::Toml, Path::new("a.toml"), input).unwrap();
    assert_eq!(direct, dispatched);
}

#[test]
fn dispatch_bash_matches_direct_call() {
    let input = "echo hello\n";
    let direct = bash::format(Path::new("a.sh"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::Bash, Path::new("a.sh"), input).unwrap();
    assert_eq!(direct, dispatched);
}

#[test]
fn dispatch_xml_matches_direct_call() {
    let input = "<a><b>1</b></a>";
    let direct = xml::format(Path::new("a.xml"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::Xml, Path::new("a.xml"), input).unwrap();
    assert_eq!(direct, dispatched);
}

#[test]
fn dispatch_css_matches_direct_call() {
    let input = "h1{color:red;}";
    let direct = css::format(Path::new("a.css"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::Css, Path::new("a.css"), input).unwrap();
    assert_eq!(direct, dispatched);
}

#[test]
fn dispatch_typescript_matches_direct_call() {
    let input = "const  x=1";
    let direct = typescript::format(Path::new("a.ts"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::TypeScript, Path::new("a.ts"), input).unwrap();
    assert_eq!(direct, dispatched);
}

#[test]
fn dispatch_javascript_matches_direct_call() {
    let input = "const  a=1";
    let direct = javascript::format(Path::new("a.js"), input).unwrap();
    let dispatched = format_dispatch(FormatKind::JavaScript, Path::new("a.js"), input).unwrap();
    assert_eq!(direct, dispatched);
}

// ══════════════════════════════════════════════
// JavaScript 委託 TypeScript：確保不互相影響
// ══════════════════════════════════════════════

#[test]
fn js_delegates_to_ts_same_formatting() {
    // JavaScript should produce same formatting as TypeScript for pure JS code
    let input = "const  x=1";
    let ts_out = run(typescript::format, "a.ts", input);
    let js_out = run(javascript::format, "a.js", input);
    // Both should normalize the spacing
    assert!(ts_out.contains("const x = 1;"));
    assert!(js_out.contains("const x = 1;"));
}

#[test]
fn ts_specific_syntax_not_broken_by_js() {
    // TypeScript-specific syntax should work fine
    let ts_input = "const x:number=1";
    let out = run(typescript::format, "a.ts", ts_input);
    assert!(out.contains("const x: number = 1;"));

    // JavaScript code should still work independently
    let js_input = "const y=2";
    let out = run(javascript::format, "a.js", js_input);
    assert!(out.contains("const y = 2;"));
}

// ══════════════════════════════════════════════
// Markdown 內嵌程式碼：不影響外部 Markdown 結構
// ══════════════════════════════════════════════

#[test]
fn markdown_code_block_formatting_preserves_surrounding_text() {
    let input = "# Title\n\nParagraph before.\n\n```json\n{\"a\":1}\n```\n\nParagraph after.\n";
    let out = run(markdown::format, "a.md", input);
    assert!(out.contains("# Title"));
    assert!(out.contains("Paragraph before."));
    assert!(out.contains("Paragraph after."));
    assert!(out.contains("\"a\": 1")); // JSON was formatted
}

#[test]
fn markdown_inline_code_not_formatted() {
    let input = "Use `{\"a\":1}` for config.\n";
    let out = run(markdown::format, "a.md", input);
    // Inline code should NOT be formatted (only fenced code blocks)
    assert!(out.contains("`{\"a\":1}`"));
}

#[test]
fn markdown_code_block_error_does_not_crash() {
    // Malformed code in block should not crash the markdown formatter
    let input = "```json\n{invalid json!!!}\n```\n";
    let r = markdown::format(Path::new("a.md"), input);
    // Should not panic; either Ok or handled error
    let _ = r;
}

// ══════════════════════════════════════════════
// Dockerfile RUN 區塊：bash 格式化不影響 Dockerfile 語法
// ══════════════════════════════════════════════

#[test]
fn dockerfile_non_run_lines_unchanged() {
    let input = "FROM alpine:3.18\nLABEL maintainer=\"test@test.com\"\nEXPOSE 8080\nCMD [\"echo\", \"hello\"]\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    assert!(out.contains("FROM alpine:3.18"));
    assert!(out.contains("EXPOSE 8080"));
    assert!(out.contains("CMD"));
}

#[test]
fn dockerfile_run_formatting_doesnt_break_dockerfile_structure() {
    let input = "FROM alpine\nRUN echo hello&&echo world\nEXPOSE 8080\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    // Non-RUN lines should be preserved
    assert!(out.contains("FROM alpine"));
    assert!(out.contains("EXPOSE 8080"));
    // RUN should be present (may be reformatted)
    assert!(out.contains("RUN"));
}

#[test]
fn dockerfile_multiple_run_blocks() {
    let input = "FROM alpine\nRUN echo first\nRUN echo second\nRUN echo third\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    // All RUN commands should be present
    let run_count = out.matches("RUN").count();
    assert!(
        run_count >= 3,
        "Expected at least 3 RUN commands, got {}",
        run_count
    );
}

// ══════════════════════════════════════════════
// INI formatter：不影響 section 結構
// ══════════════════════════════════════════════

#[test]
fn ini_preserves_all_sections() {
    let input = "[section1]\nkey1=val1\n\n[section2]\nkey2=val2\n\n[section3]\nkey3=val3\n";
    let out = run(ini::format, "a.ini", input);
    assert!(out.contains("[section1]"));
    assert!(out.contains("[section2]"));
    assert!(out.contains("[section3]"));
}

#[test]
fn ini_preserves_all_keys() {
    let input = "[db]\nhost=localhost\nport=5432\nname=mydb\nuser=admin\n";
    let out = run(ini::format, "a.ini", input);
    assert!(out.contains("host"));
    assert!(out.contains("port"));
    assert!(out.contains("name"));
    assert!(out.contains("user"));
}

// ══════════════════════════════════════════════
// 交叉驗證：格式化器 A 的輸出不影響格式化器 B 的行為
// ══════════════════════════════════════════════

#[test]
fn json_and_yaml_independent() {
    // Format JSON first, then YAML — they should work independently
    let json_input = "{\"a\":1}";
    let yaml_input = "b: 2\n";

    let json_out = run(json::format, "a.json", json_input);
    let yaml_out = run(yaml::format, "a.yaml", yaml_input);

    assert!(json_out.contains("\"a\": 1"));
    assert!(yaml_out.contains("b: 2"));
}

#[test]
fn bash_and_makefile_independent() {
    // Bash and Makefile use different indentation rules
    let bash_input = "if true; then\necho hi\nfi\n";
    let make_input = "all:\n\techo hi\n";

    let bash_out = run(bash::format, "a.sh", bash_input);
    let make_out = run(makefile::format, "Makefile", make_input);

    // Bash uses spaces
    assert!(bash_out.contains("  echo hi"));
    // Makefile uses tabs
    assert!(make_out.contains("\techo hi"));
}

#[test]
fn nginx_and_protobuf_independent() {
    // Both use brace-based formatting but different indent sizes
    let nginx_input = "server {\nlisten 80;\n}\n";
    let proto_input = "message A {\nint32 x=1;\n}\n";

    let nginx_out = run(nginx::format, "nginx.conf", nginx_input);
    let proto_out = run(protobuf::format, "a.proto", proto_input);

    // Nginx uses 4-space indent
    assert!(nginx_out.contains("    listen 80;"));
    // Protobuf uses 2-space indent
    assert!(proto_out.contains("  int32 x = 1;"));
}

// ══════════════════════════════════════════════
// 並行安全性：多個格式化器可以並行執行
// ══════════════════════════════════════════════

#[test]
fn parallel_formatting_safety() {
    use std::thread;

    let handles: Vec<_> = vec![
        thread::spawn(|| {
            for _ in 0..10 {
                let _ = json::format(Path::new("a.json"), "{\"a\":1}");
            }
        }),
        thread::spawn(|| {
            for _ in 0..10 {
                let _ = yaml::format(Path::new("a.yaml"), "a: 1\n");
            }
        }),
        thread::spawn(|| {
            for _ in 0..10 {
                let _ = bash::format(Path::new("a.sh"), "echo hi\n");
            }
        }),
        thread::spawn(|| {
            for _ in 0..10 {
                let _ = css::format(Path::new("a.css"), "h1{color:red;}");
            }
        }),
        thread::spawn(|| {
            for _ in 0..10 {
                let _ = toml_fmt::format(Path::new("a.toml"), "[a]\nb=1\n");
            }
        }),
    ];

    for handle in handles {
        handle
            .join()
            .expect("Thread panicked during parallel formatting");
    }
}

// ══════════════════════════════════════════════
// FormatError 轉換測試
// ══════════════════════════════════════════════

#[test]
fn format_error_from_anyhow() {
    let anyhow_err = anyhow::anyhow!("test error");
    let format_err: FormatError = anyhow_err.into();
    assert_eq!(format_err.to_string(), "test error");
}

#[test]
fn format_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let format_err: FormatError = io_err.into();
    assert!(format_err.to_string().contains("file not found"));
}

#[test]
fn format_error_display() {
    let err = FormatError::Message("custom error message".to_string());
    assert_eq!(err.to_string(), "custom error message");
}
