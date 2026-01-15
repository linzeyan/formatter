use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use tree_sitter::{Language, Parser, Tree};

use super::{FormatError, ensure_newline};

static BASH_LANG: Lazy<Language> = Lazy::new(pepegsitter::bash::language);

const INDENT: usize = 2;

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    if text.is_empty() {
        return Ok(None);
    }

    let _ = parse(text); // syntax validation; formatting falls back to heuristics

    let mut out = String::new();
    let mut indent = 0usize;
    let mut continuation = false;
    let mut last_blank = false;
    let mut case_stack: Vec<CaseState> = Vec::new();

    for raw in text.lines() {
        let line = raw.trim_end();
        let blank = line.trim().is_empty();
        if blank {
            if last_blank {
                continue;
            }
            last_blank = true;
            out.push('\n');
            continue;
        }
        last_blank = false;

        let trimmed = line.trim_start();
        let lower = trimmed.to_lowercase();

        // Pre-dedent for closers.
        if starts_with_any(&lower, &["fi", "done", "}"]) || lower.starts_with("esac") {
            indent = indent.saturating_sub(1);
        }
        if lower.starts_with("elif") || lower.starts_with("else") {
            indent = indent.saturating_sub(1);
        }

        let is_case_pattern = is_case_pattern(trimmed, &case_stack);
        let is_case_terminator = is_case_terminator(&lower);

        let base_indent = if is_case_pattern || is_case_terminator {
            case_stack
                .last()
                .map(|c| c.base_indent + 1)
                .unwrap_or(indent)
        } else {
            indent
        };
        let line_indent = if continuation {
            base_indent + 1
        } else {
            base_indent
        };

        let formatted = normalize_line(trimmed, line_indent);
        out.push_str(&formatted);
        out.push('\n');

        if lower.starts_with("case ") {
            case_stack.push(CaseState {
                base_indent: indent,
            });
            indent += 1;
        } else if is_case_pattern {
            if let Some(state) = case_stack.last() {
                indent = state.base_indent + 2;
            } else {
                indent += 1;
            }
        } else if is_case_terminator {
            if let Some(state) = case_stack.last() {
                indent = state.base_indent + 1;
            } else {
                indent = indent.saturating_sub(1);
            }
        } else if lower.starts_with("else") || lower.starts_with("elif") {
            indent += 1;
        }

        if opens_block(trimmed) {
            indent += 1;
        }

        if lower.starts_with("esac") {
            if let Some(state) = case_stack.pop() {
                indent = state.base_indent;
            }
        }

        continuation = line.trim_end().ends_with('\\');
    }

    let mut result = ensure_newline(out);
    for pat in ["> =", "< =", "! ="] {
        let fixed = pat.replace(' ', "");
        result = result.replace(pat, &fixed);
    }
    if result == text {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn parse(text: &str) -> Result<Tree, FormatError> {
    let mut parser = Parser::new();
    parser
        .set_language(*BASH_LANG)
        .map_err(|e| FormatError::Message(format!("load bash grammar: {e}")))?;
    parser
        .parse(text, None)
        .ok_or_else(|| FormatError::Message("failed to parse shell script".to_string()))
}

fn starts_with_any(line: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|p| line.trim_start().starts_with(p))
}

fn is_case_pattern(line: &str, case_stack: &[CaseState]) -> bool {
    if case_stack.is_empty() {
        return false;
    }
    let trimmed = line.trim();
    if trimmed.starts_with('#') {
        return false;
    }
    trimmed.ends_with(')') && !trimmed.starts_with("case ")
}

fn is_case_terminator(lower: &str) -> bool {
    lower.starts_with(";;") || lower.starts_with(";;&") || lower.starts_with(";&")
}

fn opens_block(line: &str) -> bool {
    let trimmed = line.trim();
    let lower = trimmed.to_lowercase();

    if lower == "then" || lower == "do" {
        return true;
    }
    if lower.contains(" fi")
        || lower.ends_with("fi")
        || lower.contains(" done")
        || lower.ends_with("done")
    {
        return false;
    }
    if lower.starts_with("if ")
        && (lower.contains(" then") || lower.contains(";then") || lower.ends_with("then"))
    {
        return true;
    }
    if lower.starts_with("for ")
        && (lower.contains(" do") || lower.contains(";do") || lower.ends_with("do"))
    {
        return true;
    }
    if lower.starts_with("while ")
        && (lower.contains(" do") || lower.contains(";do") || lower.ends_with("do"))
    {
        return true;
    }
    if lower.starts_with("until ")
        && (lower.contains(" do") || lower.contains(";do") || lower.ends_with("do"))
    {
        return true;
    }
    if lower.starts_with("select ")
        && (lower.contains(" do") || lower.contains(";do") || lower.ends_with("do"))
    {
        return true;
    }
    if lower.starts_with("function ") && trimmed.ends_with('{') {
        return true;
    }
    trimmed.ends_with('{')
}

#[derive(Debug, Clone, Copy)]
struct CaseState {
    base_indent: usize,
}

fn normalize_line(line: &str, indent_level: usize) -> String {
    let (code, comment) = split_comment(line);
    let code_part = normalize_tokens(code);

    if code_part.is_empty() && comment.is_some() {
        match comment {
            Some(c) => return format!("{}{}", indent_str(indent_level), c),
            None => unreachable!(),
        }
    }

    if let Some(c) = comment {
        format!("{}{}  {}", indent_str(indent_level), code_part, c)
    } else {
        format!("{}{}", indent_str(indent_level), code_part)
    }
}

fn normalize_tokens(input: &str) -> String {
    let mut s = input.replace('\t', "    ");
    s = s.replace(";then", "; then");
    s = s.replace(";do", "; do");
    s = s.replace(";else", "; else");
    while s.contains("> =") {
        s = s.replace("> =", ">=");
    }
    while s.contains("< =") {
        s = s.replace("< =", "<=");
    }
    while s.contains("! =") {
        s = s.replace("! =", "!=");
    }
    for kw in ["if", "while", "for", "until", "select"] {
        let needle = format!("{kw}[");
        let replacement = format!("{kw} [");
        if s.contains(&needle) {
            s = s.replace(&needle, &replacement);
        }
    }
    s = normalize_function_brace(&s);
    s = normalize_binary_ops(&s);
    s = normalize_comparators(&s);
    s = normalize_redirects(&s);
    s.trim().to_string()
}

fn normalize_function_brace(s: &str) -> String {
    if let Some(pos) = s.find("(){") {
        let name = s[..pos].trim_end();
        let rest = s[pos + 3..].trim_start();
        if rest.is_empty() {
            return format!("{name}() {{");
        }
        return format!("{name}() {{ {rest}");
    }
    s.to_string()
}

fn normalize_binary_ops(s: &str) -> String {
    ["&&", "||", "|"].iter().fold(s.to_string(), |acc, op| {
        if acc.contains(op) {
            acc.split(op)
                .map(str::trim)
                .filter(|p| !p.is_empty())
                .collect::<Vec<_>>()
                .join(&format!(" {} ", op))
        } else {
            acc
        }
    })
}

fn normalize_redirects(s: &str) -> String {
    let mut parts = Vec::new();
    for token in s.split_whitespace() {
        if let Some((lhs, op, rhs)) = split_redirect(token) {
            if !lhs.is_empty() {
                parts.push(lhs);
            }
            parts.push(op);
            if !rhs.is_empty() {
                parts.push(rhs);
            }
        } else {
            parts.push(token.to_string());
        }
    }
    parts.join(" ")
}

fn normalize_comparators(s: &str) -> String {
    let tokens: Vec<&str> = s.split_whitespace().collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if i + 1 < tokens.len()
            && (tokens[i] == ">" || tokens[i] == "<" || tokens[i] == "!")
            && tokens[i + 1] == "="
        {
            out.push(format!("{}=", tokens[i]));
            i += 2;
            continue;
        }
        out.push(tokens[i].to_string());
        i += 1;
    }
    out.join(" ")
}

fn split_redirect(token: &str) -> Option<(String, String, String)> {
    let ops = ["<<<", "<<-", "<<", ">>", ">&", "&>", "<&", ">", "<"];
    for op in ops {
        if let Some(pos) = token.find(op) {
            let lhs = token[..pos].to_string();
            let rhs = token[pos + op.len()..].to_string();
            return Some((lhs, op.to_string(), rhs));
        }
    }
    None
}

fn split_comment(line: &str) -> (&str, Option<&str>) {
    let mut in_single = false;
    let mut in_double = false;
    for (idx, ch) in line.char_indices() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '#' if !in_single && !in_double => {
                if idx == 0 || line.as_bytes()[idx - 1].is_ascii_whitespace() {
                    let code = line[..idx].trim_end();
                    let comment = &line[idx..];
                    return (code, Some(comment));
                }
            }
            _ => {}
        }
    }
    (line.trim_end(), None)
}

fn indent_str(level: usize) -> String {
    " ".repeat(level * INDENT)
}
