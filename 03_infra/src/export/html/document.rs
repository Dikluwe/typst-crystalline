//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/document.md
//! @prompt-hash dfd97583
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
        .map_content(&mut |node| Ok(project_visual_root(node)))
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

// Children are already mapped. Reclassify only a transported immediate root.
fn project_visual_root(content: &Content) -> Option<Content> {
    let body = |content: &Content| {
        project_visual_root(content).unwrap_or_else(|| content.clone())
    };
    match content {
        Content::Source(e) => Some(body(&e.body)),
        Content::Located(e) => Some(body(&e.body)),
        Content::ShowRecipe(e) => Some(body(&e.body)),
        Content::Label(e) => Some(body(&e.body)),
        Content::Realized(e) => {
            let visual = body(&e.visual);
            Some(if figure_origin(&e.original) {
                Content::Sequence(
                    vec![Content::Parbreak, visual, Content::Parbreak].into(),
                )
            } else {
                visual
            })
        }
        Content::State(_) | Content::StateUpdate(_) | Content::Metadata(_) => {
            Some(Content::Empty)
        }
        Content::Sequence(parts) => {
            parts.iter().any(|part| matches!(part, Content::Empty)).then(|| {
                Content::sequence(
                    parts
                        .iter()
                        .filter(|part| !matches!(part, Content::Empty))
                        .cloned()
                        .collect(),
                )
            })
        }
        Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
        | Content::Symbol(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Styled(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => None,
    }
}

// Keep only the measured Figure show-product boundary, never its consumed body.
fn figure_origin(mut content: &Content) -> bool {
    loop {
        if let Content::Source(source) = content {
            content = &source.body;
        } else if let Content::ShowRecipe(recipe) = content {
            content = &recipe.body;
        } else if let Content::Located(located) = content {
            content = &located.body;
        } else if let Content::Styled(styled) = content {
            content = &styled.body;
        } else {
            return matches!(content, Content::Figure(_));
        }
    }
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

fn inline_cell_visual(
    cell: &typst_core::entities::elements::table_cell::TableCellElem,
) -> &Content {
    cell.standalone
        .as_ref()
        .map_or(&cell.body, |snapshot| &snapshot.visual)
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
    if let Content::TableCell(cell) = content {
        return has_block(inline_cell_visual(cell));
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
        Content::TableCell(cell) => {
            render_body(inline_cell_visual(cell), mode, resolver, chain)?
        }
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
        Content::TableCell(cell) => append_block(
            inline_cell_visual(cell),
            mode,
            resolver,
            chain,
            out,
            paragraph,
            context,
        )?,
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

#[cfg(test)]
mod figure_product_morphology {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::{
        content::{LocatedContent, RealizedContent},
        locator::Locator,
        style::Styles,
    };

    fn product(visual: Content, figure: bool) -> Content {
        let original = if figure {
            Content::figure(Content::text("unrendered"), None, None, None)
        } else {
            Content::heading(1, Content::text("unrendered"))
        };
        Content::Realized(Arc::new(RealizedContent {
            original: Content::Located(Arc::new(LocatedContent {
                body: Content::styled(Box::new(original), Styles::default()),
                location: Locator::new().next(),
            })),
            visual,
        }))
    }
    fn paragraph_payloads(content: &Content) -> Vec<String> {
        let html = export_html_with_frame_resolver(
            content,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .unwrap();
        regex::Regex::new(r"<p>(.*?)</p>")
            .unwrap()
            .captures_iter(&html)
            .map(|capture| capture[1].to_string())
            .collect()
    }

    #[test]
    fn figure_html_show_products_keep_paragraphs_and_rich_inline_morphology() {
        let content = Content::sequence(vec![
            Content::text("A "),
            product(Content::text("X"), true),
            Content::text(" B "),
            product(Content::strong(Content::text("Y")), true),
            Content::text(" C"),
        ]);
        assert_eq!(
            paragraph_payloads(&content),
            ["A", "X", "B", "<strong>Y</strong>", "C"]
        );
        let adjacent = Content::sequence(vec![
            product(Content::text("X"), true),
            product(Content::text("Y"), true),
        ]);
        assert_eq!(paragraph_payloads(&adjacent), ["X", "Y"]);
    }

    #[test]
    fn figure_html_captured_showrecipe_original_keeps_real_source_paragraphs() {
        use typst_core::entities::{
            content::ShowRecipeContent, source::Source, style_chain::StyleChain,
        };
        let captured = |visual| {
            let base = product(visual, true);
            let Content::Realized(done) = base else { unreachable!() };
            Content::Realized(Arc::new(RealizedContent {
                original: Content::ShowRecipe(Arc::new(ShowRecipeContent {
                    body: done.original.clone(),
                    textual: true,
                    context_id_floor: 0,
                    rules: Arc::from([]),
                    styles: StyleChain::default_chain(),
                    guards: vec![],
                    file: Source::detached("").expect("valid Source test fixture").id(),
                })),
                visual: done.visual.clone(),
            }))
        };
        let content = Content::sequence(vec![
            captured(Content::text("raw")),
            Content::SpaceFromNewline,
            captured(Content::text("table")),
        ]);
        assert_eq!(paragraph_payloads(&content), ["raw", "table"]);
        let rich = Content::sequence(vec![
            Content::text("A "),
            captured(Content::strong(Content::text("X"))),
            Content::text(" B"),
        ]);
        assert_eq!(paragraph_payloads(&rich), ["A", "<strong>X</strong>", "B"]);
    }

    #[test]
    fn figure_html_empty_product_keeps_boundary_and_nonfigure_product_stays_inline() {
        let hidden = Content::sequence(vec![
            Content::text("A "),
            product(Content::Empty, true),
            Content::text(" B"),
        ]);
        assert_eq!(paragraph_payloads(&hidden), ["A", "B"]);
        let heading = Content::sequence(vec![
            Content::text("A "),
            product(Content::text("X"), false),
            Content::text(" B"),
        ]);
        assert_eq!(paragraph_payloads(&heading), ["A X B"]);
    }
}

#[cfg(test)]
mod batch_g_html_document_tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::{
        content::{LocatedContent, RealizedContent, SourceContent},
        elements::context_block::ContextBlockElem,
        func::Func,
        location::Location,
        locator::Locator,
        state_update::StateUpdate,
        value::Value,
    };

    fn located(body: Content, location: Location) -> Content {
        Content::Located(Arc::new(LocatedContent { body, location }))
    }
    fn html(body: &Content) -> Result<String, SourceDiagnostic> {
        export_html_with_frame_resolver(body, HtmlSerializationMode::Vanilla, false, None)
    }
    #[test]
    fn batch_g_html_located_markers_preserve_empty_visual_contribution() {
        let mut locator = Locator::new();
        let markers = [
            Content::state("k", Value::Int(1)),
            Content::state_update("k", StateUpdate::Set(Box::new(Value::Int(2)))),
            Content::metadata(Value::Int(3)),
        ];
        for marker in markers {
            for wrapped in [
                located(marker.clone(), locator.next()),
                located(located(marker.clone(), locator.next()), locator.next()),
                located(
                    Content::Source(Arc::new(SourceContent {
                        body: marker.clone(),
                        span: Span::detached(),
                    })),
                    locator.next(),
                ),
                located(Content::label("marker", marker.clone()), locator.next()),
            ] {
                let content = Content::sequence(vec![
                    Content::text("A"),
                    wrapped,
                    Content::Parbreak,
                    Content::heading(1, Content::text("H")),
                ]);
                let output = html(&content)
                    .expect("causal marker must not become unsupported visual content");
                assert!(output.contains("<p>A</p><h2>H</h2>"), "{output}");
            }
        }
    }
    #[test]
    fn batch_g_html_suppressed_realized_figure_keeps_visual_paragraphs() {
        let mut locator = Locator::new();
        let figure = Content::figure(Content::text("ORIGINAL"), None, None, None);
        let product = located(
            Content::Realized(Arc::new(RealizedContent {
                original: figure,
                visual: Content::strong(Content::text("VISUAL")),
            })),
            locator.next(),
        );
        let output = html(&Content::sequence(vec![
            Content::text("A"),
            product,
            Content::text("B"),
        ]))
        .unwrap();
        assert!(
            output.contains("<p>A</p><p><strong>VISUAL</strong></p><p>B</p>"),
            "{output}"
        );
        assert!(!output.contains("ORIGINAL"));
    }
    #[test]
    fn batch_g_html_located_label_keeps_body_and_unresolved_producers_reject() {
        let mut locator = Locator::new();
        let output = html(&located(
            Content::label("keep", Content::strong(Content::text("BODY"))),
            locator.next(),
        ))
        .unwrap();
        assert!(output.contains("<strong>BODY</strong>"), "{output}");
        let unresolved = Content::ContextBlock(Arc::new(ContextBlockElem {
            id: 99,
            closure: Func::native("must-not-run", |_, _, _, _| {
                panic!("export must not execute context")
            }),
        }));
        let error = html(&located(unresolved, locator.next())).unwrap_err();
        assert_eq!(error.message, "HTML export does not yet support this content");
    }
}

#[cfg(test)]
mod batch_g_cell_flow_tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::html::{HtmlBody, HtmlElem};

    fn projected_heading(body: Content) -> Content {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            "h2".into(),
            None,
            HtmlBody::Content(Box::new(body)),
        )))
    }

    fn document_body(content: &Content) -> String {
        let html = export_html_with_frame_resolver(
            content,
            HtmlSerializationMode::Vanilla,
            false,
            None,
        )
        .expect("supported Cell visual must export");
        html.strip_prefix(PREFIX)
            .expect("automatic document envelope")
            .strip_suffix("</body></html>")
            .expect("automatic document suffix")
            .to_owned()
    }

    #[test]
    fn batch_g_html_root_cell_uses_selected_block_flow() {
        use typst_core::contracts::world::World;
        let raw = Content::table_cell(
            projected_heading(Content::text("H")),
            None,
            None,
            None,
            None,
        );
        let world = crate::world::SystemWorld::for_stdin(
            std::path::PathBuf::from("/tmp"),
            String::new(),
        );
        let source = world.source(world.main()).unwrap();
        let table = Content::table(
            vec![],
            vec![],
            vec![Content::table_cell(Content::text("SEED"), None, None, None, None)],
        );
        let (realized, warnings) = typst_core::compiler::realize::realize_document(
            &table,
            &world,
            &source,
            &typst_core::entities::element_registry::ElementRegistry::new(),
            &typst_core::entities::introspector::TagIntrospector::empty(),
            false,
            typst_core::compiler::eval::EvalTarget::Html,
            typst_core::entities::compiler_features::Features::empty(),
        );
        assert!(warnings.is_empty());
        let mut prepared = None;
        realized
            .expect("public Table preparation")
            .visual
            .map_content(&mut |node| {
                if let Content::TableCell(cell) = node {
                    if cell.standalone.is_some() && cell.resolved.is_some() {
                        prepared = Some(cell.as_ref().clone());
                    }
                }
                Ok(None)
            })
            .expect("borrow prepared public snapshots");
        let mut prepared = prepared.expect("publicly prepared Cell has both snapshots");
        prepared.body = Content::text("WRONG-BODY");
        let mut resolved = prepared.resolved.as_ref().unwrap().as_ref().clone();
        resolved.visual = Content::text("WRONG-RESOLVED");
        prepared.resolved = Some(Arc::new(resolved));
        let mut standalone = prepared.standalone.as_ref().unwrap().as_ref().clone();
        standalone.visual = projected_heading(Content::text("H"));
        prepared.standalone = Some(Arc::new(standalone));
        for cell in [raw, Content::TableCell(Arc::new(prepared))] {
            assert_eq!(document_body(&cell), "<h2>H</h2>");
        }
    }

    #[test]
    fn batch_g_html_sequence_cell_flushes_neighboring_paragraphs() {
        let cell = Content::table_cell(
            projected_heading(Content::strong(Content::text("H"))),
            None,
            None,
            None,
            None,
        );
        let content = Content::sequence(vec![
            Content::text("Before"),
            cell,
            Content::text("After"),
        ]);
        assert_eq!(
            document_body(&content),
            "<p>Before</p><h2><strong>H</strong></h2><p>After</p>"
        );
    }

    #[test]
    fn batch_g_html_plain_cell_preserves_inline_paragraph() {
        let cell = Content::table_cell(
            Content::strong(Content::text("middle")),
            None,
            None,
            None,
            None,
        );
        let content = Content::sequence(vec![
            Content::text("Before "),
            cell,
            Content::text(" after"),
        ]);
        assert_eq!(
            document_body(&content),
            "<p>Before <strong>middle</strong> after</p>"
        );
    }

    #[test]
    fn batch_g_html_table_row_cell_keeps_td_boundary() {
        let cell = Content::table_cell(
            projected_heading(Content::text("ROW")),
            None,
            None,
            Some(2),
            Some(3),
        );
        let table = Content::table(vec![], vec![], vec![cell]);
        assert_eq!(
            document_body(&table),
            "<table><tbody><tr><td colspan=\"2\" rowspan=\"3\"><h2>ROW</h2></td></tr></tbody></table>"
        );
    }
}
