//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/test-support/wiring/p1293_contract.md
//! @prompt-hash 3eb5861d
//! @layer L3
//! @updated 2026-10-16
//! Infraestrutura de teste; compilada apenas pelo harness L4.

use std::path::PathBuf;
use std::process::Output;
use std::{env, fs};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Transcript {
    pub(super) code: Option<i32>,
    pub(super) stdout: String,
    pub(super) stderr: String,
}

impl Transcript {
    pub(super) fn from_output(output: Output) -> Self {
        Self {
            code: output.status.code(),
            stdout: trim_transport(&output.stdout),
            stderr: trim_transport(&output.stderr),
        }
    }

    pub(super) fn success(&self) -> bool {
        self.code == Some(0)
    }
}

fn trim_transport(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .trim_end_matches(['\r', '\n'])
        .to_owned()
}

pub(super) struct Scratch(pub(super) PathBuf);

impl Scratch {
    pub(super) fn new(label: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "typst-p1293-{label}-{}-{:?}",
            std::process::id(),
            std::thread::current().id(),
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create P1293 scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum DomNode {
    Element { tag: String, attrs: Vec<(String, String)>, children: Vec<DomNode> },
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ParsedDom {
    pub(super) roots: Vec<DomNode>,
    pub(super) closing_tags: Vec<String>,
}

fn decode_entities(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn tag_end(html: &str, start: usize) -> usize {
    let mut quote = false;
    for (offset, ch) in html[start..].char_indices() {
        match ch {
            '"' => quote = !quote,
            '>' if !quote => return start + offset,
            _ => {}
        }
    }
    panic!("unterminated HTML tag")
}

fn parse_open_tag(raw: &str) -> (String, Vec<(String, String)>, bool) {
    let mut input = raw.trim();
    let self_closing = input.ends_with('/');
    if self_closing {
        input = input[..input.len() - 1].trim_end();
    }
    let name_end = input.find(char::is_whitespace).unwrap_or(input.len());
    let name = input[..name_end].to_owned();
    let mut rest = input[name_end..].trim_start();
    let mut attrs = Vec::new();
    while !rest.is_empty() {
        let key_end = rest
            .find(|ch: char| ch.is_whitespace() || ch == '=')
            .unwrap_or(rest.len());
        let key = rest[..key_end].to_owned();
        rest = rest[key_end..].trim_start();
        if !rest.starts_with('=') {
            attrs.push((key, String::new()));
            continue;
        }
        rest = rest[1..].trim_start();
        assert!(rest.starts_with('"'), "quoted HTML attribute expected: {raw}");
        rest = &rest[1..];
        let value_end = rest.find('"').expect("attribute closing quote");
        attrs.push((key, decode_entities(&rest[..value_end])));
        rest = rest[value_end + 1..].trim_start();
    }
    (name, attrs, self_closing)
}

fn push_dom_node(roots: &mut Vec<DomNode>, stack: &mut [DomNode], node: DomNode) {
    if let Some(DomNode::Element { children, .. }) = stack.last_mut() {
        children.push(node);
    } else {
        roots.push(node);
    }
}

pub(super) fn parse_dom(html: &str) -> ParsedDom {
    let void = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
        "source", "track", "wbr",
    ];
    let mut roots = Vec::new();
    let mut stack = Vec::new();
    let mut closing_tags = Vec::new();
    let mut at = 0;
    while at < html.len() {
        let Some(relative) = html[at..].find('<') else {
            let text = &html[at..];
            if !text.is_empty() {
                push_dom_node(
                    &mut roots,
                    &mut stack,
                    DomNode::Text(decode_entities(text)),
                );
            }
            break;
        };
        let open = at + relative;
        if open > at {
            let text = &html[at..open];
            if !text.is_empty() {
                push_dom_node(
                    &mut roots,
                    &mut stack,
                    DomNode::Text(decode_entities(text)),
                );
            }
        }
        let end = tag_end(html, open + 1);
        let raw = &html[open + 1..end];
        if raw.starts_with('!') || raw.starts_with('?') {
            at = end + 1;
            continue;
        }
        if let Some(name) = raw.strip_prefix('/') {
            let name = name.trim().to_owned();
            closing_tags.push(name.clone());
            let node =
                stack.pop().unwrap_or_else(|| panic!("closing {name} without opener"));
            let DomNode::Element { tag, .. } = &node else { unreachable!() };
            assert_eq!(tag, &name, "mismatched HTML closing tag");
            push_dom_node(&mut roots, &mut stack, node);
        } else {
            let (tag, attrs, self_closing) = parse_open_tag(raw);
            let is_void = void.contains(&tag.as_str());
            let node = DomNode::Element { tag, attrs, children: Vec::new() };
            if self_closing || is_void {
                push_dom_node(&mut roots, &mut stack, node);
            } else {
                stack.push(node);
            }
        }
        at = end + 1;
    }
    assert!(stack.is_empty(), "unclosed HTML nodes: {stack:#?}");
    ParsedDom { roots, closing_tags }
}

pub(super) fn visit_elements<'a>(
    nodes: &'a [DomNode],
    output: &mut Vec<(&'a str, &'a [(String, String)])>,
) {
    for node in nodes {
        if let DomNode::Element { tag, attrs, children } = node {
            output.push((tag, attrs));
            visit_elements(children, output);
        }
    }
}
