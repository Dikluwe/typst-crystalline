//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/content_methods.md
//! @prompt-hash 4192ce54
//! @layer L1
//! @updated 2026-10-16

use super::*;

/// Intercepta Location, Content e OutlineEntry após os demais métodos.
pub(super) fn try_content_methods(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>> {
    // **P792** — Métodos de `Location`: `loc.page()`, `loc.position()`,
    // `loc.page-numbering()` — paridade vanilla `location.rs #[scope]`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "page" | "position" | "page-numbering") {
            let target = super::call::resolve_receiver(
                access,
                cached_receiver,
                scopes,
                ctx,
                engine,
            )?;
            if let Value::Location(loc) = target {
                let _args = eval_args(call.args(), scopes, ctx, engine)?;
                return eval_location_method(*loc, method, ctx, call.span()).map(Some);
            }
        }
    }

    // **P829-B** — Métodos de instância de `Value::Content`:
    // `func`/`has`/`at`/`fields`/`location` — paridade vanilla `Content`
    // `#[scope]` (`foundations/content/mod.rs:510-590`). Corre depois de
    // todos os despachos legítimos acima (nenhum intercepta estes nomes para
    // Content) e antes do fallback P815, que de outra forma reportaria
    // `element strong has no method `func`` (medido m14/b1).
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "func" | "has" | "at" | "fields" | "location") {
            let target = super::call::resolve_receiver(
                access,
                cached_receiver,
                scopes,
                ctx,
                engine,
            )?;
            if let Value::Content(c) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_content_method(c, method, args, call.span())
                    .map(Some);
            }
            if let Value::LocatedContent(c, loc) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_introspected_content_method_at(
                    c,
                    *loc,
                    method,
                    args,
                    call.span(),
                )
                .map(Some);
            }
        }
    }

    // P1402/P1403 — métodos próprios da entry.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "body" | "prefix" | "page" | "inner" | "indented") {
            let target = super::call::resolve_receiver(
                access,
                cached_receiver,
                scopes,
                ctx,
                engine,
            )?;
            let content = match target {
                Value::Content(content) => Some(content),
                Value::LocatedContent(content, _) => Some(content.content()),
                _ => None,
            };
            if let Some(content @ crate::entities::content::Content::OutlineEntry(_)) =
                content.and_then(outline_entry_receiver)
            {
                let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                if method == "body" {
                    return bindings::eval_outline_entry_body_method(
                        content,
                        args,
                        call.span(),
                    )
                    .map(Some);
                }
                args.items.insert(0, Value::Content(content.clone()));
                args.occurrences = None;
                let world = engine.world;
                let current_file = engine.current_file;
                return match method {
                    "prefix" => {
                        native_outline_entry_prefix(ctx, &args, world, current_file)
                    }
                    "page" => native_outline_entry_page(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    "inner" => native_outline_entry_inner(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    "indented" => native_outline_entry_indented(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    _ => unreachable!("outline entry method already checked"),
                }
                .map(Some);
            }
        }
    }

    Ok(None)
}

fn outline_entry_receiver(
    content: &crate::entities::content::Content,
) -> Option<&crate::entities::content::Content> {
    use crate::entities::content::Content;
    let content = content.source_body();
    match content {
        Content::Located(located) => outline_entry_receiver(&located.body),
        Content::OutlineEntry(_) => Some(content),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Styled(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => None,
    }
}

#[cfg(test)]
mod v16_walk_receiver_tests {
    use super::*;
    use crate::entities::{
        content::{Content, LocatedContent, RealizedContent},
        file_id::FileId,
        locator::Locator,
        span::Span,
    };
    use std::sync::Arc;

    fn source(body: Content) -> Content {
        body.with_source_span(Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            2..5,
        ))
    }
    fn located(body: Content) -> Content {
        Content::Located(Arc::new(LocatedContent {
            location: Locator::new().next(),
            body,
        }))
    }

    #[test]
    fn v16_walk_receiver_resolves_borrowed_entry_through_source_and_located() {
        let entry =
            Content::outline_entry(1, Content::heading(1, Content::text("h")), None);
        for content in
            [entry.clone(), source(located(entry.clone())), located(source(entry))]
        {
            let resolved = outline_entry_receiver(&content).expect("entry receiver lost");
            let Content::OutlineEntry(entry) = resolved else { panic!("entry lost") };
            assert_eq!(entry.level, 1);
            assert_eq!(entry.body().unwrap().plain_text(), "h");
        }
    }

    #[test]
    fn v16_walk_receiver_does_not_unwrap_other_semantic_nodes() {
        let entry =
            Content::outline_entry(1, Content::heading(1, Content::text("h")), None);
        for content in [
            Content::text("x"),
            source(located(Content::strong(entry.clone()))),
            Content::Realized(Arc::new(RealizedContent {
                original: entry,
                visual: Content::text("v"),
            })),
        ] {
            assert!(outline_entry_receiver(&content).is_none());
        }
    }
}
