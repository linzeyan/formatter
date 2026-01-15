use std::path::Path;

use formatter::formats::bash;

fn fmt(input: &str) -> String {
    bash::format(Path::new("test.sh"), input)
        .unwrap()
        .unwrap_or_else(|| input.to_string())
}

#[test]
fn formats_if_else_blocks() {
    let input = "if [ \"$a\" -gt 0 ];then\n echo yes\nelse\n echo no\nfi\n";
    let expected = "if [ \"$a\" -gt 0 ]; then\n  echo yes\nelse\n  echo no\nfi\n";
    assert_eq!(fmt(input), expected);
}

#[test]
fn spaces_pipes_and_redirects() {
    let input = "echo 1|grep 1>out.txt\n";
    let expected = "echo 1 | grep 1 > out.txt\n";
    assert_eq!(fmt(input), expected);
}

#[test]
fn formats_case_arms_with_body_indent() {
    let input = "case \"$1\" in\na)\n echo a\n;;\n*)\n echo other\n;;\nesac\n";
    let expected = "case \"$1\" in\n  a)\n    echo a\n  ;;\n  *)\n    echo other\n  ;;\nesac\n";
    assert_eq!(fmt(input), expected);
}

#[test]
fn fixes_function_brace_and_inline_comment_spacing() {
    let input = "foo(){\n echo hi  #comment\n}\n";
    let expected = "foo() {\n  echo hi  #comment\n}\n";
    assert_eq!(fmt(input), expected);
}
