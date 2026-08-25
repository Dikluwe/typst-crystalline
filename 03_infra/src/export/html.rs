//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html.md
//! @prompt-hash ba8a0c7e
//! @layer L3

use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

const DOCTYPE: &str = "<!DOCTYPE html>";
const AUTO_ROOT_PREFIX: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head>";
const PREFIX: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body>";

const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
    "track", "wbr",
];

const GROUPABLE_PHRASING_TAGS: &[&str] = &[
    "a", "abbr", "audio", "b", "bdi", "bdo", "br", "button", "canvas", "cite", "code",
    "data", "del", "dfn", "em", "embed", "i", "iframe", "img", "input", "ins", "kbd",
    "label", "map", "mark", "math", "meter", "noscript", "object", "output", "picture",
    "progress", "q", "ruby", "s", "samp", "select", "slot", "small", "span", "strong",
    "sub", "sup", "textarea", "time", "u", "var", "video", "wbr",
];

const BLOCK_TAGS: &[&str] = &[
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

const REPLACED_TAGS: &[&str] =
    &["audio", "canvas", "embed", "iframe", "img", "input", "object", "video"];

const PROTECTED_SPACE: &str = "<span style=\"white-space: pre-wrap\">&#x20;</span>";

pub fn export_html(content: &Content) -> Result<String, SourceDiagnostic> {
    if let Some(root) = explicit_document_root(content)? {
        return if matches!(root, Content::HtmlElem(elem) if elem.tag == "html") {
            Ok(format!("{DOCTYPE}{}", inline(root)?))
        } else {
            Ok(format!("{AUTO_ROOT_PREFIX}{}</html>", inline(root)?))
        };
    }
    let body = match content {
        Content::Empty => String::new(),
        Content::Heading(h) => {
            format!("<h{}>{}</h{}>", h.level, inline(&h.body)?.trim(), h.level)
        }
        Content::HtmlElem(elem) if should_group_into_paragraph(elem.tag.as_str()) => {
            format!("<p>{}</p>", inline(content)?)
        }
        Content::HtmlElem(_) => inline(content)?,
        Content::Sequence(seq)
            if seq.iter().any(|c| {
                matches!(
                    c,
                    Content::Heading(_)
                        | Content::Parbreak
                        | Content::Par { .. }
                        | Content::HtmlElem(_)
                )
            }) =>
        {
            block_sequence(seq)?
        }
        Content::Par { body } => format!("<p>{}</p>", inline(body)?.trim()),
        other => format!("<p>{}</p>", inline(other)?.trim()),
    };
    Ok(format!("{PREFIX}{body}</body></html>"))
}

fn explicit_document_root(
    content: &Content,
) -> Result<Option<&Content>, SourceDiagnostic> {
    let nodes: Vec<&Content> = match content {
        Content::Sequence(items) => items
            .iter()
            .filter(|item| !matches!(item, Content::Empty | Content::Space))
            .collect(),
        Content::Empty | Content::Space => vec![],
        other => vec![other],
    };

    for node in &nodes {
        if let Content::HtmlElem(elem) = node {
            if matches!(elem.tag.as_str(), "html" | "body") {
                if nodes.len() == 1 {
                    return Ok(Some(node));
                }
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "`<{}>` element must be the only element in the document",
                        elem.tag
                    ),
                ));
            }
        }
    }
    Ok(None)
}

fn block_sequence(seq: &[Content]) -> Result<String, SourceDiagnostic> {
    let mut out = String::new();
    let mut paragraph = String::new();
    let flush = |out: &mut String, paragraph: &mut String| {
        if !paragraph.trim().is_empty() {
            out.push_str("<p>");
            out.push_str(paragraph.trim());
            out.push_str("</p>");
            paragraph.clear();
        }
    };
    for (index, item) in seq.iter().enumerate() {
        match item {
            Content::Parbreak => flush(&mut out, &mut paragraph),
            Content::Heading(h) => {
                flush(&mut out, &mut paragraph);
                out.push_str(&format!(
                    "<h{}>{}</h{}>",
                    h.level,
                    inline(&h.body)?.trim(),
                    h.level
                ));
            }
            Content::Par { body } => {
                flush(&mut out, &mut paragraph);
                out.push_str(&format!("<p>{}</p>", inline(body)?.trim()));
            }
            Content::HtmlElem(elem) => {
                if should_group_into_paragraph(elem.tag.as_str()) {
                    paragraph.push_str(&inline(item)?);
                } else {
                    flush(&mut out, &mut paragraph);
                    out.push_str(&inline(item)?);
                }
            }
            Content::Space => paragraph.push_str(top_level_space_between(seq, index)),
            other => paragraph.push_str(&inline(other)?),
        }
    }
    flush(&mut out, &mut paragraph);
    Ok(out)
}

fn should_group_into_paragraph(tag: &str) -> bool {
    GROUPABLE_PHRASING_TAGS.contains(&tag)
}

fn inline(content: &Content) -> Result<String, SourceDiagnostic> {
    Ok(match content {
        Content::Empty => String::new(),
        Content::Text(text) => escape(text),
        Content::Space => " ".into(),
        Content::Sequence(seq) => {
            seq.iter().map(inline).collect::<Result<Vec<_>, _>>()?.concat()
        }
        Content::Strong(e) => format!("<strong>{}</strong>", inline(&e.body)?),
        Content::Emph(e) => format!("<em>{}</em>", inline(&e.body)?),
        Content::Styled(body, _) => inline(body)?,
        Content::Linebreak(_) => "<br>".into(),
        Content::HtmlElem(elem) => {
            let mut out = String::new();
            out.push('<');
            out.push_str(elem.tag.as_str());
            if let Some(attrs) = &elem.attrs {
                for (name, value) in attrs {
                    out.push(' ');
                    out.push_str(name.as_str());
                    if !value.is_empty() {
                        out.push_str("=\"");
                        out.push_str(&escape(value));
                        out.push('"');
                    }
                }
            }
            out.push('>');
            if VOID_TAGS.contains(&elem.tag.as_str()) {
                if elem.body.content().is_some() {
                    return Err(SourceDiagnostic::error(
                        Span::detached(),
                        "HTML void elements must not have children",
                    ));
                }
                return Ok(out);
            }
            if let Some(body) = elem.body.content() {
                if elem.tag == "title" {
                    out.push_str(&title_text(body)?);
                } else {
                    out.push_str(&inline_html_body(elem.tag.as_str(), body)?);
                }
            }
            out.push_str("</");
            out.push_str(elem.tag.as_str());
            out.push('>');
            out
        }
        _ => {
            return Err(SourceDiagnostic::error(
                Span::detached(),
                "HTML export does not yet support this content",
            ))
        }
    })
}

fn title_text(content: &Content) -> Result<String, SourceDiagnostic> {
    Ok(match content {
        Content::Empty => String::new(),
        Content::Text(text) => text.as_str().replace('&', "&amp;").replace('<', "&lt;"),
        Content::Space => " ".into(),
        Content::Linebreak(_) => "\n".into(),
        Content::Sequence(items) => {
            let mut out = String::new();
            let mut after_linebreak = false;
            for item in items.iter() {
                if after_linebreak && matches!(item, Content::Space) {
                    after_linebreak = false;
                    continue;
                }
                out.push_str(&title_text(item)?);
                after_linebreak = matches!(item, Content::Linebreak(_));
            }
            out
        }
        Content::Styled(body, _) => title_text(body)?,
        _ => {
            return Err(SourceDiagnostic::error(
                Span::detached(),
                "HTML raw text element cannot have non-text children",
            ))
        }
    })
}

fn inline_html_body(
    parent_tag: &str,
    content: &Content,
) -> Result<String, SourceDiagnostic> {
    let Content::Sequence(items) = content else {
        return inline(content);
    };
    let first = items.iter().position(|item| !matches!(item, Content::Space));
    let Some(first) = first else { return Ok(String::new()) };
    let last = items
        .iter()
        .rposition(|item| !matches!(item, Content::Space))
        .unwrap();
    let mut out = String::new();
    for index in first..=last {
        if matches!(items[index], Content::Space) {
            if parent_tag == "ruby"
                && index > first
                && index < last
                && is_ruby_annotation(&items[index - 1])
                && is_ruby_annotation(&items[index + 1])
            {
                continue;
            }
            if index > first && collapses_adjacent_whitespace(&items[index - 1])
                || index < last && collapses_adjacent_whitespace(&items[index + 1])
            {
                continue;
            }
            out.push_str(space_between(items, index));
        } else {
            out.push_str(&inline(&items[index])?);
        }
    }
    Ok(out)
}

fn is_ruby_annotation(content: &Content) -> bool {
    matches!(content, Content::HtmlElem(elem) if matches!(elem.tag.as_str(), "rp" | "rt"))
}

fn space_between(items: &[Content], index: usize) -> &'static str {
    match (
        space_support(items[..index].iter().rev()),
        space_support(items[index + 1..].iter()),
    ) {
        (SpaceSupport::Boundary, _) | (_, SpaceSupport::Boundary) => "",
        (SpaceSupport::Supportive, SpaceSupport::Supportive) => " ",
        _ => PROTECTED_SPACE,
    }
}

fn top_level_space_between(items: &[Content], index: usize) -> &'static str {
    match (
        paragraph_space_support(items[..index].iter().rev()),
        paragraph_space_support(items[index + 1..].iter()),
    ) {
        (SpaceSupport::Boundary, _) | (_, SpaceSupport::Boundary) => "",
        (SpaceSupport::Supportive, SpaceSupport::Supportive) => " ",
        _ => PROTECTED_SPACE,
    }
}

fn paragraph_space_support<'a>(items: impl Iterator<Item = &'a Content>) -> SpaceSupport {
    let mut found_inline = false;
    for item in items {
        match item {
            Content::Parbreak | Content::Heading(_) | Content::Par { .. } => {
                return if found_inline {
                    SpaceSupport::EmptyInline
                } else {
                    SpaceSupport::Boundary
                };
            }
            Content::HtmlElem(elem)
                if !should_group_into_paragraph(elem.tag.as_str()) =>
            {
                return if found_inline {
                    SpaceSupport::EmptyInline
                } else {
                    SpaceSupport::Boundary
                };
            }
            _ => {}
        }
        if supports_adjacent_space(item) {
            return SpaceSupport::Supportive;
        }
        if !matches!(item, Content::Space | Content::Empty) {
            found_inline = true;
        }
    }
    if found_inline {
        SpaceSupport::EmptyInline
    } else {
        SpaceSupport::Boundary
    }
}

#[derive(Clone, Copy)]
enum SpaceSupport {
    Boundary,
    EmptyInline,
    Supportive,
}

fn space_support<'a>(items: impl Iterator<Item = &'a Content>) -> SpaceSupport {
    let mut found_inline = false;
    for item in items {
        if collapses_adjacent_whitespace(item) {
            return if found_inline {
                SpaceSupport::EmptyInline
            } else {
                SpaceSupport::Boundary
            };
        }
        if supports_adjacent_space(item) {
            return SpaceSupport::Supportive;
        }
        if !matches!(item, Content::Space | Content::Empty) {
            found_inline = true;
        }
    }
    if found_inline {
        SpaceSupport::EmptyInline
    } else {
        SpaceSupport::Boundary
    }
}

fn supports_adjacent_space(content: &Content) -> bool {
    match content {
        Content::Text(text) => !text.is_empty(),
        Content::Sequence(items) => items.iter().any(supports_adjacent_space),
        Content::Strong(elem) => supports_adjacent_space(&elem.body),
        Content::Emph(elem) => supports_adjacent_space(&elem.body),
        Content::Styled(body, _) => supports_adjacent_space(body),
        Content::HtmlElem(elem) => {
            if collapses_adjacent_whitespace(content) {
                false
            } else if REPLACED_TAGS.contains(&elem.tag.as_str()) {
                true
            } else {
                elem.body.content().is_some_and(supports_adjacent_space)
            }
        }
        Content::Empty | Content::Space | Content::Parbreak => false,
        _ => true,
    }
}

fn collapses_adjacent_whitespace(content: &Content) -> bool {
    matches!(content, Content::HtmlElem(elem) if elem.tag == "br" || BLOCK_TAGS.contains(&elem.tag.as_str()))
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::html::{HtmlAttrs, HtmlElem};

    #[test]
    fn exports_plain_paragraph_like_vanilla() {
        let html = export_html(&Content::text("Hello, parity. ")).unwrap();
        assert_eq!(html, format!("{PREFIX}<p>Hello, parity.</p></body></html>"));
    }

    #[test]
    fn escapes_html_metacharacters() {
        let html = export_html(&Content::text("<&\"> ")).unwrap();
        assert!(html.contains("&lt;&amp;&quot;&gt;"));
    }

    #[test]
    fn exports_explicit_html_element_without_paragraph_wrapper() {
        let mut attrs = HtmlAttrs::default();
        attrs.insert("lang".into(), "pt".into());
        attrs.insert("title".into(), "a&b".into());
        let elem = Content::HtmlElem(Arc::new(HtmlElem::new(
            "article".into(),
            Some(attrs),
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(
                "Olá & mundo",
            ))),
        )));
        let html = export_html(&Content::sequence(vec![elem])).unwrap();
        assert_eq!(
            html,
            format!(
                "{PREFIX}<article lang=\"pt\" title=\"a&amp;b\">Olá &amp; mundo</article></body></html>"
            )
        );
    }

    #[test]
    fn p1168_exports_empty_attribute_without_equals_or_quotes() {
        let mut attrs = HtmlAttrs::default();
        attrs.insert("hidden".into(), "".into());
        let elem = Content::HtmlElem(Arc::new(HtmlElem::new(
            "div".into(),
            Some(attrs),
            typst_core::entities::html::HtmlBody::None,
        )));
        let html = export_html(&elem).unwrap();
        assert!(html.contains("<div hidden></div>"));
        assert!(!html.contains("hidden=\"\""));
    }

    #[test]
    fn p1171_1_void_nao_fecha_e_rejeita_body() {
        let br = Content::HtmlElem(Arc::new(HtmlElem::new(
            "br".into(),
            None,
            typst_core::entities::html::HtmlBody::Unset,
        )));
        assert!(export_html(&br).unwrap().contains("<body><p><br></p></body>"));

        let with_body = Content::HtmlElem(Arc::new(HtmlElem::new(
            "br".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::text("X"))),
        )));
        assert_eq!(
            export_html(&with_body).unwrap_err().message,
            "HTML void elements must not have children"
        );
    }

    #[test]
    fn p1171_1_remove_bordas_e_preserva_espaco_inline() {
        let spans = Content::HtmlElem(Arc::new(HtmlElem::new(
            "div".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(
                vec![
                    Content::Space,
                    Content::HtmlElem(Arc::new(HtmlElem::new(
                        "span".into(),
                        None,
                        typst_core::entities::html::HtmlBody::Content(Box::new(
                            Content::text("A"),
                        )),
                    ))),
                    Content::Space,
                    Content::HtmlElem(Arc::new(HtmlElem::new(
                        "span".into(),
                        None,
                        typst_core::entities::html::HtmlBody::Content(Box::new(
                            Content::text("B"),
                        )),
                    ))),
                    Content::Space,
                ],
            ))),
        )));
        assert!(export_html(&spans)
            .unwrap()
            .contains("<div><span>A</span> <span>B</span></div>"));
    }

    #[test]
    fn p1172_1_agrupa_phrasing_e_respeita_boundaries() {
        let elem = |tag: &str, text: &str| {
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag.into(),
                None,
                typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(
                    text,
                ))),
            )))
        };
        let content =
            Content::sequence(vec![elem("span", "A"), elem("div", "B"), elem("a", "C")]);
        let html = export_html(&content).unwrap();
        assert!(html.contains("<p><span>A</span></p><div>B</div><p><a>C</a></p>"));

        let custom = export_html(&elem("x-widget", "X")).unwrap();
        assert!(custom.contains("<body><x-widget>X</x-widget></body>"));
        assert!(!custom.contains("<p><x-widget>"));
    }

    #[test]
    fn p1173_1_remove_espaco_adjacente_a_block_siblings() {
        let elem = |tag: &str, text: &str| {
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag.into(),
                None,
                typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(
                    text,
                ))),
            )))
        };
        let article = Content::HtmlElem(Arc::new(HtmlElem::new(
            "article".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(
                vec![elem("address", "A"), Content::Space, elem("aside", "B")],
            ))),
        )));
        assert!(export_html(&article)
            .unwrap()
            .contains("<article><address>A</address><aside>B</aside></article>"));
    }

    #[test]
    fn p1175_1_protege_espaco_entre_inline_vazios() {
        let empty = |tag: &str| {
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag.into(),
                None,
                typst_core::entities::html::HtmlBody::None,
            )))
        };
        let content =
            Content::sequence(vec![empty("span"), Content::Space, empty("span")]);
        assert!(export_html(&content).unwrap().contains(
            "<p><span></span><span style=\"white-space: pre-wrap\">&#x20;</span><span></span></p>"
        ));
    }

    #[test]
    fn p1176_1_summary_block_e_boundary_top_level() {
        let empty = |tag: &str| {
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag.into(),
                None,
                typst_core::entities::html::HtmlBody::None,
            )))
        };
        let top = Content::sequence(vec![
            empty("datalist"),
            Content::Space,
            empty("datalist"),
            Content::Space,
            empty("summary"),
        ]);
        let html = export_html(&top).unwrap();
        assert!(html
            .contains("<datalist></datalist><datalist></datalist><summary></summary>"));
        assert!(!html.contains("white-space: pre-wrap"));

        let details = Content::HtmlElem(Arc::new(HtmlElem::new(
            "details".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(
                vec![empty("summary"), Content::Space, Content::text("Body")],
            ))),
        )));
        assert!(export_html(&details)
            .unwrap()
            .contains("<details><summary></summary>Body</details>"));
    }

    #[test]
    fn p1177_1_ruby_descarta_so_espaco_estrutural_rp_rt() {
        let elem = |tag: &str, body: Content| {
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag.into(),
                None,
                typst_core::entities::html::HtmlBody::Content(Box::new(body)),
            )))
        };
        let ruby = elem(
            "ruby",
            Content::sequence(vec![
                elem("rp", Content::text("(")),
                Content::Space,
                elem("rt", Content::text("T")),
                Content::Space,
                elem("rp", Content::text(")")),
            ]),
        );
        assert!(export_html(&ruby)
            .unwrap()
            .contains("<p><ruby><rp>(</rp><rt>T</rt><rp>)</rp></ruby></p>"));

        let counterexample = elem(
            "ruby",
            Content::sequence(vec![
                Content::text("A"),
                Content::Space,
                elem("rt", Content::text("T")),
                Content::Space,
                elem("span", Content::text("X")),
                Content::Space,
                Content::text("B"),
            ]),
        );
        assert!(export_html(&counterexample)
            .unwrap()
            .contains("<ruby>A <rt>T</rt> <span>X</span> B</ruby>"));
    }

    #[test]
    fn p1178_1_adota_html_e_body_unicos() {
        let elem = |tag: &str, body: Content| {
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag.into(),
                None,
                typst_core::entities::html::HtmlBody::Content(Box::new(body)),
            )))
        };
        let explicit_html = elem(
            "html",
            Content::sequence(vec![
                elem("head", elem("title", Content::text("T"))),
                elem("body", Content::text("B")),
            ]),
        );
        assert_eq!(
            export_html(&explicit_html).unwrap(),
            "<!DOCTYPE html><html><head><title>T</title></head><body>B</body></html>"
        );

        let body = elem("body", Content::text("B"));
        assert_eq!(
            export_html(&body).unwrap(),
            format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body>B</body></html>")
        );
    }

    #[test]
    fn p1178_1_rejeita_html_e_body_com_siblings() {
        let empty = |tag: &str| {
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag.into(),
                None,
                typst_core::entities::html::HtmlBody::None,
            )))
        };
        let html = Content::sequence(vec![Content::text("A"), empty("html")]);
        assert_eq!(
            export_html(&html).unwrap_err().message,
            "`<html>` element must be the only element in the document"
        );
        let bodies = Content::sequence(vec![empty("body"), empty("body")]);
        assert_eq!(
            export_html(&bodies).unwrap_err().message,
            "`<body>` element must be the only element in the document"
        );
    }

    #[test]
    fn p1178_1_title_usa_texto_pre_escapable_raw() {
        let title = Content::HtmlElem(Arc::new(HtmlElem::new(
            "title".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(
                vec![
                    Content::text("A  B"),
                    Content::linebreak(),
                    Content::text("C & < > \""),
                ],
            ))),
        )));
        assert!(export_html(&title)
            .unwrap()
            .contains("<title>A  B\nC &amp; &lt; > \"</title>"));

        let invalid = Content::HtmlElem(Arc::new(HtmlElem::new(
            "title".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::HtmlElem(
                Arc::new(HtmlElem::new(
                    "span".into(),
                    None,
                    typst_core::entities::html::HtmlBody::None,
                )),
            ))),
        )));
        assert_eq!(
            export_html(&invalid).unwrap_err().message,
            "HTML raw text element cannot have non-text children"
        );
    }
}
