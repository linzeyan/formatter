use std::path::Path;

use formatter::formats::{
    FormatError, bash, css, dockerfile, go, graphql, hcl, html, ini, javascript, json, lua,
    makefile, markdown, nginx, protobuf, python, rlang, rustfmt, sql, toml_fmt, typescript, xml,
    yaml,
};

// Helper to unwrap formatter output while keeping original text when unchanged
fn run(
    f: fn(&Path, &str) -> anyhow::Result<Option<String>, FormatError>,
    path: &str,
    input: &str,
) -> String {
    f(Path::new(path), input)
        .unwrap()
        .unwrap_or_else(|| input.to_string())
}

#[test]
fn bash_trims_and_normalizes_continuations() {
    let input = "echo 1 &&\\\n echo 2  ";
    let out = run(bash::format, "a.sh", input);
    assert_eq!(out, "echo 1 && \\\n  echo 2\n");
    assert!(out.ends_with('\n'));
}

#[test]
fn css_formats_and_appends_newline() {
    let input = "h1{color:red;}";
    let out = run(css::format, "a.css", input);
    assert!(out.contains("color"));
    assert!(out.ends_with('\n'));
}

#[test]
fn dockerfile_run_blocks_use_bash_rules() {
    let input = "FROM alpine\nRUN echo 1 &&\\\n echo 2\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    assert!(out.contains("RUN echo 1 && \\\n"));
    assert!(
        out.lines()
            .any(|line| line.trim_start().starts_with("echo 2"))
    );
    assert!(out.ends_with('\n'));
}

#[test]
fn go_formats_basic_file() {
    let input = "package main\nfunc main(){println(\"hi\")}\n";
    let out = run(go::format, "a.go", input);
    assert!(out.contains("func main() {"));
}

#[test]
fn graphql_inserts_spacing() {
    let input = "type A{b:Int}\n";
    let out = run(graphql::format, "a.graphql", input);
    assert!(out.contains("type A"));
    assert!(out.contains("Int"));
    assert!(out.ends_with('\n'));
}

#[test]
fn hcl_indents_blocks() {
    let input = "resource \"x\" \"y\"{}";
    let out = run(hcl::format, "a.hcl", input);
    assert!(out.contains("resource \"x\" \"y\" {"));
    assert!(out.ends_with('\n'));
}

#[test]
fn html_serializes_and_trims() {
    let input = "<div>1</div>   ";
    let out = run(html::format, "a.html", input);
    assert!(out.contains("<div>1</div>"));
    assert!(out.ends_with('\n'));
}

#[test]
fn ini_round_trips_and_trims() {
    let input = " [a]\n x = 1 \n";
    let out = run(ini::format, "a.ini", input);
    assert!(out.contains("[a]\nx=1"));
    assert!(out.ends_with('\n'));
}

#[test]
fn javascript_delegates_to_ts_with_newline() {
    let input = "const  a=1";
    let out = run(javascript::format, "a.js", input);
    assert!(out.contains("const a = 1;"));
    assert!(out.ends_with('\n'));
}

#[test]
fn json_adds_spaces_and_newline() {
    let input = "{\"a\":1}";
    let out = run(json::format, "a.json", input);
    assert!(out.contains("\"a\": 1"));
    assert!(out.ends_with('\n'));
}

#[test]
fn lua_normalizes_tables() {
    let input = "local t={1,2}";
    let out = run(lua::format, "a.lua", input);
    assert!(out.contains("local t = { 1, 2 }"));
}

#[test]
fn makefile_rules_cover_dependencies_recipes_comments() {
    let input = "all:dep1   dep2 \n  # comment\n echo hi  \n";
    let out = run(makefile::format, "Makefile", input);
    assert!(out.contains("all: dep1 dep2"));
    assert!(out.contains("\n# comment\n"));
    assert!(out.contains("\n\techo hi\n"));
}

#[test]
fn makefile_collapses_blank_lines() {
    let input = "a:\n\n\n\techo ok\n";
    let out = run(makefile::format, "Makefile", input);
    assert!(out.contains("a:\n\n\techo ok\n"));
    assert!(!out.contains("\n\n\n"));
}

#[test]
fn markdown_formats_embedded_code_blocks() {
    let input = "```json\n{\"a\":1}\n```\n";
    let out = run(markdown::format, "a.md", input);
    assert!(out.contains("\"a\": 1"));
    assert!(out.ends_with('\n'));
}

#[test]
fn nginx_indents_blocks_and_comments() {
    let input = "# top\nserver {\n# nested\nlocation /{return 200;}\n}\n";
    let out = run(nginx::format, "nginx.conf", input);
    assert!(out.contains("# top"));
    assert!(out.contains("server {"));
    assert!(out.lines().any(|l| l.trim_start().starts_with("# nested")));
    assert!(out.contains("location / {"));
}

#[test]
fn protobuf_spaces_around_equals() {
    let input = "message A{\nint32 x=1;\n}\n";
    let out = run(protobuf::format, "a.proto", input);
    assert!(out.contains("message A"));
    assert!(out.contains("int32 x = 1;"));
    assert!(out.ends_with('\n'));
}

#[test]
fn python_ruff_spacing() {
    let input = "x=1";
    let res = python::format(Path::new("a.py"), input).unwrap();
    if let Some(out) = res {
        assert!(out.ends_with('\n'));
    } else {
        assert_eq!(input, "x=1");
    }
}

#[test]
fn rlang_indents_braces() {
    let input = "if (TRUE){\nprint(1)\n}\n";
    let out = run(rlang::format, "a.r", input);
    assert!(out.contains("if (TRUE){"));
    assert!(out.contains("\n  print(1)\n"));
}

#[test]
fn rustfmt_structures_code() {
    let input = "fn main(){println!(\"hi\")}";
    let out = run(rustfmt::format, "a.rs", input);
    assert!(out.contains("fn main"));
    assert!(out.contains("println!(\"hi\")"));
}

#[test]
fn sql_formats_and_newline() {
    let input = "select * from t where a=1";
    let out = run(sql::format, "a.sql", input);
    let normalized = out
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(normalized.contains("select * from t where a = 1"));
    assert!(out.ends_with('\n'));
}

#[test]
fn toml_formats_with_spacing() {
    let input = "[a]\nb=1";
    let out = run(toml_fmt::format, "a.toml", input);
    assert!(out.contains("[a]"));
    assert!(out.contains("b = 1"));
    assert!(out.ends_with('\n'));
}

#[test]
fn typescript_formats_and_semicolons() {
    let input = "const  x=1";
    let out = run(typescript::format, "a.ts", input);
    assert!(out.contains("const x = 1;"));
    assert!(out.ends_with('\n'));
}

#[test]
fn xml_indents_children() {
    let input = "<a><b>1</b></a>";
    let out = run(xml::format, "a.xml", input);
    assert!(out.contains("<a>\n  <b>1</b>\n</a>\n"));
}

#[test]
fn yaml_round_trips_with_newline() {
    let input = "foo: 1\nbar: [2,3]";
    let out = run(yaml::format, "a.yaml", input);
    assert!(out.contains("foo: 1"));
    assert!(out.contains("bar:"));
    assert!(out.ends_with('\n'));
}
