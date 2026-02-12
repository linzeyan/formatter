use std::path::Path;

use formatter::formats::{FormatKind, detect_kind, detect_kind_from_label};

// ──────────────────────────────────────────────
// detect_kind_from_label: 完整標籤映射測試
// ──────────────────────────────────────────────

#[test]
fn label_json() {
    assert_eq!(detect_kind_from_label("json"), Some(FormatKind::Json));
    assert_eq!(detect_kind_from_label("JSON"), Some(FormatKind::Json));
    assert_eq!(detect_kind_from_label("Json"), Some(FormatKind::Json));
}

#[test]
fn label_yaml() {
    assert_eq!(detect_kind_from_label("yaml"), Some(FormatKind::Yaml));
    assert_eq!(detect_kind_from_label("yml"), Some(FormatKind::Yaml));
    assert_eq!(detect_kind_from_label("YML"), Some(FormatKind::Yaml));
}

#[test]
fn label_toml() {
    assert_eq!(detect_kind_from_label("toml"), Some(FormatKind::Toml));
    assert_eq!(detect_kind_from_label("TOML"), Some(FormatKind::Toml));
}

#[test]
fn label_xml() {
    assert_eq!(detect_kind_from_label("xml"), Some(FormatKind::Xml));
}

#[test]
fn label_markdown() {
    assert_eq!(detect_kind_from_label("md"), Some(FormatKind::Markdown));
    assert_eq!(
        detect_kind_from_label("markdown"),
        Some(FormatKind::Markdown)
    );
    assert_eq!(
        detect_kind_from_label("MARKDOWN"),
        Some(FormatKind::Markdown)
    );
}

#[test]
fn label_bash() {
    assert_eq!(detect_kind_from_label("bash"), Some(FormatKind::Bash));
    assert_eq!(detect_kind_from_label("sh"), Some(FormatKind::Bash));
    assert_eq!(detect_kind_from_label("shell"), Some(FormatKind::Bash));
    assert_eq!(detect_kind_from_label("SHELL"), Some(FormatKind::Bash));
}

#[test]
fn label_dockerfile() {
    assert_eq!(
        detect_kind_from_label("docker"),
        Some(FormatKind::Dockerfile)
    );
    assert_eq!(
        detect_kind_from_label("dockerfile"),
        Some(FormatKind::Dockerfile)
    );
    assert_eq!(
        detect_kind_from_label("DOCKERFILE"),
        Some(FormatKind::Dockerfile)
    );
}

#[test]
fn label_makefile() {
    assert_eq!(
        detect_kind_from_label("makefile"),
        Some(FormatKind::Makefile)
    );
    assert_eq!(detect_kind_from_label("mk"), Some(FormatKind::Makefile));
}

#[test]
fn label_ini() {
    assert_eq!(detect_kind_from_label("ini"), Some(FormatKind::Ini));
}

#[test]
fn label_nginx() {
    assert_eq!(detect_kind_from_label("nginx"), Some(FormatKind::Nginx));
}

#[test]
fn label_html() {
    assert_eq!(detect_kind_from_label("html"), Some(FormatKind::Html));
    assert_eq!(detect_kind_from_label("htm"), Some(FormatKind::Html));
}

#[test]
fn label_css() {
    assert_eq!(detect_kind_from_label("css"), Some(FormatKind::Css));
}

#[test]
fn label_typescript() {
    assert_eq!(detect_kind_from_label("ts"), Some(FormatKind::TypeScript));
    assert_eq!(detect_kind_from_label("tsx"), Some(FormatKind::TypeScript));
    assert_eq!(
        detect_kind_from_label("typescript"),
        Some(FormatKind::TypeScript)
    );
}

#[test]
fn label_javascript() {
    assert_eq!(detect_kind_from_label("js"), Some(FormatKind::JavaScript));
    assert_eq!(detect_kind_from_label("jsx"), Some(FormatKind::JavaScript));
    assert_eq!(
        detect_kind_from_label("javascript"),
        Some(FormatKind::JavaScript)
    );
}

#[test]
fn label_golang() {
    assert_eq!(detect_kind_from_label("go"), Some(FormatKind::Golang));
    assert_eq!(detect_kind_from_label("golang"), Some(FormatKind::Golang));
}

#[test]
fn label_rust() {
    assert_eq!(detect_kind_from_label("rs"), Some(FormatKind::Rust));
    assert_eq!(detect_kind_from_label("rust"), Some(FormatKind::Rust));
}

#[test]
fn label_python() {
    assert_eq!(detect_kind_from_label("py"), Some(FormatKind::Python));
    assert_eq!(detect_kind_from_label("python"), Some(FormatKind::Python));
}

#[test]
fn label_protobuf() {
    assert_eq!(detect_kind_from_label("proto"), Some(FormatKind::Protobuf));
    assert_eq!(
        detect_kind_from_label("protobuf"),
        Some(FormatKind::Protobuf)
    );
}

#[test]
fn label_graphql() {
    assert_eq!(detect_kind_from_label("gql"), Some(FormatKind::Graphql));
    assert_eq!(detect_kind_from_label("graphql"), Some(FormatKind::Graphql));
}

#[test]
fn label_hcl() {
    assert_eq!(detect_kind_from_label("hcl"), Some(FormatKind::Hcl));
    assert_eq!(detect_kind_from_label("tf"), Some(FormatKind::Hcl));
}

#[test]
fn label_lua() {
    assert_eq!(detect_kind_from_label("lua"), Some(FormatKind::Lua));
}

#[test]
fn label_r() {
    assert_eq!(detect_kind_from_label("r"), Some(FormatKind::R));
}

#[test]
fn label_sql() {
    assert_eq!(detect_kind_from_label("sql"), Some(FormatKind::Sql));
}

#[test]
fn label_unknown_returns_none() {
    assert_eq!(detect_kind_from_label("unknown"), None);
    assert_eq!(detect_kind_from_label(""), None);
    assert_eq!(detect_kind_from_label("c"), None);
    assert_eq!(detect_kind_from_label("cpp"), None);
    assert_eq!(detect_kind_from_label("java"), None);
    assert_eq!(detect_kind_from_label("ruby"), None);
    assert_eq!(detect_kind_from_label("swift"), None);
}

// ──────────────────────────────────────────────
// detect_kind: 副檔名與特殊檔名映射測試
// ──────────────────────────────────────────────

#[test]
fn detect_json_extensions() {
    assert_eq!(detect_kind(Path::new("data.json")), Some(FormatKind::Json));
    assert_eq!(
        detect_kind(Path::new("tsconfig.jsonc")),
        Some(FormatKind::Json)
    );
}

#[test]
fn detect_yaml_extensions() {
    assert_eq!(
        detect_kind(Path::new("config.yaml")),
        Some(FormatKind::Yaml)
    );
    assert_eq!(detect_kind(Path::new("config.yml")), Some(FormatKind::Yaml));
}

#[test]
fn detect_toml_extension() {
    assert_eq!(detect_kind(Path::new("Cargo.toml")), Some(FormatKind::Toml));
}

#[test]
fn detect_xml_extension() {
    assert_eq!(detect_kind(Path::new("pom.xml")), Some(FormatKind::Xml));
}

#[test]
fn detect_markdown_extensions() {
    assert_eq!(
        detect_kind(Path::new("README.md")),
        Some(FormatKind::Markdown)
    );
    assert_eq!(
        detect_kind(Path::new("doc.markdown")),
        Some(FormatKind::Markdown)
    );
}

#[test]
fn detect_bash_extensions() {
    assert_eq!(detect_kind(Path::new("build.sh")), Some(FormatKind::Bash));
    assert_eq!(detect_kind(Path::new("setup.bash")), Some(FormatKind::Bash));
}

#[test]
fn detect_dockerfile_special_name() {
    assert_eq!(
        detect_kind(Path::new("Dockerfile")),
        Some(FormatKind::Dockerfile)
    );
    // case-insensitive filename matching
    assert_eq!(
        detect_kind(Path::new("dockerfile")),
        Some(FormatKind::Dockerfile)
    );
}

#[test]
fn detect_makefile_special_name_and_extension() {
    assert_eq!(
        detect_kind(Path::new("Makefile")),
        Some(FormatKind::Makefile)
    );
    assert_eq!(
        detect_kind(Path::new("rules.mk")),
        Some(FormatKind::Makefile)
    );
}

#[test]
fn detect_nginx_special_names() {
    assert_eq!(
        detect_kind(Path::new("nginx.conf")),
        Some(FormatKind::Nginx)
    );
    assert_eq!(
        detect_kind(Path::new("server.nginx")),
        Some(FormatKind::Nginx)
    );
    assert_eq!(detect_kind(Path::new("site.conf")), Some(FormatKind::Nginx));
}

#[test]
fn detect_html_extensions() {
    assert_eq!(detect_kind(Path::new("index.html")), Some(FormatKind::Html));
    assert_eq!(detect_kind(Path::new("page.htm")), Some(FormatKind::Html));
}

#[test]
fn detect_css_extension() {
    assert_eq!(detect_kind(Path::new("style.css")), Some(FormatKind::Css));
}

#[test]
fn detect_typescript_extensions() {
    assert_eq!(
        detect_kind(Path::new("app.ts")),
        Some(FormatKind::TypeScript)
    );
    assert_eq!(
        detect_kind(Path::new("App.tsx")),
        Some(FormatKind::TypeScript)
    );
}

#[test]
fn detect_javascript_extensions() {
    assert_eq!(
        detect_kind(Path::new("app.js")),
        Some(FormatKind::JavaScript)
    );
    assert_eq!(
        detect_kind(Path::new("App.jsx")),
        Some(FormatKind::JavaScript)
    );
    assert_eq!(
        detect_kind(Path::new("module.mjs")),
        Some(FormatKind::JavaScript)
    );
    assert_eq!(
        detect_kind(Path::new("module.cjs")),
        Some(FormatKind::JavaScript)
    );
}

#[test]
fn detect_go_extension() {
    assert_eq!(detect_kind(Path::new("main.go")), Some(FormatKind::Golang));
}

#[test]
fn detect_rust_extension() {
    assert_eq!(detect_kind(Path::new("lib.rs")), Some(FormatKind::Rust));
}

#[test]
fn detect_python_extension() {
    assert_eq!(detect_kind(Path::new("app.py")), Some(FormatKind::Python));
}

#[test]
fn detect_protobuf_extension() {
    assert_eq!(
        detect_kind(Path::new("service.proto")),
        Some(FormatKind::Protobuf)
    );
}

#[test]
fn detect_graphql_extensions() {
    assert_eq!(
        detect_kind(Path::new("schema.graphql")),
        Some(FormatKind::Graphql)
    );
    assert_eq!(
        detect_kind(Path::new("query.gql")),
        Some(FormatKind::Graphql)
    );
}

#[test]
fn detect_hcl_extensions() {
    assert_eq!(detect_kind(Path::new("main.hcl")), Some(FormatKind::Hcl));
    assert_eq!(detect_kind(Path::new("main.tf")), Some(FormatKind::Hcl));
}

#[test]
fn detect_lua_extension() {
    assert_eq!(detect_kind(Path::new("init.lua")), Some(FormatKind::Lua));
}

#[test]
fn detect_r_extension() {
    assert_eq!(detect_kind(Path::new("script.r")), Some(FormatKind::R));
}

#[test]
fn detect_sql_extension() {
    assert_eq!(detect_kind(Path::new("query.sql")), Some(FormatKind::Sql));
}

#[test]
fn detect_unknown_extension_returns_none() {
    assert_eq!(detect_kind(Path::new("file.c")), None);
    assert_eq!(detect_kind(Path::new("file.cpp")), None);
    assert_eq!(detect_kind(Path::new("file.java")), None);
    assert_eq!(detect_kind(Path::new("file.rb")), None);
    assert_eq!(detect_kind(Path::new("file.swift")), None);
    assert_eq!(detect_kind(Path::new("file")), None);
}

#[test]
fn detect_no_filename_returns_none() {
    // Path with no file_name (e.g., root or empty)
    assert_eq!(detect_kind(Path::new("")), None);
}

#[test]
fn detect_case_insensitive_extensions() {
    // Extensions are lowercased before matching
    assert_eq!(detect_kind(Path::new("data.JSON")), Some(FormatKind::Json));
    assert_eq!(detect_kind(Path::new("style.CSS")), Some(FormatKind::Css));
    assert_eq!(
        detect_kind(Path::new("CONFIG.YAML")),
        Some(FormatKind::Yaml)
    );
}

#[test]
fn detect_with_directory_path() {
    assert_eq!(
        detect_kind(Path::new("/some/deep/path/file.json")),
        Some(FormatKind::Json)
    );
    assert_eq!(
        detect_kind(Path::new("relative/path/Makefile")),
        Some(FormatKind::Makefile)
    );
}

#[test]
fn detect_dockerfile_extension_variant() {
    // ".dockerfile" as extension
    assert_eq!(
        detect_kind(Path::new("build.dockerfile")),
        Some(FormatKind::Dockerfile)
    );
}

// ──────────────────────────────────────────────
// ensure_newline 規格測試
// ──────────────────────────────────────────────

#[test]
fn ensure_newline_adds_when_missing() {
    use formatter::formats::ensure_newline;
    assert_eq!(ensure_newline("hello".to_string()), "hello\n");
}

#[test]
fn ensure_newline_preserves_existing() {
    use formatter::formats::ensure_newline;
    assert_eq!(ensure_newline("hello\n".to_string()), "hello\n");
}

#[test]
fn ensure_newline_empty_string() {
    use formatter::formats::ensure_newline;
    assert_eq!(ensure_newline("".to_string()), "\n");
}

#[test]
fn ensure_newline_only_newline() {
    use formatter::formats::ensure_newline;
    assert_eq!(ensure_newline("\n".to_string()), "\n");
}

#[test]
fn ensure_newline_multiple_newlines() {
    use formatter::formats::ensure_newline;
    assert_eq!(ensure_newline("a\n\n".to_string()), "a\n\n");
}

// ──────────────────────────────────────────────
// format_dispatch 路由完整性測試
// ──────────────────────────────────────────────

#[test]
fn dispatch_routes_to_correct_formatter() {
    use formatter::formats::format_dispatch;

    // Test that dispatch doesn't panic for each kind with trivial input
    let kinds_and_paths = vec![
        (FormatKind::Json, "a.json", "{\"a\":1}"),
        (FormatKind::Yaml, "a.yaml", "a: 1\n"),
        (FormatKind::Toml, "a.toml", "[a]\nb = 1\n"),
        (FormatKind::Xml, "a.xml", "<a/>"),
        (FormatKind::Bash, "a.sh", "echo hi\n"),
        (FormatKind::Css, "a.css", "h1 { color: red; }\n"),
        (FormatKind::Ini, "a.ini", "[s]\nk=v\n"),
        (FormatKind::Protobuf, "a.proto", "message A {\n}\n"),
        (FormatKind::R, "a.r", "x <- 1\n"),
        (FormatKind::Sql, "a.sql", "SELECT 1;\n"),
    ];

    for (kind, path, input) in kinds_and_paths {
        let result = format_dispatch(kind, Path::new(path), input);
        assert!(
            result.is_ok(),
            "format_dispatch failed for {:?}: {:?}",
            kind,
            result.err()
        );
    }
}
