use formatter::formats::yaml;
use std::path::Path;

fn run(input: &str) -> String {
    yaml::format(Path::new("test.yaml"), input)
        .unwrap()
        .unwrap_or_else(|| input.to_string())
}

#[test]
fn adds_trailing_newline() {
    let input = "foo: 1";
    let out = run(input);
    assert!(out.ends_with('\n'));
}

#[test]
fn preserves_comments() {
    let input = "foo: 1 # cmt";
    let out = run(input);
    assert!(out.contains("# cmt"));
}

#[test]
fn preserves_document_boundaries() {
    let input = "a: 1\n---\nb: 2\n";
    let out = run(input);
    assert!(out.contains("a: 1"));
    assert!(out.contains("---\n"));
    assert!(out.contains("b: 2"));
}

#[test]
fn normalizes_sequence_indent() {
    let input = "list:\n- 1\n- 2\n";
    let out = run(input);
    assert!(out.contains("list:\n  - 1\n  - 2\n"));
}

#[test]
fn detects_crlf_and_preserves() {
    let input = "foo: 1\r\nbar: 2\r\n";
    let out = run(input);
    assert!(out.contains("\r\n"));
}
