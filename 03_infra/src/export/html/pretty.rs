//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/pretty.md
//! @prompt-hash cf325930
//! @layer L3
//! @updated 2026-09-26

use super::tags::{should_group_into_paragraph, VOID_TAGS};

#[derive(Clone, Copy, PartialEq, Eq)]
enum HtmlTokenKind {
    Open,
    Close,
    Void,
    Declaration,
    Text,
}

struct HtmlToken<'a> {
    raw: &'a str,
    name: &'a str,
    kind: HtmlTokenKind,
}

/// Adds whitespace only between structural tokens. The compact encoder is
/// still the semantic source of truth; this pass never rewrites a token.
pub(super) fn pretty_html(html: &str) -> String {
    let tokens = html_tokens(html);
    let mut out = String::with_capacity(html.len() + tokens.len() * 2);
    let mut stack: Vec<&str> = Vec::new();
    let mut last_was_structural = false;

    for token in tokens {
        let protected = stack.iter().any(|name| is_protected_html_context(name));
        let structural = token.kind != HtmlTokenKind::Text
            && token.kind != HtmlTokenKind::Declaration
            && !should_group_into_paragraph(token.name);

        match token.kind {
            HtmlTokenKind::Declaration => {
                out.push_str(token.raw);
                last_was_structural = true;
            }
            HtmlTokenKind::Open => {
                if structural && !protected {
                    push_structural_newline(&mut out, stack.len());
                }
                out.push_str(token.raw);
                stack.push(token.name);
                last_was_structural = structural && !protected;
            }
            HtmlTokenKind::Void => {
                if structural && !protected {
                    push_structural_newline(&mut out, stack.len());
                }
                out.push_str(token.raw);
                last_was_structural = structural && !protected;
            }
            HtmlTokenKind::Close => {
                let depth = stack.len().saturating_sub(1);
                let closing_protected = is_protected_html_context(token.name)
                    || stack
                        .iter()
                        .take(depth)
                        .any(|name| is_protected_html_context(name));
                if structural && !closing_protected && last_was_structural {
                    push_structural_newline(&mut out, depth);
                }
                out.push_str(token.raw);
                stack.pop();
                last_was_structural = structural && !closing_protected;
            }
            HtmlTokenKind::Text => {
                out.push_str(token.raw);
                if !token.raw.is_empty() {
                    last_was_structural = false;
                }
            }
        }
    }

    out.push('\n');
    out
}

fn push_structural_newline(out: &mut String, depth: usize) {
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out.extend(std::iter::repeat(' ').take(depth * 2));
}

fn is_protected_html_context(tag: &str) -> bool {
    should_group_into_paragraph(tag)
        || matches!(
            tag,
            "p" | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "h5"
                | "h6"
                | "title"
                | "pre"
                | "textarea"
                | "script"
                | "style"
                | "td"
                | "th"
        )
}

fn html_tokens(html: &str) -> Vec<HtmlToken<'_>> {
    let mut tokens = Vec::new();
    let bytes = html.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] != b'<' {
            let end = html[cursor..]
                .find('<')
                .map(|offset| cursor + offset)
                .unwrap_or(bytes.len());
            tokens.push(HtmlToken {
                raw: &html[cursor..end],
                name: "",
                kind: HtmlTokenKind::Text,
            });
            cursor = end;
            continue;
        }

        let end = quoted_tag_end(bytes, cursor).unwrap_or(bytes.len() - 1);
        let raw = &html[cursor..=end];
        let (name, kind) = classify_html_tag(raw);
        tokens.push(HtmlToken { raw, name, kind });
        cursor = end + 1;
    }
    tokens
}

fn quoted_tag_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut quote = None;
    for (offset, byte) in bytes[start + 1..].iter().copied().enumerate() {
        match byte {
            b'\'' | b'"' if quote.is_none() => quote = Some(byte),
            current if quote == Some(current) => quote = None,
            b'>' if quote.is_none() => return Some(start + 1 + offset),
            _ => {}
        }
    }
    None
}

fn classify_html_tag(raw: &str) -> (&str, HtmlTokenKind) {
    if raw.starts_with("<!") || raw.starts_with("<?") {
        return ("", HtmlTokenKind::Declaration);
    }
    let closing = raw.starts_with("</");
    let name_start = if closing { 2 } else { 1 };
    let rest = raw.get(name_start..).unwrap_or("");
    let name_end = rest
        .find(|character: char| {
            character.is_ascii_whitespace() || character == '>' || character == '/'
        })
        .unwrap_or(rest.len());
    let name = &rest[..name_end];
    let kind = if closing {
        HtmlTokenKind::Close
    } else if VOID_TAGS.contains(&name) || raw.trim_end().ends_with("/>") {
        HtmlTokenKind::Void
    } else {
        HtmlTokenKind::Open
    };
    (name, kind)
}
