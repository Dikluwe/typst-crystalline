//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/inline.md
//! @prompt-hash 40a43968
//! @layer L3
//! @updated 2026-09-26

use super::encoding::{escape_attribute, escape_text, raw_text, title_text};
use super::table::html_table;
use super::tags::{RAW_TEXT_TAGS, VOID_TAGS};
use super::whitespace::inline_html_body;
use super::{HtmlFrameResolver, HtmlSerializationMode};
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

pub(super) fn inline(
    content: &Content,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
    Ok(match content {
        Content::Empty => String::new(),
        Content::Text(text) => escape_text(text, mode),
        Content::Space | Content::SpaceFromNewline => " ".into(),
        Content::SetPage { .. } => String::new(),
        Content::Sequence(seq) => seq
            .iter()
            .map(|item| inline(item, mode, resolver, chain))
            .collect::<Result<Vec<_>, _>>()?
            .concat(),
        Content::Strong(e) => {
            format!("<strong>{}</strong>", inline(&e.body, mode, resolver, chain)?)
        }
        Content::Emph(e) => {
            format!("<em>{}</em>", inline(&e.body, mode, resolver, chain)?)
        }
        Content::Styled(styled) => {
            let child_chain = chain.push_styles(&styled.styles);
            inline(&styled.body, mode, resolver, &child_chain)?
        }
        Content::Linebreak(_) => "<br>".into(),
        Content::Table(table) => html_table(table, mode, resolver, chain)?,
        Content::TableCell(cell) => inline(&cell.body, mode, resolver, chain)?,
        Content::TableHeader(header) => inline(&header.body, mode, resolver, chain)?,
        Content::TableFooter(footer) => inline(&footer.body, mode, resolver, chain)?,
        Content::HtmlFrame(elem) => match resolver {
            Some(resolve) => resolve(elem, chain)?,
            None => {
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    "html.frame must be resolved by the HTML compilation pipeline",
                ))
            }
        },
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
                        out.push_str(&escape_attribute(value, mode));
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
                } else if RAW_TEXT_TAGS.contains(&elem.tag.as_str()) {
                    out.push_str(&raw_text(body)?);
                } else {
                    out.push_str(&inline_html_body(
                        elem.tag.as_str(),
                        body,
                        mode,
                        resolver,
                        chain,
                    )?);
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
