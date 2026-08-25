//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html.md
//! @prompt-hash 9d9b5836
//! @layer L3

use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

const PREFIX: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body>";

pub fn export_html(content: &Content) -> Result<String, SourceDiagnostic> {
    let body = match content {
        Content::Empty => String::new(),
        Content::Heading(h) => {
            format!("<h{}>{}</h{}>", h.level, inline(&h.body)?.trim(), h.level)
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
            Content::HtmlElem(_) => {
                flush(&mut out, &mut paragraph);
                out.push_str(&inline(item)?);
            }
            other => paragraph.push_str(&inline(other)?),
        }
    }
    flush(&mut out, &mut paragraph);
    Ok(out)
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
                    out.push_str("=\"");
                    out.push_str(&escape(value));
                    out.push('"');
                }
            }
            out.push('>');
            if let Some(body) = &elem.body {
                out.push_str(&inline(body)?);
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
            Some(Content::text("Olá & mundo")),
        )));
        let html = export_html(&Content::sequence(vec![elem])).unwrap();
        assert_eq!(
            html,
            format!(
                "{PREFIX}<article lang=\"pt\" title=\"a&amp;b\">Olá &amp; mundo</article></body></html>"
            )
        );
    }
}
