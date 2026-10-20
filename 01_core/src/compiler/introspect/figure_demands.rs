//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/figure_demands.md
//! @prompt-hash 6da6484a
//! @layer L1
//! @updated 2026-10-19

use crate::entities::{
    content::Content,
    counter::CounterKey,
    introspector::{Introspector, TagIntrospector},
    location::Location,
    source_result::{SourceDiagnostic, SourceResult},
    span::Span,
    style_chain::StyleChain,
};

pub(super) struct FigureCounterDemand {
    pub(super) key: CounterKey,
    pub(super) location: Location,
    pub(super) span: Span,
}

pub(super) fn collect_figure_counter_demands(
    content: &Content,
    intr: &TagIntrospector,
    chain: &StyleChain,
) -> SourceResult<Vec<FigureCounterDemand>> {
    let mut out = Vec::new();
    visit(content, intr, chain, None, Span::detached(), &mut out)?;
    Ok(out)
}

fn visit(
    content: &Content,
    intr: &TagIntrospector,
    chain: &StyleChain,
    location: Option<Location>,
    span: Span,
    out: &mut Vec<FigureCounterDemand>,
) -> SourceResult<()> {
    if let Some(child) =
        crate::compiler::realize::reference_supplements::prepared_reference_supplement(
            content,
        )
    {
        visit(child, intr, chain, None, span, out)?;
        if let Content::Styled(carrier) = content {
            visit(&carrier.body, intr, chain, location, span, out)?;
        }
        return Ok(());
    }
    match content {
        Content::Source(e) => {
            let span = if e.span.is_detached() { span } else { e.span };
            visit(&e.body, intr, chain, location, span, out)?;
        }
        Content::Located(e) => visit(&e.body, intr, chain, Some(e.location), span, out)?,
        Content::ShowRecipe(e) => visit(&e.body, intr, chain, location, span, out)?,
        Content::Realized(e) => visit(&e.visual, intr, chain, location, span, out)?,
        Content::Sequence(parts) | Content::MathSequence(parts) => {
            for child in parts.iter() {
                visit(child, intr, chain, None, span, out)?;
            }
        }
        Content::Par { body } | Content::SmallCaps { body } => {
            visit(body, intr, chain, None, span, out)?
        }
        Content::HtmlElem(e) => {
            if let crate::entities::html::HtmlBody::Content(body) = &e.body {
                visit(body, intr, chain, None, span, out)?;
            }
        }
        Content::PageRun(e) => {
            if let Some(Some(body)) = &e.background {
                visit(body, intr, chain, None, span, out)?;
            }
            if let Some(crate::entities::page_running::PageMarginal::Content(body)) =
                &e.header
            {
                visit(body, intr, chain, None, span, out)?;
            }
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(crate::entities::page_running::PageMarginal::Content(body)) =
                &e.footer
            {
                visit(body, intr, chain, None, span, out)?;
            }
            if let Some(Some(body)) = &e.foreground {
                visit(body, intr, chain, None, span, out)?;
            }
        }
        Content::HtmlFrame(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Heading(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Title(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Strong(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Emph(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::ListItem(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::EnumItem(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Link(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Label(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Equation(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathStretch(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathDelimited(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathLr(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathMid(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathCancel(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathUnderline(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathOverline(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathClassOverride(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathLimitsOverride(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::MathStyled(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::PdfArtifact(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Underline(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Strike(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Overline(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Pad(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Hide(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Block(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Boxed(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Repeat(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Columns(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Transform(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::GridHeader(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::GridFooter(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::GridCell(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::TableHeader(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::TableFooter(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Footnote(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Align(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Place(e) => visit(&e.body, intr, chain, None, span, out)?,
        Content::Styled(e) => {
            let pushed = chain.push_styles(&e.styles);
            visit(&e.body, intr, &pushed, location, span, out)?;
        }
        Content::Figure(e) => {
            if let Some(kind) = &e.resolved_kind {
                if e.caption.is_some()
                    && !matches!(
                        chain.custom("figure.numbering"),
                        Some(crate::entities::value::Value::None)
                    )
                {
                    let location = location.ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            "figure counter snapshot could not be verified",
                        )]
                    })?;
                    out.push(FigureCounterDemand {
                        key: super::figure_counters::figure_counter_key(kind),
                        location,
                        span,
                    });
                }
            }
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.caption {
                if e.resolved_kind.is_some()
                    && matches!(
                        chain.custom("figure.numbering"),
                        None | Some(crate::entities::value::Value::Str(_))
                    )
                {
                    if let Some(Some(supplement)) = &e.resolved_supplement {
                        visit(supplement, intr, chain, None, span, out)?;
                    }
                }
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::FigureCaption(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.separator {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathFrac(e) => {
            visit(&e.num, intr, chain, None, span, out)?;
            visit(&e.den, intr, chain, None, span, out)?;
        }
        Content::MathAttach(e) => {
            visit(&e.base, intr, chain, None, span, out)?;
            for slot in [&e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br] {
                if let crate::entities::elements::math_attach::MathAttachSlot::Present(
                    c,
                ) = slot
                {
                    visit(c, intr, chain, None, span, out)?;
                }
            }
        }
        Content::MathRoot(e) => {
            if let Some(c) = &e.index {
                visit(c, intr, chain, None, span, out)?;
            }
            visit(&e.radicand, intr, chain, None, span, out)?;
        }
        Content::MathOverbrace(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathUnderbrace(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathUnderbracket(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathUnderparen(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathUndershell(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathOverbracket(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathOverparen(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathOvershell(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.annotation {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathMatrix(e) => {
            for row in &e.rows {
                for c in row {
                    visit(c, intr, chain, None, span, out)?;
                }
            }
        }
        Content::MathCases(e) => {
            for c in &e.children {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathVec(e) => {
            for c in &e.children {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathAccent(e) => visit(&e.base, intr, chain, None, span, out)?,
        Content::MathUnderover(e) => {
            visit(&e.base, intr, chain, None, span, out)?;
            for c in [&e.under, &e.over].into_iter().flatten() {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::MathOp(e) => visit(&e.text, intr, chain, None, span, out)?,
        Content::Terms(e) => {
            for c in &e.items {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::TermItem(e) => {
            visit(&e.term, intr, chain, None, span, out)?;
            visit(&e.description, intr, chain, None, span, out)?;
        }
        Content::Quote(e) => {
            visit(&e.body, intr, chain, None, span, out)?;
            if let Some(c) = &e.attribution {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::Stack(e) => {
            for c in e.children.iter() {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::Outline(e) => {
            if let Some(c) = &e.title {
                visit(c, intr, chain, None, span, out)?;
            }
            if let Some(c) = e.realized_body() {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::OutlineEntry(e) => {
            if let Ok(body) = e.body() {
                visit(&body, intr, chain, None, span, out)?;
            }
        }
        Content::Grid(e) => {
            for c in &e.cells {
                visit(c, intr, chain, None, span, out)?;
            }
            for c in [&e.header, &e.footer].into_iter().flatten() {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::Table(e) => {
            if let Some(plan) = &e.resolved {
                for c in &plan.cells {
                    visit(c, intr, chain, None, span, out)?;
                }
            } else {
                for c in &e.children {
                    visit(c, intr, chain, None, span, out)?;
                }
                for c in [&e.header, &e.footer].into_iter().flatten() {
                    visit(c, intr, chain, None, span, out)?;
                }
            }
            if let Some(c) = &e.caption {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::TableCell(e) => visit(e.visual(), intr, chain, None, span, out)?,
        Content::Bibliography(e) => {
            if let Some(c) = &e.title {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::Cite(e) => {
            if let Some(c) = &e.supplement {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::FootnoteEntry(e) => {
            visit(&e.note, intr, chain, None, span, out)?;
            if let Some(c) = &e.separator {
                visit(c, intr, chain, None, span, out)?;
            }
        }
        Content::Ref(e) => {
            if e.form == crate::entities::elements::r#ref::RefForm::Normal
                && intr.bib_entry_for_key(e.name.as_str()).is_none()
            {
                let label = crate::entities::label::Label(e.name.to_string());
                if let Some(target) = intr.query_by_label(&label) {
                    if let Some(Content::Figure(figure)) = intr.element_at(target) {
                        if let Some(kind) = &figure.resolved_kind {
                            let key = super::figure_counters::figure_counter_key(kind);
                            if intr.counters.actions().iter().any(|event| {
                                event.location == target && event.key.is_none()
                            }) {
                                out.push(FigureCounterDemand {
                                    key,
                                    location: target,
                                    span,
                                });
                            }
                        }
                    }
                }
            }
        }
        Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Raw(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathPrimes(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::SetPage(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::{LocatedContent, RealizedContent, SourceContent},
        elements::figure::{FigureElem, FigureKind},
        func::Func,
        style::Styles,
        value::Value,
    };
    use std::sync::Arc;

    fn prepared(body: Content) -> Content {
        Content::Figure(Arc::new(FigureElem {
            body,
            caption: Some(Content::text("caption")),
            kind: None,
            resolved_kind: Some(FigureKind::Element(Func::native(
                "image",
                crate::compiler::stdlib::native_image,
            ))),
            supplement: None,
            resolved_supplement: Some(Some(Content::text("Figure"))),
        }))
    }
    fn located(body: Content, location: Location) -> Content {
        Content::Located(Arc::new(LocatedContent { body, location }))
    }

    #[test]
    fn figure_demand_carries_source_style_location_through_only_transparent_root() {
        let intr = TagIntrospector::empty();
        let location = Location::from_raw(71);
        let span = Span::from_range(
            crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(7).unwrap(),
            ),
            4..11,
        );
        let content = Content::Source(Arc::new(SourceContent {
            span,
            body: located(
                Content::styled(prepared(Content::Empty), Styles::new()),
                location,
            ),
        }));
        let demands =
            collect_figure_counter_demands(&content, &intr, &StyleChain::default_chain())
                .unwrap();
        assert_eq!(demands.len(), 1);
        assert_eq!(demands[0].location, location);
        assert_eq!(demands[0].span, span);
        let disabled = Content::styled(
            content,
            Styles::new().push_custom("figure.numbering", Value::None),
        );
        assert!(collect_figure_counter_demands(
            &disabled,
            &intr,
            &StyleChain::default_chain()
        )
        .unwrap()
        .is_empty());
        let nested = located(prepared(prepared(Content::Empty)), location);
        let errors =
            collect_figure_counter_demands(&nested, &intr, &StyleChain::default_chain())
                .err()
                .unwrap();
        assert_eq!(
            errors[0].message, "figure counter snapshot could not be verified",
            "nested Figure cannot borrow parent identity"
        );
    }

    #[test]
    fn figure_demand_uses_realized_visual_and_ignores_metadata_and_document_fields() {
        let intr = TagIntrospector::empty();
        let hidden = prepared(Content::Empty);
        let content = Content::Sequence(
            vec![
                Content::Realized(Arc::new(RealizedContent {
                    original: hidden.clone(),
                    visual: Content::Empty,
                })),
                Content::Metadata(Arc::new(
                    crate::entities::elements::metadata::MetadataElem {
                        value: Box::new(Value::Content(hidden.clone())),
                    },
                )),
                Content::Document {
                    path: "test.pdf".into(),
                    body: Box::new(hidden.clone()),
                    format: None,
                    title: Some(Box::new(hidden)),
                    author: vec![],
                    date: None,
                    keywords: vec![],
                },
            ]
            .into(),
        );
        assert!(collect_figure_counter_demands(
            &content,
            &intr,
            &StyleChain::default_chain()
        )
        .unwrap()
        .is_empty());
    }

    #[test]
    fn figure_demand_outline_entry_uses_emitted_body_not_semantic_element_or_fill() {
        let intr = TagIntrospector::empty();
        let hidden = prepared(Content::Empty);
        let entry = Content::OutlineEntry(Arc::new(
            crate::entities::elements::outline_entry::OutlineEntryElem::new(
                1,
                hidden.clone(),
                Some(hidden),
                true,
            ),
        ));
        assert!(
            collect_figure_counter_demands(&entry, &intr, &StyleChain::default_chain())
                .unwrap()
                .is_empty(),
            "invalid outline entry emits no body"
        );
    }
}
