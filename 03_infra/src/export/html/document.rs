//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/document.md
//! @prompt-hash 06f11a36
//! @layer L3
//! @updated 2026-09-26

use super::inline::inline;
use super::pretty::pretty_html;
use super::tags::should_group_into_paragraph;
use super::whitespace::top_level_space_between;
use super::{HtmlFrameResolver, HtmlSerializationMode};
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

const DOCTYPE: &str = "<!DOCTYPE html>";
const AUTO_ROOT_PREFIX: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head>";
pub(super) const PREFIX: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body>";

pub(super) fn export_html_with_frame_resolver(
    content: &Content,
    mode: HtmlSerializationMode,
    pretty: bool,
    resolver: Option<&HtmlFrameResolver<'_>>,
) -> Result<String, SourceDiagnostic> {
    let chain = typst_core::entities::style_chain::StyleChain::default_chain();
    let compact = if let Some(root) = explicit_document_root(content)? {
        if matches!(root, Content::HtmlElem(elem) if elem.tag == "html") {
            Ok(format!("{DOCTYPE}{}", inline(root, mode, resolver, &chain)?))
        } else {
            Ok(format!(
                "{AUTO_ROOT_PREFIX}{}</html>",
                inline(root, mode, resolver, &chain)?
            ))
        }
    } else {
        let body = match content {
            Content::Empty => String::new(),
            Content::Heading(h) => {
                format!(
                    "<h{}>{}</h{}>",
                    h.level,
                    inline(&h.body, mode, resolver, &chain)?.trim(),
                    h.level
                )
            }
            Content::HtmlElem(elem) if should_group_into_paragraph(elem.tag.as_str()) => {
                format!("<p>{}</p>", inline(content, mode, resolver, &chain)?)
            }
            Content::HtmlElem(_) | Content::HtmlFrame(_) => {
                inline(content, mode, resolver, &chain)?
            }
            Content::Table(_) => inline(content, mode, resolver, &chain)?,
            Content::Sequence(seq)
                if seq.iter().any(|c| {
                    matches!(
                        c,
                        Content::Heading(_)
                            | Content::Parbreak
                            | Content::Par { .. }
                            | Content::HtmlElem(_)
                            | Content::HtmlFrame(_)
                            | Content::Table(_)
                    )
                }) =>
            {
                block_sequence(seq, mode, resolver, &chain)?
            }
            Content::Par { body } => {
                format!("<p>{}</p>", inline(body, mode, resolver, &chain)?.trim())
            }
            other => format!("<p>{}</p>", inline(other, mode, resolver, &chain)?.trim()),
        };
        Ok(format!("{PREFIX}{body}</body></html>"))
    };

    compact.map(|html| if pretty { pretty_html(&html) } else { html })
}

fn explicit_document_root(
    content: &Content,
) -> Result<Option<&Content>, SourceDiagnostic> {
    let nodes: Vec<&Content> = match content {
        Content::Sequence(items) => items
            .iter()
            .filter(|item| {
                !matches!(
                    item,
                    Content::Empty | Content::Space | Content::SpaceFromNewline
                )
            })
            .collect(),
        Content::Empty | Content::Space | Content::SpaceFromNewline => vec![],
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

fn block_sequence(
    seq: &[Content],
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
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
                    inline(&h.body, mode, resolver, chain)?.trim(),
                    h.level
                ));
            }
            Content::Par { body } => {
                flush(&mut out, &mut paragraph);
                out.push_str(&format!(
                    "<p>{}</p>",
                    inline(body, mode, resolver, chain)?.trim()
                ));
            }
            Content::HtmlElem(elem) => {
                if should_group_into_paragraph(elem.tag.as_str()) {
                    paragraph.push_str(&inline(item, mode, resolver, chain)?);
                } else {
                    flush(&mut out, &mut paragraph);
                    out.push_str(&inline(item, mode, resolver, chain)?);
                }
            }
            Content::HtmlFrame(_) => {
                flush(&mut out, &mut paragraph);
                out.push_str(&inline(item, mode, resolver, chain)?);
            }
            Content::Table(_) => {
                flush(&mut out, &mut paragraph);
                out.push_str(&inline(item, mode, resolver, chain)?);
            }
            Content::Space | Content::SpaceFromNewline => {
                paragraph.push_str(top_level_space_between(seq, index))
            }
            other => paragraph.push_str(&inline(other, mode, resolver, chain)?),
        }
    }
    flush(&mut out, &mut paragraph);
    Ok(out)
}
