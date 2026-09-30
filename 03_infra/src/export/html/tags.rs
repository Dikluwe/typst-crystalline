//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/tags.md
//! @prompt-hash b4761c72
//! @layer L3
//! @updated 2026-09-26

pub(super) const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
    "track", "wbr",
];

pub(super) const RAW_TEXT_TAGS: &[&str] = &["script", "style"];

const GROUPABLE_PHRASING_TAGS: &[&str] = &[
    "a", "abbr", "audio", "b", "bdi", "bdo", "br", "button", "canvas", "cite", "code",
    "data", "del", "dfn", "em", "embed", "i", "iframe", "img", "input", "ins", "kbd",
    "label", "map", "mark", "math", "meter", "noscript", "object", "output", "picture",
    "progress", "q", "ruby", "s", "samp", "select", "slot", "small", "span", "strong",
    "sub", "sup", "textarea", "time", "u", "var", "video", "wbr",
];

pub(super) const BLOCK_TAGS: &[&str] = &[
    "html",
    "body",
    "address",
    "blockquote",
    "dialog",
    "div",
    "figure",
    "figcaption",
    "footer",
    "form",
    "header",
    "hr",
    "legend",
    "main",
    "p",
    "pre",
    "search",
    "article",
    "aside",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "hgroup",
    "nav",
    "section",
    "dd",
    "dl",
    "dt",
    "menu",
    "ol",
    "ul",
    "summary",
];

pub(super) const REPLACED_TAGS: &[&str] =
    &["audio", "canvas", "embed", "iframe", "img", "input", "object", "video"];

pub(super) fn should_group_into_paragraph(tag: &str) -> bool {
    GROUPABLE_PHRASING_TAGS.contains(&tag)
}
