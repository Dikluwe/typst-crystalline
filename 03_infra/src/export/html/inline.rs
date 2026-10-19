//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/inline.md
//! @prompt-hash 6dc04158
//! @layer L3
//! @updated 2026-10-07

use super::encoding::{escape_attribute, escape_text, raw_text, title_text_with_context};
use super::table::html_table;
use super::tags::{RAW_TEXT_TAGS, VOID_TAGS};
use super::whitespace::inline_html_body_with_context;
use super::{HtmlFrameResolver, HtmlSerializationMode};
use typst_core::compiler::lang::quotes::{resolve_smart_quote, SmartQuoteContext};
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

pub(super) fn inline(
    content: &Content,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
    inline_with_context(content, mode, resolver, chain, &mut SmartQuoteContext::default())
}

pub(super) fn inline_with_context(
    content: &Content,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
    context: &mut SmartQuoteContext,
) -> Result<String, SourceDiagnostic> {
    Ok(match content {
        Content::Source(e) => {
            inline_with_context(&e.body, mode, resolver, chain, context)?
        }
        Content::ShowRecipe(e) => {
            inline_with_context(&e.body, mode, resolver, chain, context)?
        }
        Content::Located(e) => {
            inline_with_context(&e.body, mode, resolver, chain, context)?
        }
        Content::Realized(e) => {
            inline_with_context(&e.visual, mode, resolver, chain, context)?
        }
        Content::Empty => String::new(),
        Content::Raw(atom) => {
            if !atom.block
                && atom.lang.as_deref() == Some("typc")
                && matches!(atom.text.as_str(), "auto" | "true" | "false")
            {
                context.observe_text(&atom.text);
                format!("<code data-lang=\"typc\"><span style=\"color: #d73948\">{}</span></code>",escape_text(&atom.text,mode))
            } else {
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    "HTML export does not yet support this content",
                ));
            }
        }
        Content::Text(text) => {
            context.observe_text(text);
            escape_text(text, mode)
        }
        Content::Space | Content::SpaceFromNewline => {
            context.observe_text(" ");
            " ".into()
        }
        Content::SetPage { .. } => String::new(),
        Content::Sequence(seq) => seq
            .iter()
            .map(|item| inline_with_context(item, mode, resolver, chain, context))
            .collect::<Result<Vec<_>, _>>()?
            .concat(),
        Content::Strong(e) => {
            format!(
                "<strong>{}</strong>",
                inline_with_context(&e.body, mode, resolver, chain, context)?
            )
        }
        Content::Emph(e) => {
            format!(
                "<em>{}</em>",
                inline_with_context(&e.body, mode, resolver, chain, context)?
            )
        }
        Content::Styled(styled) => {
            let child_chain = chain.push_styles(&styled.styles);
            inline_with_context(&styled.body, mode, resolver, &child_chain, context)?
        }
        Content::Linebreak(_) => {
            context.observe_text("\n");
            "<br>".into()
        }
        Content::Parbreak => {
            context.reset_paragraph();
            String::new()
        }
        Content::SmartQuote(e) => {
            let glyph = resolve_smart_quote(context, e, chain, None)
                .map_err(|e| e.into_iter().next().expect("quote diagnostic"))?;
            escape_text(&glyph, mode)
        }
        Content::Table(table) => {
            context.reset_paragraph();
            let html = html_table(table, mode, resolver, chain)?;
            context.reset_paragraph();
            html
        }
        Content::TableCell(cell) => inline_with_context(
            cell.standalone.as_ref().map_or(&cell.body, |r| &r.visual),
            mode,
            resolver,
            chain,
            context,
        )?,
        Content::TableHeader(header) => {
            inline_with_context(&header.body, mode, resolver, chain, context)?
        }
        Content::TableFooter(footer) => {
            inline_with_context(&footer.body, mode, resolver, chain, context)?
        }
        Content::HtmlFrame(elem) => match resolver {
            Some(resolve) => {
                let html = resolve(elem, chain)?;
                context.observe_object();
                html
            }
            None => {
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    "html.frame must be resolved by the HTML compilation pipeline",
                ))
            }
        },
        Content::HtmlElem(elem) => {
            let is_block = super::tags::BLOCK_TAGS.contains(&elem.tag.as_str());
            if is_block {
                context.reset_paragraph();
            }
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
                context.observe_object();
                return Ok(out);
            }
            if let Some(body) = elem.body.content() {
                if elem.tag == "title" {
                    out.push_str(&title_text_with_context(
                        body,
                        chain,
                        &mut SmartQuoteContext::default(),
                    )?);
                } else if RAW_TEXT_TAGS.contains(&elem.tag.as_str()) {
                    out.push_str(&raw_text(body)?);
                } else {
                    out.push_str(&inline_html_body_with_context(
                        elem.tag.as_str(),
                        body,
                        mode,
                        resolver,
                        chain,
                        context,
                    )?);
                }
            }
            out.push_str("</");
            out.push_str(elem.tag.as_str());
            out.push('>');
            if is_block {
                context.reset_paragraph();
            }
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
