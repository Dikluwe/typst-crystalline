//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html.md
//! @prompt-hash afe9f695
//! @layer L3

use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

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

pub fn export_html(content: &Content) -> Result<String, SourceDiagnostic> {
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
    for item in seq {
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
                out.push_str(&inline_html_body(body)?);
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

fn inline_html_body(content: &Content) -> Result<String, SourceDiagnostic> {
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
        if matches!(items[index], Content::Space)
            && (index > first && is_void_html(&items[index - 1])
                || index < last && is_void_html(&items[index + 1]))
        {
            continue;
        }
        out.push_str(&inline(&items[index])?);
    }
    Ok(out)
}

fn is_void_html(content: &Content) -> bool {
    matches!(content, Content::HtmlElem(elem) if VOID_TAGS.contains(&elem.tag.as_str()))
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
                typst_core::entities::html::HtmlBody::Content(Box::new(Content::text(text))),
            )))
        };
        let article = Content::HtmlElem(Arc::new(HtmlElem::new(
            "article".into(),
            None,
            typst_core::entities::html::HtmlBody::Content(Box::new(Content::sequence(vec![
                elem("address", "A"),
                Content::Space,
                elem("aside", "B"),
            ]))),
        )));
        assert!(export_html(&article)
            .unwrap()
            .contains("<article><address>A</address><aside>B</aside></article>"));
    }
}
