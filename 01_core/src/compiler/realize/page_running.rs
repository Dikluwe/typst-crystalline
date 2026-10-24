//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/page_running.md
//! @prompt-hash 9b3dc536
//! @layer L1
//! @updated 2026-10-21

use crate::compiler::eval::{rules::textual, EvalContext};
use crate::compiler::introspect::page_running::prepare_with_root_chain;
use crate::compiler::layout::{PageRunningRequest, PageRunningResolution};
use crate::entities::{
    content::{Content, SetPageDelta, ShowRecipeContent},
    elements::page_run::PageRunElem,
    engine::Engine,
    func::{Func, FuncRepr},
    page_running::PageMarginal,
    source_result::{SourceDiagnostic, SourceResult},
    span::Span,
    style_chain::StyleChain,
    value::Value,
    world_types::Sink,
};
use comemo::Track;
use std::sync::{Arc, Weak};

#[derive(Default)]
pub(in crate::compiler) struct CaptureMemo {
    sets: Vec<Weak<SetPageDelta>>,
    runs: Vec<Weak<PageRunElem>>,
}

const OPERATOR_MARKER: &str = "eval.page.running.operator-marker";
const OPERATOR_TOKEN: &str = "eval.page.running.operator-token";

fn operator_marker(
    _: &mut EvalContext,
    _: &crate::entities::args::Args,
    _: &dyn crate::contracts::world::World,
    _: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "internal page operator marker is not callable",
    )])
}

pub(in crate::compiler) fn operator_capture_token(
    content: &Content,
) -> Option<Arc<crate::entities::version::Version>> {
    let Content::ShowRecipe(recipe) = content.source_body() else { return None };
    let Some(Value::Func(function)) = recipe.styles.custom(OPERATOR_MARKER) else {
        return None;
    };
    let FuncRepr::Native(native) = function.repr() else { return None };
    let expected: fn(
        &mut EvalContext,
        &crate::entities::args::Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = operator_marker;
    if native.namespace.is_some() || !std::ptr::fn_addr_eq(native.call, expected) {
        return None;
    }
    let Some(Value::Version(token)) = recipe.styles.custom(OPERATOR_TOKEN) else {
        return None;
    };
    Some(token.clone())
}

fn operator_styles(
    styles: &StyleChain,
    token: &Arc<crate::entities::version::Version>,
) -> StyleChain {
    styles
        .push_custom(
            OPERATOR_MARKER,
            Value::Func(Func::native("page-operator-marker", operator_marker)),
        )
        .push_custom(OPERATOR_TOKEN, Value::Version(token.clone()))
}

const CONTEXT_ENVIRONMENT: &str = "eval.page.running.context-environment";

fn context_environment_token(
    _: &mut EvalContext,
    _: &crate::entities::args::Args,
    _: &dyn crate::contracts::world::World,
    _: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "internal page context environment token is not callable",
    )])
}

pub(super) fn insertion_context_styles(styles: &StyleChain) -> StyleChain {
    styles.push_custom(
        CONTEXT_ENVIRONMENT,
        Value::Func(Func::native("page-context-environment", context_environment_token)),
    )
}

pub(super) fn retains_context_styles(styles: &StyleChain) -> bool {
    let Some(Value::Func(function)) = styles.custom(CONTEXT_ENVIRONMENT) else {
        return false;
    };
    let FuncRepr::Native(native) = function.repr() else { return false };
    let expected: fn(
        &mut EvalContext,
        &crate::entities::args::Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = context_environment_token;
    native.namespace.is_none() && std::ptr::fn_addr_eq(native.call, expected)
}

const CONTEXT_TRANSPORT: &str = "eval.page.running.prepared-context";

fn context_transport_token(
    _: &mut EvalContext,
    _: &crate::entities::args::Args,
    _: &dyn crate::contracts::world::World,
    _: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "internal prepared page context token is not callable",
    )])
}

pub(super) fn transport_context(
    content: Content,
    styles: crate::entities::style::Styles,
    ctx: &EvalContext,
    engine: &Engine<'_>,
) -> Content {
    let body = Content::styled(
        content,
        styles.push_custom(
            CONTEXT_TRANSPORT,
            Value::Func(Func::native("prepared-page-context", context_transport_token)),
        ),
    );
    Content::ShowRecipe(Arc::new(ShowRecipeContent {
        body,
        textual: false,
        context_id_floor: ctx.next_context_id,
        rules: Arc::from([]),
        styles: engine.styles.clone(),
        guards: Vec::new(),
        file: engine.current_file,
    }))
}

pub(super) fn is_context_transport(recipe: &ShowRecipeContent) -> bool {
    if recipe.textual || !recipe.rules.is_empty() || !recipe.guards.is_empty() {
        return false;
    }
    let Content::Styled(carrier) = recipe.body.source_body() else {
        return false;
    };
    let marker = carrier
        .styles
        .delta()
        .custom
        .iter()
        .rev()
        .find(|(key, _)| key == CONTEXT_TRANSPORT)
        .map(|(_, value)| value);
    let Some(Value::Func(function)) = marker else {
        return false;
    };
    let FuncRepr::Native(native) = function.repr() else {
        return false;
    };
    let expected: fn(
        &mut EvalContext,
        &crate::entities::args::Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = context_transport_token;
    if native.namespace.is_some() || !std::ptr::fn_addr_eq(native.call, expected) {
        return false;
    }
    let captured = carrier
        .styles
        .delta()
        .custom
        .iter()
        .rev()
        .find(|(key, _)| key == "eval.context.recipe")
        .map(|(_, value)| value);
    let Some(Value::Content(Content::ShowRecipe(snapshot))) = captured else {
        return false;
    };
    if !matches!(snapshot.body, Content::Empty) {
        return false;
    }
    let a = recipe.styles.raw_deltas().next();
    let b = snapshot.styles.raw_deltas().next();
    if !(a.zip(b).is_some_and(|(a, b)| std::ptr::eq(a, b))
        || (a.is_none() && b.is_none()))
    {
        return false;
    }
    let mut body = &carrier.body;
    loop {
        match body {
            Content::Source(origin) => body = &origin.body,
            Content::Located(located) => body = &located.body,
            Content::ContextBlock(_) => return true,
            Content::SetPage(_)
            | Content::PageRun(_)
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
            | Content::Dynamic(_) => return false,
        }
    }
}

enum Operational<'a> {
    Set(&'a SetPageDelta),
    Run(&'a PageRunElem),
}

fn operational(content: &Content) -> Option<Operational<'_>> {
    match content {
        Content::SetPage(page) => Some(Operational::Set(page)),
        Content::PageRun(page) => Some(Operational::Run(page)),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
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

/// This predicate inspects field presence; it executes no template descendant.
pub(in crate::compiler) fn needs_capture(content: &Content) -> bool {
    let mut found = false;
    content.visit_content_static(&mut |node| {
        if found {
            return false;
        }
        match operational(node) {
            Some(Operational::Set(_)) => {
                for_each_template(node, &mut |_| found = true);
                false
            }
            Some(Operational::Run(page)) => {
                for_each_template(node, &mut |_| found = true);
                if !found {
                    found = needs_capture(&page.body);
                }
                false
            }
            None => true,
        }
    });
    found
}

fn capture_template(
    content: &Content,
    ctx: &EvalContext,
    engine: &Engine<'_>,
    token: &Arc<crate::entities::version::Version>,
) -> Content {
    if let Content::ShowRecipe(existing) = content.source_body() {
        let mut recipe = (**existing).clone();
        recipe.styles = operator_styles(&recipe.styles, token);
        return Content::ShowRecipe(Arc::new(recipe))
            .with_source_span(content.source_span());
    }
    let prepared_alias = matches!(content.source_body(), Content::Located(_));
    let origin = match engine.styles.custom("eval.reference.origin") {
        Some(Value::Args(args)) => args.span,
        _ => content.source_span(),
    };
    Content::ShowRecipe(Arc::new(ShowRecipeContent {
        body: content.clone().with_source_span(origin),
        textual: false,
        context_id_floor: ctx.next_context_id,
        rules: if prepared_alias { Arc::from([]) } else { Arc::clone(engine.show_rules) },
        styles: operator_styles(engine.styles, token),
        guards: if prepared_alias { Vec::new() } else { engine.active_guards.clone() },
        file: engine.current_file,
    }))
}

/// Capture only the four operational fields, preserving every other field.
pub(super) fn capture(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &Engine<'_>,
) -> Content {
    let captured_output = match operational(content) {
        Some(Operational::Set(page)) => {
            ctx.page_running_captures.sets.iter().any(|weak| {
                weak.upgrade().is_some_and(|owned| std::ptr::eq(owned.as_ref(), page))
            })
        }
        Some(Operational::Run(page)) => {
            ctx.page_running_captures.runs.iter().any(|weak| {
                weak.upgrade().is_some_and(|owned| std::ptr::eq(owned.as_ref(), page))
            })
        }
        None => return content.clone(),
    };
    if captured_output {
        return content.clone();
    }
    let token = super::context_occurrences::next_capture_token(ctx);
    fn marginal(
        value: &mut Option<PageMarginal>,
        ctx: &EvalContext,
        engine: &Engine<'_>,
        token: &Arc<crate::entities::version::Version>,
    ) {
        if let Some(PageMarginal::Content(body)) = value {
            *body = Arc::new(capture_template(body, ctx, engine, token));
        }
    }
    match operational(content) {
        Some(Operational::Set(page)) => {
            let mut page = page.clone();
            marginal(&mut page.header, ctx, engine, &token);
            marginal(&mut page.footer, ctx, engine, &token);
            for layer in [&mut page.background, &mut page.foreground] {
                if let Some(Some(body)) = layer {
                    *body = Arc::new(capture_template(body, ctx, engine, &token));
                }
            }
            let page = Arc::new(page);
            ctx.page_running_captures.sets.push(Arc::downgrade(&page));
            Content::SetPage(page)
        }
        Some(Operational::Run(page)) => {
            let mut page = page.clone();
            marginal(&mut page.header, ctx, engine, &token);
            marginal(&mut page.footer, ctx, engine, &token);
            for layer in [&mut page.background, &mut page.foreground] {
                if let Some(Some(body)) = layer {
                    *body = capture_template(body, ctx, engine, &token);
                }
            }
            let page = Arc::new(page);
            ctx.page_running_captures.runs.push(Arc::downgrade(&page));
            Content::PageRun(page)
        }
        None => content.clone(),
    }
}

/// Stop automatic descent into templates while preserving the lexical body.
pub(in crate::compiler) fn map_body<F>(
    content: &Content,
    mut transform: F,
) -> SourceResult<Option<Content>>
where
    F: FnMut(&Content) -> SourceResult<Content>,
{
    match operational(content) {
        Some(Operational::Set(_)) => Ok(Some(content.clone())),
        Some(Operational::Run(page)) => {
            let mut page = page.clone();
            page.body = transform(&page.body)?;
            Ok(Some(Content::PageRun(Arc::new(page))))
        }
        None => Ok(None),
    }
}

/// Reservation sees dormant aliases without making them semantic occurrences.
pub(in crate::compiler) fn for_each_template(
    content: &Content,
    visit: &mut impl FnMut(&Content),
) {
    match operational(content) {
        Some(Operational::Set(page)) => {
            for marginal in [&page.header, &page.footer] {
                if let Some(PageMarginal::Content(body)) = marginal {
                    visit(body);
                }
            }
            for layer in [&page.background, &page.foreground] {
                if let Some(Some(body)) = layer {
                    visit(body);
                }
            }
        }
        Some(Operational::Run(page)) => {
            for marginal in [&page.header, &page.footer] {
                if let Some(PageMarginal::Content(body)) = marginal {
                    visit(body);
                }
            }
            for layer in [&page.background, &page.foreground] {
                if let Some(Some(body)) = layer {
                    visit(body);
                }
            }
        }
        None => {}
    }
}

fn require_frame(
    ctx: &EvalContext,
    span: Span,
    expected: Option<&Arc<()>>,
) -> SourceResult<()> {
    let Some(owner) = super::context_occurrences::allocation_owner(ctx) else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "page running requires a retained occurrence frame",
        )]);
    };
    let Some(expected) = expected else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "page running source has no allocation owner",
        )]);
    };
    if !Arc::ptr_eq(&owner, expected) {
        return Err(vec![SourceDiagnostic::error(
            span,
            "page running allocation frame belongs to another arena",
        )]);
    }
    Ok(())
}

/// Retain a proven persistent suffix without resolving or comparing style values.
fn rebase_chain(
    chain: &StyleChain,
    ambient: &StyleChain,
    effective: &StyleChain,
) -> Option<StyleChain> {
    let boundary = ambient.raw_deltas().next();
    let mut suffix = Vec::new();
    for delta in chain.raw_deltas() {
        if boundary.is_some_and(|base| std::ptr::eq(base, delta)) {
            return Some(
                suffix
                    .into_iter()
                    .rev()
                    .fold(effective.clone(), |chain: StyleChain, delta| {
                        chain.push(delta)
                    }),
            );
        }
        suffix.push(delta.clone());
    }
    boundary.is_none().then(|| {
        suffix
            .into_iter()
            .rev()
            .fold(effective.clone(), |chain: StyleChain, delta| chain.push(delta))
    })
}

fn rebase_recipe_styles(
    recipe: &ShowRecipeContent,
    ambient: &StyleChain,
    effective: &StyleChain,
) -> Option<StyleChain> {
    let mut styles = rebase_chain(&recipe.styles, ambient, effective)?;
    // Retain local-rule composition metadata, rebasing its ambient data only.
    if let Some(Value::Content(Content::ShowRecipe(baseline))) =
        recipe.styles.custom("eval.textual.baseline")
    {
        let mut baseline = (**baseline).clone();
        baseline.styles = rebase_chain(&baseline.styles, ambient, effective)?;
        styles = styles.push_custom(
            "eval.textual.baseline",
            Value::Content(Content::ShowRecipe(Arc::new(baseline))),
        );
    }
    Some(styles)
}

fn rebase_templates(
    content: &Content,
    ambient: &StyleChain,
    effective: &StyleChain,
) -> SourceResult<Content> {
    content.map_content_pre(&mut |node| {
        if matches!(node, Content::Located(_)) {
            return Ok(Some(node.clone()));
        }
        let Content::ShowRecipe(recipe) = node else { return Ok(None) };
        let Some(styles) = rebase_recipe_styles(recipe, ambient, effective) else {
            // An independent local capture keeps its own environment.
            return Ok(None);
        };
        let mut recipe = (**recipe).clone();
        recipe.styles = styles;
        recipe.body = rebase_templates(&recipe.body, ambient, effective)?;
        Ok(Some(Content::ShowRecipe(Arc::new(recipe))))
    })
}

pub fn realize_page_running(
    request: &PageRunningRequest,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<PageRunningResolution> {
    require_frame(
        ctx,
        request.span(),
        request.authority.source.allocation_owner.as_ref(),
    )?;
    if let Some(resolution) = &request.reusable {
        resolution.validate_owner()?;
        require_frame(
            ctx,
            request.span(),
            resolution.body.plan.allocation_owner.as_ref(),
        )?;
        if !request.matches(&resolution.request) {
            return Err(vec![SourceDiagnostic::error(
                request.span(),
                "invalid page running reusable request ownership",
            )]);
        }
        for diagnostic in resolution.realization_diagnostics.iter() {
            engine.sink.warn_diagnostic(diagnostic.clone());
        }
        return resolution.unbound();
    }
    let node =
        request
            .authority
            .source
            .node(request.authority.template)
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    request.span(),
                    "invalid page running template ownership",
                )]
            })?;
    let token = operator_capture_token(&node.content)
        .map(super::context_occurrences::RunningToken::Captured)
        .unwrap_or_else(|| {
            super::context_occurrences::RunningToken::Source(
                request.authority.source.clone(),
                request.authority.template,
            )
        });
    super::context_occurrences::with_running_cohort(
        ctx,
        super::context_occurrences::RunningStage::Realize,
        request.key(),
        token,
        |ctx| realize_actual(request, &node.content, ctx, engine),
    )
}

fn realize_actual(
    request: &PageRunningRequest,
    source: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<PageRunningResolution> {
    let insertion_styles = insertion_context_styles(&request.authority.chain);
    let template = if let Content::ShowRecipe(root) = source.source_body() {
        rebase_templates(source, &textual::captured_styles(root), &insertion_styles)?
    } else {
        source.clone()
    };
    let previous_styles = std::mem::replace(engine.styles, insertion_styles);
    let previous_rules = if let Content::ShowRecipe(root) = source.source_body() {
        Some(std::mem::replace(engine.show_rules, Arc::clone(&root.rules)))
    } else {
        None
    };
    let previous_realizing = std::mem::replace(&mut ctx.realizing_show_rules, true);
    let previous_defer = std::mem::replace(&mut ctx.defer_show_rules, false);
    let mut local_sink = Sink::new();
    let result = {
        let mut tracked_sink = local_sink.track_mut();
        let mut child = Engine {
            world: engine.world,
            font_metrics: engine.font_metrics,
            route: engine.route,
            styles: &mut *engine.styles,
            show_rules: &mut *engine.show_rules,
            active_guards: &mut *engine.active_guards,
            current_file: engine.current_file,
            sink: &mut tracked_sink,
        };
        super::context_occurrences::with_template(ctx, |ctx| {
            super::observe_occurrences(&template, ctx);
            let visual = super::recipes::realize(&template, ctx, &mut child)?;
            let visual = super::finish_occurrences(&visual, ctx)?;
            let visual = super::normalize_insertions(&visual)?;
            prepare_with_root_chain(&visual, ctx, child.styles)
        })
    };
    *engine.styles = previous_styles;
    if let Some(rules) = previous_rules {
        *engine.show_rules = rules;
    }
    ctx.realizing_show_rules = previous_realizing;
    ctx.defer_show_rules = previous_defer;
    let diagnostics = local_sink.into_diagnostics();
    for diagnostic in &diagnostics {
        engine.sink.warn_diagnostic(diagnostic.clone());
    }
    result.map(|body| {
        let mut resolution = PageRunningResolution::new(request, body);
        resolution.realization_diagnostics = diagnostics.into();
        resolution
    })
}

pub fn bind_page_running_content(
    resolution: &PageRunningResolution,
    expanded: Content,
    ctx: &mut EvalContext,
) -> SourceResult<PageRunningResolution> {
    resolution.validate_owner()?;
    require_frame(
        ctx,
        resolution.request.span(),
        resolution.request.authority.source.allocation_owner.as_ref(),
    )?;
    require_frame(
        ctx,
        resolution.request.span(),
        resolution.body.plan.allocation_owner.as_ref(),
    )?;
    let node = resolution
        .request
        .authority
        .source
        .node(resolution.request.authority.template)
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                resolution.request.span(),
                "invalid page running template ownership",
            )]
        })?;
    let token = operator_capture_token(&node.content)
        .map(super::context_occurrences::RunningToken::Captured)
        .unwrap_or_else(|| {
            super::context_occurrences::RunningToken::Source(
                resolution.request.authority.source.clone(),
                resolution.request.authority.template,
            )
        });
    super::context_occurrences::with_running_cohort(
        ctx,
        super::context_occurrences::RunningStage::Bind,
        resolution.key(),
        token,
        |ctx| {
            let body = prepare_with_root_chain(
                &expanded,
                ctx,
                &resolution.request.authority.chain,
            )?;
            resolution.with_bound_body(body)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::LocatedContent, file_id::FileId, layout_types::Length,
        location::Location, style_chain::StyleDelta,
    };
    use std::num::NonZeroU16;

    fn size(value: f64) -> StyleChain {
        StyleChain::empty().push(StyleDelta {
            size: Some(Length::pt(value)),
            ..StyleDelta::empty()
        })
    }

    #[test]
    fn physical_page_rebase_preserves_relative_and_explicit_inner_styles() {
        let creation = size(10.0);
        let relative = creation
            .push(StyleDelta { size: Some(Length::em(2.0)), ..StyleDelta::empty() });
        let explicit = creation.push(StyleDelta {
            size: Some(Length::pt(18.0)),
            ..StyleDelta::empty()
        });
        for (page, expected) in [(20.0, 40.0), (30.0, 60.0)] {
            let effective = size(page);
            let relative = rebase_chain(&relative, &creation, &effective).unwrap();
            assert_eq!(relative.size(), expected);
            assert_eq!(relative.raw_deltas().next().unwrap().size, Some(Length::em(2.0)));
            assert_eq!(
                rebase_chain(&explicit, &creation, &effective).unwrap().size(),
                18.0
            );
        }
        assert!(
            rebase_chain(&size(18.0), &creation, &size(20.0)).is_none(),
            "an independent capture is not an ancestor proof"
        );
    }

    #[test]
    fn physical_page_rebase_keeps_recipe_composition_and_source() {
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let span = Span::from_range(file, 8..14);
        let creation = size(10.0);
        let baseline = ShowRecipeContent {
            body: Content::Empty,
            textual: false,
            context_id_floor: 0,
            rules: Arc::from([]),
            styles: creation.clone(),
            guards: vec![],
            file,
        };
        let captured = creation.push_custom(
            "eval.textual.baseline",
            Value::Content(Content::ShowRecipe(Arc::new(baseline))),
        );
        let recipe = ShowRecipeContent {
            body: Content::text("HEADER").with_source_span(span),
            textual: false,
            context_id_floor: 77,
            rules: Arc::from([]),
            styles: captured,
            guards: vec![9],
            file,
        };
        let content = Content::ShowRecipe(Arc::new(recipe));
        let result = rebase_templates(&content, &creation, &size(30.0)).unwrap();
        let Content::ShowRecipe(recipe) = result else {
            panic!("captured header remains a recipe")
        };
        assert_eq!(textual::captured_styles(&recipe).size(), 30.0);
        assert!(recipe.styles.custom("eval.textual.baseline").is_some());
        assert_eq!(recipe.context_id_floor, 77);
        assert_eq!(recipe.guards, vec![9]);
        assert_eq!(recipe.file, file);
        assert_eq!(recipe.body.source_span(), span);
        assert_eq!(recipe.body.plain_text(), "HEADER");
    }

    #[test]
    fn physical_page_rebase_preserves_prepared_alias_and_dormant_fields() {
        let alias = Arc::new(LocatedContent {
            location: Location::from_raw(91),
            body: Content::heading(1, Content::text("ALIAS")),
        });
        let content = Content::Located(alias.clone());
        let result = rebase_templates(&content, &size(10.0), &size(30.0)).unwrap();
        let Content::Located(after) = result else {
            panic!("existing alias remains located")
        };
        assert!(Arc::ptr_eq(&alias, &after));
        let marker = Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(content))),
            ..SetPageDelta::default()
        }));
        assert!(needs_capture(&Content::strong(marker.clone())));
        let mut visits = 0;
        let unchanged = map_body(&marker, |_| {
            visits += 1;
            Ok(Content::Empty)
        })
        .unwrap()
        .unwrap();
        assert_eq!(visits, 0, "a SetPage template is not an emitted body child");
        let Content::SetPage(before) = marker else { unreachable!() };
        let Content::SetPage(after) = unchanged else { unreachable!() };
        assert!(Arc::ptr_eq(&before, &after));
        assert!(!needs_capture(&Content::SetPage(Arc::new(SetPageDelta::default()))));
    }

    fn domain_fixture(
        arena: &mut super::super::context_occurrences::ContextOccurrenceArena,
        ctx: &mut EvalContext,
    ) -> PageRunningResolution {
        use crate::compiler::introspect::page_running::{
            prepare_page_running_body, PlanEdge,
        };
        use crate::compiler::layout::page_running_requests::{
            EnvironmentPath, PageRunningKey, PageRunningSlot, RequestAuthority,
        };
        use crate::entities::layout_types::{Pt, Rect};
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let span = Span::from_range(file, 12..18);
        let source = Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(
                Content::text("HEADER").with_source_span(span),
            ))),
            ..SetPageDelta::default()
        }));
        let mut frame = arena.frame();
        arena.evaluate(&mut frame, ctx, |ctx| {
            let body = prepare_page_running_body(&source, ctx).unwrap();
            let template =
                body.plan.child(body.plan.root, PlanEdge::SetPageHeader).unwrap();
            let request = PageRunningRequest::new(RequestAuthority {
                source: body.plan.clone(),
                template,
                key: PageRunningKey {
                    page: std::num::NonZeroUsize::new(1).unwrap(),
                    slot: PageRunningSlot::Header,
                },
                environment: EnvironmentPath {
                    base: body.plan.node(template).unwrap().environment,
                    overlays: vec![],
                },
                chain: StyleChain::default_chain(),
                region: Rect { x: Pt(0.0), y: Pt(0.0), w: Pt(100.0), h: Pt(20.0) },
                span,
            });
            let contribution = prepare_with_root_chain(
                &Content::text("HEADER").with_source_span(span),
                ctx,
                &StyleChain::default_chain(),
            )
            .unwrap();
            PageRunningResolution::new(&request, contribution)
        })
    }

    #[test]
    fn physical_page_domain_rejects_foreign_arena_binding() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        let mut ctx = EvalContext::new();
        let mut source_arena = ContextOccurrenceArena::new(&Content::Empty);
        let resolution = domain_fixture(&mut source_arena, &mut ctx);
        let mut foreign = ContextOccurrenceArena::new(&Content::Empty);
        let mut frame = foreign.frame();
        let result = foreign.evaluate(&mut frame, &mut ctx, |ctx| {
            bind_page_running_content(&resolution, resolution.content().clone(), ctx)
        });
        let errors = result.expect_err(
            "an installed foreign frame cannot grant source allocation authority",
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].span, resolution.request.span());
        assert_eq!(
            errors[0].message,
            "page running allocation frame belongs to another arena"
        );
    }

    #[test]
    fn physical_page_domain_rejects_temporary_template_slots() {
        use super::super::context_occurrences::{with_template, ContextOccurrenceArena};
        let mut ctx = EvalContext::new();
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let resolution = domain_fixture(&mut arena, &mut ctx);
        let errors = with_template(&mut ctx, |ctx| {
            bind_page_running_content(&resolution, resolution.content().clone(), ctx)
        })
        .expect_err("temporary legacy slots are not an arena frame");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].span, resolution.request.span());
        assert_eq!(
            errors[0].message,
            "page running requires a retained occurrence frame"
        );
    }

    #[test]
    fn physical_page_domain_accepts_another_frame_in_same_arena() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        let mut ctx = EvalContext::new();
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let resolution = domain_fixture(&mut arena, &mut ctx);
        let mut plan_frame = arena.frame();
        let rebound = arena
            .evaluate(&mut plan_frame, &mut ctx, |ctx| {
                bind_page_running_content(&resolution, resolution.content().clone(), ctx)
            })
            .unwrap();
        assert_eq!(rebound.key(), resolution.key());
        assert_eq!(rebound.content().plain_text(), "HEADER");
        assert_eq!(rebound.content().source_span(), resolution.content().source_span());
        assert!(ctx.context_occurrence_slots.is_none(), "caller frame is restored");
    }

    #[test]
    fn physical_page_boundary_seed_survives_interleaved_frames() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        use crate::compiler::introspect::page_running::prepare_page_running_body;
        let mut ctx = EvalContext::new();
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let mut document = arena.frame();
        let first = arena
            .evaluate(&mut document, &mut ctx, |ctx| {
                prepare_page_running_body(&Content::Empty, ctx)
            })
            .unwrap();
        let mut sibling = arena.frame();
        arena
            .evaluate(&mut sibling, &mut ctx, |ctx| {
                prepare_page_running_body(
                    &Content::heading(1, Content::text("SIBLING")),
                    ctx,
                )
            })
            .unwrap();
        let replay = arena
            .evaluate(&mut document, &mut ctx, |ctx| {
                prepare_page_running_body(&Content::Empty, ctx)
            })
            .unwrap();
        assert_eq!(
            first.plan.allocation_start, replay.plan.allocation_start,
            "document start is the original retained boundary, not the later shared tail"
        );
    }

    struct CaptureWorld {
        source: crate::entities::source::Source,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for CaptureWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Ok(self.source.clone())
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }
    fn with_capture_engine<R>(
        ctx: &mut EvalContext,
        action: impl FnOnce(&mut EvalContext, &mut Engine<'_>) -> R,
    ) -> R {
        use crate::contracts::world::World;
        let world = CaptureWorld {
            source: crate::entities::source::Source::detached("")
                .expect("valid Source test fixture"),
            library: crate::entities::world_types::Library::new(),
            book: crate::entities::font_book::FontBook::new(),
        };
        let route = crate::entities::world_types::Route::root();
        let mut styles = StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &crate::compiler::layout::FixedMetrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked,
        };
        action(ctx, &mut engine)
    }
    fn captured_header_token(
        content: &Content,
    ) -> Arc<crate::entities::version::Version> {
        let Content::SetPage(page) = content else { panic!("captured operator") };
        let Some(PageMarginal::Content(body)) = &page.header else { panic!("header") };
        let Content::ShowRecipe(recipe) = body.source_body() else {
            panic!("captured header")
        };
        let Some(Value::Version(token)) =
            recipe.styles.custom("eval.page.running.operator-token")
        else {
            panic!("owning operational capture token")
        };
        token.clone()
    }
    fn header_request(
        source: &Content,
        arena: &mut super::super::context_occurrences::ContextOccurrenceArena,
        frame: &mut super::super::context_occurrences::ContextOccurrenceFrame,
        ctx: &mut EvalContext,
    ) -> PageRunningRequest {
        use crate::compiler::introspect::page_running::{
            prepare_page_running_body, PlanEdge,
        };
        use crate::compiler::layout::page_running_requests::{
            EnvironmentPath, PageRunningKey, PageRunningSlot, RequestAuthority,
        };
        use crate::entities::layout_types::{Pt, Rect};
        arena.evaluate(frame, ctx, |ctx| {
            let body = prepare_page_running_body(source, ctx).unwrap();
            let template =
                body.plan.child(body.plan.root, PlanEdge::SetPageHeader).unwrap();
            PageRunningRequest::new(RequestAuthority {
                source: body.plan.clone(),
                template,
                key: PageRunningKey {
                    page: std::num::NonZeroUsize::new(1).unwrap(),
                    slot: PageRunningSlot::Header,
                },
                environment: EnvironmentPath {
                    base: body.plan.node(template).unwrap().environment,
                    overlays: vec![],
                },
                chain: StyleChain::default_chain(),
                region: Rect { x: Pt(0.0), y: Pt(0.0), w: Pt(100.0), h: Pt(20.0) },
                span: Span::detached(),
            })
        })
    }
    fn heading_location(content: &Content) -> Location {
        use crate::entities::introspector::Introspector;
        crate::compiler::introspect::introspect_with_introspector(content)
            .query_by_kind(crate::entities::element_kind::ElementKind::Heading)[0]
    }

    #[test]
    fn physical_page_lineage_capture_is_occurrence_scoped_and_retry_stable() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        let template = Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(Content::text(
                "SAME RAW HEADER",
            )))),
            ..SetPageDelta::default()
        }));
        let mut arena = ContextOccurrenceArena::new(&template);
        let mut frame = arena.frame();
        let mut other = arena.frame();
        let mut ctx = EvalContext::new();
        let (a, b, again) = arena.evaluate(&mut frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| {
                let a = capture(&template, ctx, engine);
                let b = capture(&template, ctx, engine);
                let again = capture(&a, ctx, engine);
                (a, b, again)
            })
        });
        assert!(
            !Arc::ptr_eq(&captured_header_token(&a), &captured_header_token(&b)),
            "two fresh occurrences of the same raw operator cannot merge"
        );
        let (Content::SetPage(a_root), Content::SetPage(again_root)) = (&a, &again)
        else {
            unreachable!()
        };
        assert!(
            Arc::ptr_eq(a_root, again_root),
            "revisiting the captured output is idempotent"
        );
        let retry = arena.evaluate(&mut frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&template, ctx, engine))
        });
        assert!(
            Arc::ptr_eq(&captured_header_token(&a), &captured_header_token(&retry)),
            "same producer creation slot retains its token"
        );
        let other = arena.evaluate(&mut other, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&template, ctx, engine))
        });
        assert!(
            !Arc::ptr_eq(&captured_header_token(&a), &captured_header_token(&other)),
            "a different actual parent frame cannot inherit that token"
        );
    }

    #[test]
    fn physical_page_lineage_replays_ids_across_plans_and_resets_new_operator() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        let template = Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(Content::heading(
                1,
                Content::text("HEADER"),
            )))),
            ..SetPageDelta::default()
        }));
        let mut ctx = EvalContext::new();
        let (source, different) = with_capture_engine(&mut ctx, |ctx, engine| {
            (capture(&template, ctx, engine), capture(&template, ctx, engine))
        });
        let mut arena = ContextOccurrenceArena::new(&source);
        let mut body_frame = arena.frame();
        let mut actual_frame = arena.frame();
        let a = header_request(&source, &mut arena, &mut body_frame, &mut ctx);
        let first = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&a, ctx, engine)
                })
            })
            .unwrap();
        let b = header_request(&source, &mut arena, &mut body_frame, &mut ctx);
        assert!(
            !Arc::ptr_eq(&a.authority.source, &b.authority.source),
            "the test really rebuilt the source plan"
        );
        let replay = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&b, ctx, engine)
                })
            })
            .unwrap();
        assert_eq!(
            heading_location(first.content()),
            heading_location(replay.content()),
            "actual re-execution keeps only the certified allocation lineage"
        );
        let c = header_request(&different, &mut arena, &mut body_frame, &mut ctx);
        let changed = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&c, ctx, engine)
                })
            })
            .unwrap();
        assert_ne!(heading_location(first.content()), heading_location(changed.content()), "same physical key cannot keep a different owning operator's allocation cohort");
    }

    fn deferred_context(closure: Func) -> Content {
        Content::ContextBlock(Arc::new(
            crate::entities::elements::context_block::ContextBlockElem { id: 7, closure },
        ))
    }

    fn inert_context_closure() -> Func {
        Func::native("deferred-context-body", |_, _, _, _| {
            panic!("capture must not execute the contextual producer")
        })
    }

    fn context_producer(
        content: &Content,
    ) -> Arc<crate::entities::elements::context_block::ContextBlockElem> {
        let mut producers = Vec::new();
        content.visit_content_static(&mut |node| {
            if let Content::ContextBlock(block) = node {
                producers.push(block.clone());
            }
            true
        });
        assert_eq!(producers.len(), 1, "one actual deferred contextual descriptor");
        producers.pop().unwrap()
    }

    fn contextual_header(closure: Func) -> Content {
        Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(deferred_context(closure)))),
            ..SetPageDelta::default()
        }))
    }

    #[test]
    fn physical_page_context_producer_replays_across_source_plans() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        let template = contextual_header(inert_context_closure());
        let mut arena = ContextOccurrenceArena::new(&template);
        let mut capture_frame = arena.frame();
        let mut body_frame = arena.frame();
        let mut actual_frame = arena.frame();
        let mut other_frame = arena.frame();
        let mut ctx = EvalContext::new();
        let source = arena.evaluate(&mut capture_frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&template, ctx, engine))
        });
        let first_request =
            header_request(&source, &mut arena, &mut body_frame, &mut ctx);
        let first = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&first_request, ctx, engine)
                })
            })
            .unwrap();
        let next_request = header_request(&source, &mut arena, &mut body_frame, &mut ctx);
        assert!(!Arc::ptr_eq(
            &first_request.authority.source,
            &next_request.authority.source
        ));
        let replay = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&next_request, ctx, engine)
                })
            })
            .unwrap();
        let first = context_producer(first.content());
        let replay = context_producer(replay.content());
        assert_eq!(first.id, replay.id);
        assert!(Arc::ptr_eq(&first, &replay), "same certified insertion retains its actual immutable producer, not an execution result");
        let other_source = arena.evaluate(&mut other_frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&template, ctx, engine))
        });
        let other_request =
            header_request(&other_source, &mut arena, &mut body_frame, &mut ctx);
        let other = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&other_request, ctx, engine)
                })
            })
            .unwrap();
        let other = context_producer(other.content());
        assert_ne!(first.id, other.id);
        assert!(
            !Arc::ptr_eq(&first, &other),
            "another operator occurrence cannot inherit the contextual producer"
        );
    }

    #[test]
    fn physical_page_context_producer_rejects_changed_func_owner_at_same_slot() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        let template = contextual_header(inert_context_closure());
        let changed = contextual_header(inert_context_closure());
        let mut arena = ContextOccurrenceArena::new(&template);
        let mut capture_frame = arena.frame();
        let mut body_frame = arena.frame();
        let mut actual_frame = arena.frame();
        let mut ctx = EvalContext::new();
        let source = arena.evaluate(&mut capture_frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&template, ctx, engine))
        });
        let first_request =
            header_request(&source, &mut arena, &mut body_frame, &mut ctx);
        let first = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&first_request, ctx, engine)
                })
            })
            .unwrap();
        let changed_source = arena.evaluate(&mut capture_frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&changed, ctx, engine))
        });
        assert!(
            Arc::ptr_eq(
                &captured_header_token(&source),
                &captured_header_token(&changed_source)
            ),
            "the test retains the actual parent creation slot"
        );
        let changed_request =
            header_request(&changed_source, &mut arena, &mut body_frame, &mut ctx);
        let changed = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&changed_request, ctx, engine)
                })
            })
            .unwrap();
        let first = context_producer(first.content());
        let changed = context_producer(changed.content());
        assert_eq!(
            first.id, changed.id,
            "the allocation slot alone is deliberately unchanged"
        );
        assert!(!std::ptr::eq(first.closure.repr(), changed.closure.repr()), "the actual owned Func changed although its native call address/name are identical");
        assert!(
            !Arc::ptr_eq(&first, &changed),
            "ID, native call address and name cannot certify the producer"
        );
    }

    #[test]
    fn physical_page_context_producer_outside_running_cohort_stays_fresh() {
        use super::super::context_occurrences::{with_template, ContextOccurrenceArena};
        let source = deferred_context(inert_context_closure());
        let mut arena = ContextOccurrenceArena::new(&source);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let first = arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    with_template(ctx, |ctx| {
                        super::super::recipes::realize(&source, ctx, engine)
                    })
                })
            })
            .unwrap();
        let replay = arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    with_template(ctx, |ctx| {
                        super::super::recipes::realize(&source, ctx, engine)
                    })
                })
            })
            .unwrap();
        let first = context_producer(&first);
        let replay = context_producer(&replay);
        assert_eq!(first.id, replay.id);
        assert!(
            !Arc::ptr_eq(&first, &replay),
            "ordinary selected-template slots do not receive the running producer proof"
        );
    }

    #[test]
    fn physical_page_context_transport_keeps_chain_and_prepared_revisit() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        let template = contextual_header(inert_context_closure());
        let mut arena = ContextOccurrenceArena::new(&template);
        let mut capture_frame = arena.frame();
        let mut body_frame = arena.frame();
        let mut actual_frame = arena.frame();
        let mut ctx = EvalContext::new();
        let source = arena.evaluate(&mut capture_frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&template, ctx, engine))
        });
        for effective in [20.0, 30.0] {
            let mut request =
                header_request(&source, &mut arena, &mut body_frame, &mut ctx);
            Arc::make_mut(&mut request.authority).chain = size(effective);
            let prepared = arena
                .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                    with_capture_engine(ctx, |ctx, engine| {
                        realize_page_running(&request, ctx, engine)
                    })
                })
                .unwrap();
            let Content::ShowRecipe(transport) = prepared.content().source_body() else {
                panic!("complete contextual chain transport")
            };
            assert_eq!(transport.styles.size(), effective);
            assert!(
                transport.rules.is_empty()
                    && transport.guards.is_empty()
                    && !transport.textual
            );
            let original = context_producer(prepared.content());
            let replay = with_capture_engine(&mut ctx, |ctx, engine| {
                super::super::recipes::realize(prepared.content(), ctx, engine)
            })
            .unwrap();
            let Content::ShowRecipe(replay_transport) = replay.source_body() else {
                panic!("prepared transport retained")
            };
            assert!(
                Arc::ptr_eq(transport, replay_transport),
                "recipe re-entry preserves the actual prepared carrier"
            );
            let replay = context_producer(&replay);
            assert_eq!(original.id, replay.id);
            assert!(Arc::ptr_eq(&original, &replay));
            let table =
                Content::table(vec![], vec![], vec![Content::text("UNRELATED TABLE")]);
            let combined = Content::sequence(vec![prepared.content().clone(), table]);
            let table_replay = with_capture_engine(&mut ctx, |ctx, engine| {
                super::super::realize_table_insertions(combined, ctx, engine)
            })
            .unwrap();
            assert!(
                Arc::ptr_eq(&original, &context_producer(&table_replay)),
                "unrelated Table work cannot rematerialize the prepared Context"
            );
            let mut prepared_table = false;
            table_replay.visit_content_static(&mut |node| {
                if let Content::Table(table) = node {
                    prepared_table |= table.resolved.is_some();
                }
                true
            });
            assert!(prepared_table, "the unrelated actual Table is still prepared");
        }
    }

    #[test]
    fn physical_page_context_transport_does_not_capture_fresh_inherited_recipes() {
        use super::super::context_occurrences::ContextOccurrenceArena;
        use crate::entities::show::{Selector, ShowRule, Transformation};
        let template = contextual_header(inert_context_closure());
        let mut arena = ContextOccurrenceArena::new(&template);
        let mut capture_frame = arena.frame();
        let mut body_frame = arena.frame();
        let mut actual_frame = arena.frame();
        let mut ctx = EvalContext::new();
        let source = arena.evaluate(&mut capture_frame, &mut ctx, |ctx| {
            with_capture_engine(ctx, |ctx, engine| capture(&template, ctx, engine))
        });
        let request = header_request(&source, &mut arena, &mut body_frame, &mut ctx);
        let prepared = arena
            .evaluate(&mut actual_frame, &mut ctx, |ctx| {
                with_capture_engine(ctx, |ctx, engine| {
                    realize_page_running(&request, ctx, engine)
                })
            })
            .unwrap();
        let Content::ShowRecipe(transport) = prepared.content().source_body() else {
            panic!("prepared context transport")
        };
        let Content::Styled(carrier) = transport.body.source_body() else {
            panic!("body-local marker")
        };
        // This is the exact chain handed to Context execution by existing collection.
        let inherited = transport.styles.push_styles(&carrier.styles);
        let table_pending = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: Content::table(vec![], vec![], vec![Content::text("FRESH TABLE")]),
            textual: false,
            context_id_floor: 0,
            rules: Arc::from([]),
            styles: inherited.clone(),
            guards: vec![],
            file: transport.file,
        }));
        let table = with_capture_engine(&mut ctx, |ctx, engine| {
            super::super::recipes::realize(&table_pending, ctx, engine)
        })
        .unwrap();
        let mut prepared_table = false;
        table.visit_content_static(&mut |node| {
            if let Content::Table(table) = node {
                prepared_table |= table.resolved.is_some();
            }
            true
        });
        assert!(
            prepared_table,
            "a fresh Table recipe must execute despite the inherited marker"
        );
        let text_pending = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: Content::text("FRESH"),
            textual: true,
            context_id_floor: 0,
            rules: Arc::from([ShowRule {
                id: 432,
                selector: Selector::Text("FRESH".into()),
                transform: Transformation::Str("APPLIED".into()),
            }]),
            styles: inherited,
            guards: vec![],
            file: transport.file,
        }));
        let text = with_capture_engine(&mut ctx, |ctx, engine| {
            super::super::recipes::realize(&text_pending, ctx, engine)
        })
        .unwrap();
        assert_eq!(text.plain_text(),"APPLIED","fresh captured textual work cannot be mistaken for a prepared Context transport");
    }
}
