//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/inline.md
//! @prompt-hash bcc5faaa
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

// Complete recognition precedes output, so a refused stream cannot partly render.
fn numeric_typc_tokens(
    text: &str,
) -> Option<Vec<(typst_core::entities::syntax_kind::SyntaxKind, &str)>> {
    use typst_core::entities::syntax_kind::SyntaxKind as Kind;
    let mut lexer = typst_core::compiler::lexer::Lexer::new(
        text,
        typst_core::entities::syntax_mode::SyntaxMode::Code,
    );
    let mut tokens = Vec::new();
    let mut expecting_number = true;
    let mut sign_allowed = true;
    let mut has_number = false;
    loop {
        let start = lexer.cursor();
        let kind = lexer.next().0;
        if kind == Kind::End {
            break;
        }
        let token = &text[start..lexer.cursor()];
        if kind == Kind::Space && token.chars().all(|c| c == ' ') {
            tokens.push((kind, token));
            continue;
        }
        if matches!(kind, Kind::Numeric | Kind::Int | Kind::Float) && expecting_number {
            expecting_number = false;
            has_number = true;
            sign_allowed = true;
        } else if matches!(kind, Kind::Plus | Kind::Minus) {
            if expecting_number {
                if !sign_allowed {
                    return None;
                }
                sign_allowed = false;
            } else {
                expecting_number = true;
                sign_allowed = true;
            }
        } else {
            return None;
        }
        tokens.push((kind, token));
    }
    (has_number && !expecting_number).then_some(tokens)
}

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
            } else if !atom.block && atom.lang.as_deref() == Some("typc") {
                let tokens = numeric_typc_tokens(&atom.text).ok_or_else(|| {
                    SourceDiagnostic::error(
                        Span::detached(),
                        "HTML export does not yet support this content",
                    )
                })?;
                let mut code = String::from("<code data-lang=\"typc\">");
                for (kind, token) in tokens {
                    context.observe_text(token);
                    if kind == typst_core::entities::syntax_kind::SyntaxKind::Space {
                        code.push_str(&escape_text(token, mode));
                    } else {
                        let color =
                            if matches!(kind,
                            typst_core::entities::syntax_kind::SyntaxKind::Plus |
                            typst_core::entities::syntax_kind::SyntaxKind::Minus)
                            {
                                "#d73948"
                            } else {
                                "#b60157"
                            };
                        code.push_str(&format!(
                            "<span style=\"color: {color}\">{}</span>",
                            escape_text(token, mode)
                        ));
                    }
                }
                code.push_str("</code>");
                code
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

#[cfg(test)]
mod remaining_typc_length_tests {
    use super::*;
    #[test]
    fn remaining_typc_numeric_html_retains_code_and_palette() {
        for (text, expected) in [
            ("158.33pt", "<span style=\"color: #b60157\">158.33pt</span>"),
            ("12pt + 0.5em", "<span style=\"color: #b60157\">12pt</span> <span style=\"color: #d73948\">+</span> <span style=\"color: #b60157\">0.5em</span>"),
        ] {
            let html = inline(&Content::raw(text, Some("typc".into()), false),
                HtmlSerializationMode::Vanilla, None,
                &typst_core::entities::style_chain::StyleChain::default_chain()).unwrap();
            assert_eq!(html, format!("<code data-lang=\"typc\">{expected}</code>"));
        }
    }
}
