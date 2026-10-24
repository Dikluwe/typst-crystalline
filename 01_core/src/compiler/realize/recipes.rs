//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/recipes.md
//! @prompt-hash fecd59bc
//! @layer L1
//! @updated 2026-10-18

use super::*;
use crate::compiler::eval::{
    rules::{apply, textual},
    selector_matching,
};
use crate::entities::content::{RealizedContent, ShowRecipeContent};
use crate::entities::show::Transformation;

pub(in crate::compiler) fn realize(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if let Content::Source(source) = content {
        let previous = std::mem::replace(
            engine.styles,
            super::reference_supplements::source_styles(engine.styles, source.span),
        );
        let result = realize(&source.body, ctx, engine);
        *engine.styles = previous;
        return result.map(|body| body.with_source_span(source.span)).map_err(
            |mut errors| {
                for error in &mut errors {
                    if error.span.is_detached() {
                        error.span = source.span;
                    }
                }
                errors
            },
        );
    }
    content.map_content_pre_with_origin(&mut |node, origin| {
        with_origin(origin, ctx, engine, |ctx, engine| visit(node, ctx, engine))
    })
}

fn with_origin<T>(
    origin: crate::entities::span::Span,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    action: impl FnOnce(&mut EvalContext, &mut Engine<'_>) -> SourceResult<T>,
) -> SourceResult<T> {
    if origin.is_detached() {
        return action(ctx, engine);
    }
    let previous = std::mem::replace(
        engine.styles,
        super::reference_supplements::source_styles(engine.styles, origin),
    );
    let result = action(ctx, engine);
    *engine.styles = previous;
    result
}

/// Rebuild only emitted children. This skips the already-decided root and
/// keeps Located identity and Equation boundaries across recursive recipes.
pub(super) fn realize_children(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if ctx.recipe_math_depth == 0 {
        if let Some(text) = symbol_text(content) {
            return realize(&text, ctx, engine);
        }
    }
    let old_math = ctx.recipe_math_depth;
    let mut first = true;
    let result = content.map_content_pre_with_origin(&mut |node, origin| {
        with_origin(origin, ctx, engine, |ctx, engine| {
            if std::mem::replace(&mut first, false) {
                match node {
                    Content::Located(located) => {
                        if ctx.target == crate::compiler::eval::EvalTarget::Paged
                            && super::figure::is_captionless(&located.body)
                        {
                            // The surrounding Realized keeps the canonical Figure
                            // Location; an actual Block/replacement keeps its own.
                            return realize_children(&located.body, ctx, engine)
                                .map(Some);
                        }
                        return realize_children(&located.body, ctx, engine).map(
                            |body| {
                                Some(Content::Located(Arc::new(LocatedContent {
                                    body,
                                    location: located.location,
                                })))
                            },
                        );
                    }
                    Content::Sequence(parts) => {
                        return realize_parts(parts.to_vec(), ctx, engine).map(Some)
                    }
                    Content::Styled(styled) => {
                        return styled_body(styled, ctx, engine).map(Some)
                    }
                    Content::Ref(_) => {
                        return super::reference_supplements::capture(node, ctx, engine)
                            .map(Some)
                    }
                    Content::Figure(figure) => {
                        return super::figure::realize_children(figure, ctx, engine)
                            .map(Some)
                    }
                    Content::SetPage(_) | Content::PageRun(_) => {
                        let captured = super::page_running::capture(node, ctx, engine);
                        return super::page_running::map_body(&captured, |body| {
                            realize(body, ctx, engine)
                        });
                    }
                    Content::Equation(_) => ctx.recipe_math_depth += 1,
                    Content::Source(_)
                    | Content::ShowRecipe(_)
                    | Content::Realized(_)
                    | Content::Empty
                    | Content::Text(_)
                    | Content::Space
                    | Content::SpaceFromNewline
                    | Content::Parbreak
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
                    | Content::MathSequence(_)
                    | Content::Symbol(_)
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
                    | Content::CounterDisplay(_)
                    | Content::CounterUpdate(_)
                    | Content::Outline(_)
                    | Content::OutlineEntry(_)
                    | Content::FigureCaption(_)
                    | Content::Image(_)
                    | Content::Shape(_)
                    | Content::Curve(_)
                    | Content::Transform(_)
                    | Content::Grid(_)
                    | Content::GridHeader(_)
                    | Content::GridFooter(_)
                    | Content::GridCell(_)
                    | Content::Align(_)
                    | Content::Place(_)
                    | Content::Flush(_)
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
                    | Content::Dynamic(_) => {}
                }
                return Ok(None);
            }
            visit(node, ctx, engine)
        })
    });
    ctx.recipe_math_depth = old_math;
    result
}

fn visit(
    node: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Content>> {
    // Kind-specific normalization precedes native show rules. Mathematical
    // symbols retain identity, and textual regex rules run per element there.
    if let Content::Symbol(text) = node {
        if ctx.recipe_math_depth == 0 {
            return realize(&Content::Text(text.clone()), ctx, engine).map(Some);
        }
        if engine.show_rules.iter().any(|rule| {
            !engine.active_guards.contains(&rule.id)
                && selector_matching::is_text_rule(&rule.selector)
                && !selector_matching::textual_match_ranges(&rule.selector, text)
                    .is_empty()
        }) {
            return textual::realize_captured_text(node.clone(), ctx, engine).map(Some);
        }
    }
    match node {
        Content::ContextBlock(block) => {
            let content = if super::context_occurrences::instantiating_template(ctx)
                && !super::reference_supplements::is_reference_supplement_producer(block)
            {
                Content::ContextBlock(super::context_occurrences::instantiate_context(
                    block, ctx,
                ))
            } else {
                node.clone()
            };
            return Ok(Some(super::prepare_context_recipe(content, ctx, engine)));
        }
        Content::ShowRecipe(recipe) => {
            if super::page_running::is_context_transport(recipe) {
                return Ok(Some(node.clone()));
            }
            return realize_pending(recipe, ctx, engine).map(Some);
        }
        Content::Realized(done) => return realize_completed(done, ctx, engine).map(Some),
        Content::Styled(styled) => {
            if super::reference_supplements::already_prepared(node) {
                return Ok(Some(node.clone()));
            }
            return styled_body(styled, ctx, engine).map(Some);
        }
        Content::Located(located) if table_root(&located.body) => {
            return super::realize_table_insertions(node.clone(), ctx, engine)
                .and_then(|prepared| realize_children(&prepared, ctx, engine))
                .map(Some);
        }
        Content::Table(table) if table.resolved.is_some() => return Ok(None),
        Content::TableCell(cell)
            if cell.stage()
                == Some(
                    crate::entities::elements::table_cell::TableCellStage::Complete,
                ) =>
        {
            return Ok(None)
        }
        Content::Table(_) | Content::TableCell(_) => {
            return super::realize_table_insertions(node.clone(), ctx, engine)
                .and_then(|prepared| realize_children(&prepared, ctx, engine))
                .map(Some);
        }
        Content::Empty if super::sequences::snapshot(engine).is_some() => {
            let captured = super::sequences::capture(node.clone(), ctx, engine);
            return realize(&captured, ctx, engine).map(Some);
        }
        Content::Sequence(parts) => {
            if super::sequences::snapshot(engine).is_some() {
                let captured = super::sequences::capture(node.clone(), ctx, engine);
                return realize(&captured, ctx, engine).map(Some);
            }
            if has_root_effect(node, engine, ctx)? {
                return realize_root(node, ctx, engine).map(Some);
            }
            return realize_parts(parts.to_vec(), ctx, engine).map(Some);
        }
        Content::SetPage(_) | Content::PageRun(_) => {
            if has_root_effect(node, engine, ctx)? {
                return realize_root(node, ctx, engine).map(Some);
            }
            let captured = super::page_running::capture(node, ctx, engine);
            return super::page_running::map_body(&captured, |body| {
                realize(body, ctx, engine)
            });
        }
        Content::Source(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
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
        | Content::Symbol(_)
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
        | Content::OutlineEntry(_)
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
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
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
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::Dynamic(_) => {}
    }
    if ctx.recipe_math_depth == 0 {
        if let Some(equation) = super::bare_math::prepare_root(node, engine.styles) {
            return realize(&equation, ctx, engine).map(Some);
        }
    }
    if has_root_effect(node, engine, ctx)? || extract_payload(node).is_some() {
        return realize_root(node, ctx, engine).map(Some);
    }
    if matches!(node, Content::Ref(_)) {
        return super::reference_supplements::capture(node, ctx, engine).map(Some);
    }
    if textual::can_group(node)
        && !(ctx.recipe_math_depth > 0 && matches!(node, Content::Symbol(_)))
    {
        return textual::realize_captured_text(node.clone(), ctx, engine).map(Some);
    }
    Ok(None)
}

fn table_root(mut node: &Content) -> bool {
    loop {
        node = node.source_body();
        if let Content::Located(located) = node {
            node = &located.body;
        } else {
            return matches!(node, Content::Table(_) | Content::TableCell(_));
        }
    }
}

fn is_par(mut node: &Content) -> bool {
    loop {
        node = node.source_body();
        if let Content::Located(located) = node {
            node = &located.body;
        } else {
            return matches!(node, Content::Par { .. });
        }
    }
}

fn eligible_root_rule(
    node: &Content,
    rule: &ShowRule,
    engine: &Engine<'_>,
    ctx: &EvalContext,
) -> bool {
    let protected = matches!(node.source_body(), Content::Located(located)
        if ctx.recipe_protected_locations.contains(&located.location));
    selector_matching::is_node_rule(&rule.selector)
        && !selector_matching::is_sequence_rule(&rule.selector)
        && (!selector_matching::is_par_rule(&rule.selector) || is_par(node))
        && ((extract_payload(node).is_some() && !protected)
            || !engine.active_guards.contains(&rule.id))
}

pub(in crate::compiler) fn has_root_effect(
    node: &Content,
    engine: &Engine<'_>,
    ctx: &EvalContext,
) -> SourceResult<bool> {
    let preview = super::figure::preview(node, engine.styles)?;
    Ok(engine.show_rules.iter().any(|rule| {
        eligible_root_rule(&preview, rule, engine, ctx)
            && selector_matching::selector_matches(&preview, &rule.selector)
    }))
}

fn realize_root(
    node: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let preview = super::figure::preview(node, engine.styles)?;
    let rules: Vec<_> = engine
        .show_rules
        .iter()
        .filter(|rule| {
            eligible_root_rule(&preview, rule, engine, ctx)
                && selector_matching::selector_matches(&preview, &rule.selector)
        })
        .cloned()
        .collect();
    let mut guards: Vec<_> = rules
        .iter()
        .filter(|rule| {
            matches!(rule.transform, Transformation::Style(_))
                && selector_matching::selector_matches(&preview, &rule.selector)
        })
        .map(|rule| rule.id)
        .collect();
    if let Some(rule) = rules.iter().rev().find(|rule| {
        !matches!(rule.transform, Transformation::Style(_))
            && selector_matching::selector_matches(&preview, &rule.selector)
    }) {
        guards.push(rule.id);
    }
    if rules.is_empty() && super::figure::is_figure(node) {
        let prepared = super::figure::prepare(node, ctx, engine)?;
        let located = super::locate(prepared, ctx);
        let visual = realize_children(&located, ctx, engine)?;
        return Ok(Content::Realized(Arc::new(RealizedContent {
            original: located,
            visual,
        })));
    }
    let result = apply::apply_show_rules_root(node.clone(), &rules, ctx, engine)?;
    if guards.is_empty() {
        return realize_children(&result, ctx, engine);
    }
    if ctx.occurrence_show_depth >= Route::MAX_SHOW_RULE_DEPTH {
        return Err(vec![SourceDiagnostic::error(
            node.source_span(),
            "maximum show rule depth exceeded",
        )
        .with_hint("maybe a show rule matches its own output")
        .with_hint("maybe there are too deeply nested elements")]);
    }
    let old_guards = engine.active_guards.len();
    engine.active_guards.extend(guards);
    ctx.occurrence_show_depth += 1;
    if ctx.full_error {
        ctx.occurrence_show_path.push(node.morph_canon());
    }
    let (original, product) = if let Content::Realized(done) = &result {
        (done.original.clone(), done.visual.clone())
    } else {
        (node.clone(), result)
    };
    let old_locations = ctx.recipe_protected_locations.len();
    if let Content::Located(located) = original.source_body() {
        ctx.recipe_protected_locations.push(located.location);
    }
    let visual = textual::coalesce_for_insertion(&product, ctx, engine)
        .and_then(|product| realize(&product, ctx, engine));
    ctx.recipe_protected_locations.truncate(old_locations);
    if ctx.full_error {
        ctx.occurrence_show_path.pop();
    }
    ctx.occurrence_show_depth -= 1;
    let result =
        visual.map(|visual| textual::complete_product(original, visual, ctx, engine));
    engine.active_guards.truncate(old_guards);
    result
}

fn styled_body(
    styled: &crate::entities::content::StyledContent,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let previous =
        std::mem::replace(engine.styles, engine.styles.push_styles(&styled.styles));
    let result = realize(&styled.body, ctx, engine)
        .map(|body| Content::styled(body, styled.styles.clone()));
    *engine.styles = previous;
    result
}

fn realize_parts(
    mut parts: Vec<Content>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if ctx.recipe_math_depth == 0 {
        for part in &mut parts {
            if let Some(text) = symbol_text(part) {
                *part = text;
            }
        }
    }
    textual::space::normalize_markup_spaces(&mut parts);
    let sequence_rules = super::sequences::snapshot(engine).is_some();
    let in_math = ctx.recipe_math_depth > 0;
    let groupable = |part: &Content| {
        !in_math
            && textual::can_group(part)
            && !(sequence_rules
                && matches!(part.source_body(), Content::Empty | Content::Sequence(_)))
    };
    let mut output = Vec::new();
    let mut start = 0;
    while start < parts.len() {
        if groupable(&parts[start]) && !has_root_effect(&parts[start], engine, ctx)? {
            let mut end = start + 1;
            while end < parts.len()
                && groupable(&parts[end])
                && !has_root_effect(&parts[end], engine, ctx)?
            {
                end += 1;
            }
            output.push(textual::realize_captured_text(
                Content::sequence(parts[start..end].to_vec()),
                ctx,
                engine,
            )?);
            start = end;
        } else {
            output.push(realize(&parts[start], ctx, engine)?);
            start += 1;
        }
    }
    Ok(Content::sequence(output))
}

fn realize_pending(
    pending: &ShowRecipeContent,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let frame = textual::enter_scope(ctx, engine);
    let rules = textual::compose_rules(pending, engine.show_rules);
    let previous_styles =
        std::mem::replace(engine.styles, textual::captured_styles(pending));
    let previous_rules = std::mem::replace(engine.show_rules, rules);
    let mut guards = engine.active_guards.clone();
    for id in &pending.guards {
        if !guards.contains(id) {
            guards.push(*id);
        }
    }
    if super::sequences::marked(pending) {
        guards.retain(|id| {
            !pending.rules.iter().any(|rule| {
                rule.id == *id && selector_matching::is_sequence_rule(&rule.selector)
            })
        });
    }
    let previous_guards = std::mem::replace(engine.active_guards, guards);
    let previous_file = std::mem::replace(&mut engine.current_file, pending.file);
    let previous_defer = std::mem::replace(&mut ctx.defer_show_rules, false);
    let previous_realizing = std::mem::replace(&mut ctx.realizing_show_rules, true);
    ctx.next_context_id = ctx.next_context_id.max(pending.context_id_floor);
    let result = if super::sequences::marked(pending) {
        super::sequences::realize_root(&pending.body, &pending.guards, ctx, engine)
    } else if pending.textual {
        textual::coalesce_for_insertion(&pending.body, ctx, engine).and_then(|body| {
            let parts = if let Content::Sequence(parts) = body.source_body() {
                parts.to_vec()
            } else {
                vec![body]
            };
            realize_parts(parts, ctx, engine)
        })
    } else if !pending.rules.is_empty()
        && pending
            .rules
            .iter()
            .all(|rule| selector_matching::is_par_rule(&rule.selector))
    {
        let body = if pending
            .rules
            .iter()
            .any(|rule| !engine.active_guards.contains(&rule.id))
        {
            crate::compiler::eval::rules::realize_paragraphs(pending.body.clone())
        } else {
            crate::compiler::eval::rules::realize_node(&pending.body)
        };
        realize(&body, ctx, engine)
    } else {
        realize(&pending.body, ctx, engine)
    };
    *engine.styles = previous_styles;
    *engine.show_rules = previous_rules;
    *engine.active_guards = previous_guards;
    engine.current_file = previous_file;
    ctx.defer_show_rules = previous_defer;
    ctx.realizing_show_rules = previous_realizing;
    textual::leave_scope(ctx, frame);
    result.map_err(|errors| super::anchor_recipe_errors(errors, &pending.styles))
}

fn realize_completed(
    done: &RealizedContent,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let original = done.original.source_body();
    if let Content::ShowRecipe(origin) = original {
        if textual::is_product(&Content::Realized(Arc::new(done.clone()))) {
            let frame = textual::enter_scope(ctx, engine);
            let rules = textual::compose_rules(origin, engine.show_rules);
            let previous_rules = std::mem::replace(engine.show_rules, rules);
            let previous_guards = engine.active_guards.clone();
            for id in &origin.guards {
                if !engine.active_guards.contains(id) {
                    engine.active_guards.push(*id);
                }
            }
            let old_locations = ctx.recipe_protected_locations.len();
            if let Content::Located(located) = origin.body.source_body() {
                ctx.recipe_protected_locations.push(located.location);
            }
            let result = realize(&done.visual, ctx, engine).map(|visual| {
                Content::Realized(Arc::new(RealizedContent {
                    original: done.original.clone(),
                    visual,
                }))
            });
            ctx.recipe_protected_locations.truncate(old_locations);
            *engine.show_rules = previous_rules;
            *engine.active_guards = previous_guards;
            textual::leave_scope(ctx, frame);
            return result;
        }
    }
    realize(&done.visual, ctx, engine).map(|visual| {
        Content::Realized(Arc::new(RealizedContent {
            original: done.original.clone(),
            visual,
        }))
    })
}

/// Normalize only a direct leaf, preserving its lexical origin. Containers keep
/// their own realization boundary, especially equations and styled subtrees.
fn symbol_text(content: &Content) -> Option<Content> {
    match content.source_body() {
        Content::Symbol(text) => {
            Some(Content::Text(text.clone()).with_source_span(content.source_span()))
        }
        _ => None,
    }
}
