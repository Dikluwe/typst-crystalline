//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/document.md
//! @prompt-hash 70e739df
//! @layer L3
//! @updated 2026-10-07

use super::inline::{inline, inline_with_context};
use super::pretty::pretty_html;
use super::tags::should_group_into_paragraph;
use super::whitespace::top_level_space_between;
use super::{HtmlFrameResolver, HtmlSerializationMode};
use typst_core::compiler::lang::quotes::SmartQuoteContext;
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
    let visual = content
        .map_content(&mut |node| {
            Ok(match node {
                Content::Located(e) => Some(e.body.clone()),
                Content::Realized(e) => Some(e.visual.clone()),
                Content::ShowRecipe(e) => Some(e.body.clone()),
                Content::Label(e) => Some(e.body.clone()),
                Content::State(_) | Content::StateUpdate(_) | Content::Metadata(_) => {
                    Some(Content::Empty)
                }
                _ => None,
            })
        })
        .map_err(|errors| {
            errors.into_iter().next().expect("map error contains diagnostic")
        })?;
    let content = &visual;
    let chain = typst_core::entities::style_chain::StyleChain::default_chain();
    let compact =
        if let Some((root, root_chain)) = explicit_document_root(content, &chain)? {
            if matches!(root, Content::HtmlElem(elem) if elem.tag == "html") {
                Ok(format!("{DOCTYPE}{}", inline(root, mode, resolver, &root_chain)?))
            } else {
                Ok(format!(
                    "{AUTO_ROOT_PREFIX}{}</html>",
                    inline(root, mode, resolver, &root_chain)?
                ))
            }
        } else {
            let body = render_body(content, mode, resolver, &chain)?;
            Ok(format!("{PREFIX}{body}</body></html>"))
        };

    compact.map(|html| if pretty { pretty_html(&html) } else { html })
}

fn explicit_document_root<'a>(
    content: &'a Content,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<
    Option<(&'a Content, typst_core::entities::style_chain::StyleChain)>,
    SourceDiagnostic,
> {
    fn collect<'a>(
        content: &'a Content,
        chain: &typst_core::entities::style_chain::StyleChain,
        nodes: &mut Vec<(&'a Content, typst_core::entities::style_chain::StyleChain)>,
    ) {
        match content {
            Content::Empty | Content::Space | Content::SpaceFromNewline => {}
            Content::Sequence(parts) => {
                for part in parts.iter() {
                    collect(part, chain, nodes);
                }
            }
            Content::Source(e) => collect(&e.body, chain, nodes),
            Content::Styled(e) => collect(&e.body, &chain.push_styles(&e.styles), nodes),
            other => nodes.push((other, chain.clone())),
        }
    }
    let mut nodes = Vec::new();
    collect(content, chain, &mut nodes);
    for (node, scoped_chain) in &nodes {
        if let Content::HtmlElem(elem) = node {
            if matches!(elem.tag.as_str(), "html" | "body") {
                if nodes.len() == 1 {
                    return Ok(Some((node, scoped_chain.clone())));
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

fn has_block(content: &Content) -> bool {
    if matches!(
        content,
        Content::Heading(_)
            | Content::Parbreak
            | Content::Par { .. }
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Table(_)
    ) {
        return true;
    }
    if let Content::Sequence(parts) = content {
        return parts.iter().any(has_block);
    }
    if let Content::Source(e) = content {
        return has_block(&e.body);
    }
    if let Content::Styled(e) = content {
        return has_block(&e.body);
    }
    false
}

fn heading_html(
    heading: &typst_core::entities::elements::heading::HeadingElem,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
    let body = inline(&heading.body, mode, resolver, chain)?;
    let level = u16::from(heading.level) + 1;
    Ok(if heading.level < 6 {
        format!("<h{level}>{}</h{level}>", body.trim())
    } else {
        format!("<div role=\"heading\" aria-level=\"{level}\">{}</div>", body.trim())
    })
}

fn render_body(
    content: &Content,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
    Ok(match content {
        Content::Source(e) => render_body(&e.body, mode, resolver, chain)?,
        Content::Empty => String::new(),
        Content::Heading(h) => heading_html(h, mode, resolver, chain)?,
        Content::Styled(e) => {
            render_body(&e.body, mode, resolver, &chain.push_styles(&e.styles))?
        }
        Content::Sequence(seq) if seq.iter().any(has_block) => {
            block_sequence(seq, mode, resolver, chain)?
        }
        Content::HtmlElem(elem) if should_group_into_paragraph(elem.tag.as_str()) => {
            format!("<p>{}</p>", inline(content, mode, resolver, chain)?)
        }
        Content::HtmlElem(_) => inline(content, mode, resolver, chain)?,
        Content::HtmlFrame(_) => inline(content, mode, resolver, chain)?,
        Content::Table(_) => inline(content, mode, resolver, chain)?,
        Content::Par { body } => {
            format!("<p>{}</p>", inline(body, mode, resolver, chain)?.trim())
        }
        other => format!("<p>{}</p>", inline(other, mode, resolver, chain)?.trim()),
    })
}

fn flush_paragraph(out: &mut String, paragraph: &mut String) {
    if !paragraph.trim().is_empty() {
        out.push_str("<p>");
        out.push_str(paragraph.trim());
        out.push_str("</p>");
        paragraph.clear();
    }
}

fn append_sequence(
    seq: &[Content],
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
    out: &mut String,
    paragraph: &mut String,
    context: &mut SmartQuoteContext,
) -> Result<(), SourceDiagnostic> {
    for (index, item) in seq.iter().enumerate() {
        if matches!(item, Content::Space | Content::SpaceFromNewline) {
            let space = top_level_space_between(seq, index);
            if !space.is_empty() {
                context.observe_text(" ");
            }
            paragraph.push_str(space);
        } else {
            append_block(item, mode, resolver, chain, out, paragraph, context)?;
        }
    }
    Ok(())
}

fn append_block(
    item: &Content,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
    out: &mut String,
    paragraph: &mut String,
    context: &mut SmartQuoteContext,
) -> Result<(), SourceDiagnostic> {
    match item {
        Content::Source(e) => {
            append_block(&e.body, mode, resolver, chain, out, paragraph, context)?
        }
        Content::Sequence(parts) => {
            append_sequence(parts, mode, resolver, chain, out, paragraph, context)?
        }
        Content::Styled(e) => append_block(
            &e.body,
            mode,
            resolver,
            &chain.push_styles(&e.styles),
            out,
            paragraph,
            context,
        )?,
        Content::Parbreak => {
            flush_paragraph(out, paragraph);
            context.reset_paragraph();
        }
        Content::Heading(h) => {
            flush_paragraph(out, paragraph);
            context.reset_paragraph();
            out.push_str(&heading_html(h, mode, resolver, chain)?);
        }
        Content::Par { body } => {
            flush_paragraph(out, paragraph);
            context.reset_paragraph();
            out.push_str(&format!(
                "<p>{}</p>",
                inline(body, mode, resolver, chain)?.trim()
            ));
        }
        Content::HtmlElem(elem) if should_group_into_paragraph(elem.tag.as_str()) => {
            paragraph
                .push_str(&inline_with_context(item, mode, resolver, chain, context)?);
        }
        Content::HtmlElem(_) => {
            flush_paragraph(out, paragraph);
            context.reset_paragraph();
            out.push_str(&inline(item, mode, resolver, chain)?);
        }
        Content::HtmlFrame(_) => {
            flush_paragraph(out, paragraph);
            context.reset_paragraph();
            out.push_str(&inline(item, mode, resolver, chain)?);
        }
        Content::Table(_) => {
            flush_paragraph(out, paragraph);
            context.reset_paragraph();
            out.push_str(&inline(item, mode, resolver, chain)?);
        }

        other => paragraph
            .push_str(&inline_with_context(other, mode, resolver, chain, context)?),
    }
    Ok(())
}

fn block_sequence(
    seq: &[Content],
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
    let mut out = String::new();
    let mut paragraph = String::new();
    let mut context = SmartQuoteContext::default();
    append_sequence(seq, mode, resolver, chain, &mut out, &mut paragraph, &mut context)?;
    flush_paragraph(&mut out, &mut paragraph);
    Ok(out)
}

#[cfg(test)]
mod heading_flow_regressions {
    use super::*;
    use typst_core::entities::style::Styles;

    #[test]
    fn styled_heading_is_a_block_and_keeps_neighboring_paragraphs() {
        let content = Content::styled(
            Box::new(Content::sequence(vec![
                Content::text("Before"),
                Content::Parbreak,
                Content::heading(1, Content::text("Title")),
                Content::Parbreak,
                Content::text("After"),
            ])),
            Styles::default(),
        );
        let html = export_html_with_frame_resolver(
            &content,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .unwrap();
        assert!(
            html.contains("<body><p>Before</p><h2>Title</h2><p>After</p></body>"),
            "{html}"
        );
    }

    #[test]
    fn a_styled_sequence_keeps_the_current_paragraph_until_a_heading() {
        let content = Content::sequence(vec![
            Content::text("Outer "),
            Content::styled(
                Box::new(Content::sequence(vec![
                    Content::text("Inner"),
                    Content::heading(1, Content::text("Title")),
                ])),
                Styles::default(),
            ),
            Content::text("Tail"),
        ]);
        let html = export_html_with_frame_resolver(
            &content,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .unwrap();
        assert!(html.contains("<p>Outer Inner</p><h2>Title</h2><p>Tail</p>"), "{html}");
    }

    #[test]
    fn heading_levels_preserve_section_semantics() {
        for level in 1..=6 {
            let content = Content::heading(level, Content::text("Title"));
            let html = export_html_with_frame_resolver(
                &content,
                HtmlSerializationMode::Vanilla,
                false,
                None,
            )
            .unwrap();
            let expected = if level < 6 {
                format!("<h{}>Title</h{}>", level + 1, level + 1)
            } else {
                "<div role=\"heading\" aria-level=\"7\">Title</div>".into()
            };
            assert!(html.contains(&expected), "{html}");
        }
    }

    #[test]
    fn a_styled_explicit_body_remains_the_only_document_root() {
        use typst_core::entities::html::{HtmlBody, HtmlElem};
        let root = Content::HtmlElem(std::sync::Arc::new(HtmlElem::new(
            "body".into(),
            None,
            HtmlBody::Content(Box::new(Content::text("Hello"))),
        )));
        let styled = Content::styled(Box::new(root.clone()), Styles::default());
        let html = export_html_with_frame_resolver(
            &styled,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .unwrap();
        assert_eq!(html.matches("<body>").count(), 1, "{html}");
        let invalid = Content::styled(
            Box::new(Content::sequence(vec![root, Content::text("extra")])),
            Styles::default(),
        );
        let error = export_html_with_frame_resolver(
            &invalid,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .unwrap_err();
        assert_eq!(
            error.message,
            "`<body>` element must be the only element in the document"
        );
    }

    #[test]
    fn inline_styles_do_not_create_extra_paragraphs() {
        let content = Content::sequence(vec![
            Content::text("Before "),
            Content::styled(Box::new(Content::text("Inside")), Styles::default()),
            Content::text(" After"),
        ]);
        let html = export_html_with_frame_resolver(
            &content,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .unwrap();
        assert!(html.contains("<p>Before Inside After</p>"), "{html}");
    }
}

#[cfg(test)]
mod a1_introspection_projection {
    use super::*;
    use typst_core::entities::state_update::StateUpdate;
    use typst_core::entities::value::Value;

    #[test]
    fn a1_html_preserves_labeled_body_and_hides_data_markers() {
        let content = Content::sequence(vec![
            Content::text("Before"),
            Content::state("k", Value::Int(1)),
            Content::state_update("k", StateUpdate::Set(Box::new(Value::Int(2)))),
            Content::metadata(Value::Int(3)),
            Content::label_auto("h", Content::heading(1, Content::text("Title"))),
            Content::label("t", Content::text("After")),
        ]);
        let html = export_html_with_frame_resolver(
            &content,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .unwrap();
        assert!(html.contains("<p>Before</p><h2>Title</h2><p>After</p>"), "{html}");
        let unsupported = Content::label("r", Content::reference("r"));
        assert!(export_html_with_frame_resolver(
            &unsupported,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .is_err());
    }
}
