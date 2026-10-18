//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/realize/table_cells.md
//! @prompt-hash 5e191664
//! @layer L1
//! @updated 2026-10-17

use super::*;
use crate::entities::elements::table::{TableElem, TableGridResolved};
use crate::entities::elements::table_cell::{
    CellFieldPresence, TableCellElem, TableCellProperties, TableCellResolved,
    TableCellStage, TableCellStandaloneResolved,
};
use crate::entities::layout_types::{Align2D, TrackSizing};
use crate::entities::value::Value;
use comemo::TrackedMut;

pub(super) fn needs_preparation(content: &Content) -> bool {
    let mut needs = false;
    let _ = content.map_content_pre(&mut |node| {
        match node {
            Content::Table(e) if e.resolved.is_none() => {
                needs = true;
                return Ok(Some(node.clone()));
            }
            Content::TableCell(e) if e.stage() != Some(TableCellStage::Complete) => {
                needs = true;
                return Ok(Some(node.clone()));
            }
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
            | Content::Dynamic(_) => {}
        }
        Ok(None)
    });
    needs
}

fn captured_recipe(
    styles: &crate::entities::style_chain::StyleChain,
) -> Option<Arc<crate::entities::content::ShowRecipeContent>> {
    let value = styles
        .custom("eval.table.local.recipe")
        .or_else(|| styles.custom("eval.context.recipe"))?;
    let content = match value {
        Value::Content(content) => content,
        Value::None
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => return None,
    };
    match content.source_body() {
        Content::ShowRecipe(recipe) => Some(recipe.clone()),
        Content::Source(_)
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

fn captured_rules(
    recipe: &crate::entities::content::ShowRecipeContent,
    engine: &Engine<'_>,
) -> Arc<[crate::entities::show::ShowRule]> {
    if engine.styles.custom("eval.table.local") != Some(&Value::Bool(true)) {
        return recipe.rules.clone();
    }
    let mut rules = engine.show_rules.to_vec();
    for rule in recipe.rules.iter() {
        if !rules.iter().any(|r| r.id == rule.id) {
            rules.push(rule.clone());
        }
    }
    rules.into()
}

fn insertion_guards(
    captured: &[crate::entities::show::RuleId],
    inherited: &[crate::entities::show::RuleId],
) -> Vec<crate::entities::show::RuleId> {
    let mut guards = inherited.to_vec();
    for id in captured {
        if !guards.contains(id) {
            guards.push(*id);
        }
    }
    guards
}

pub(super) fn realize(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !needs_preparation(&content) {
        return Ok(content);
    }
    let captured = captured_recipe(engine.styles);
    let old_rules = captured.as_ref().map(|r| {
        let rules = captured_rules(r, engine);
        std::mem::replace(engine.show_rules, rules)
    });
    let old_guards = captured.as_ref().map(|r| {
        let guards = insertion_guards(&r.guards, engine.active_guards);
        std::mem::replace(engine.active_guards, guards)
    });
    let old_file = engine.current_file;
    if let Some(r) = &captured {
        engine.current_file = r.file;
        ctx.next_context_id = ctx.next_context_id.max(r.context_id_floor);
    }
    let old_realizing = ctx.realizing_show_rules;
    let old_defer = ctx.defer_show_rules;
    ctx.realizing_show_rules = true;
    ctx.defer_show_rules = false;
    let result = walk(&content, ctx, engine);
    ctx.realizing_show_rules = old_realizing;
    ctx.defer_show_rules = old_defer;
    engine.current_file = old_file;
    if let Some(r) = old_rules {
        *engine.show_rules = r;
    }
    if let Some(g) = old_guards {
        *engine.active_guards = g;
    }
    result
}

fn walk(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let span = if matches!(content, Content::Source(_)) {
        Some(content.source_span())
    } else {
        None
    };
    let body = content.source_body();
    let result = if let Some(product) = walk_node(body, ctx, engine)? {
        Ok(product)
    } else {
        body.map_content_pre(&mut |node| walk_node(node, ctx, engine))
    };
    result.map(|product| match span {
        Some(span) => product.with_source_span(span),
        None => product,
    })
}

fn walk_node(
    node: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Content>> {
    let result = match node {
        Content::Source(e) => walk(&e.body, ctx, engine)?.with_source_span(e.span),
        Content::ShowRecipe(_) => {
            return super::recipes::realize(node, ctx, engine).map(Some);
        }
        Content::Styled(styled) => {
            let body = &styled.body;
            let styles = &styled.styles;
            let old = engine.styles.clone();
            *engine.styles = TableCellProperties::chain_styles(engine.styles, styles);
            let captured = captured_recipe(engine.styles);
            let old_rules = captured.as_ref().map(|r| {
                let rules = captured_rules(r, engine);
                std::mem::replace(engine.show_rules, rules)
            });
            let old_guards = captured.as_ref().map(|r| {
                let guards = insertion_guards(&r.guards, engine.active_guards);
                std::mem::replace(engine.active_guards, guards)
            });
            let old_file = engine.current_file;
            if let Some(r) = captured {
                engine.current_file = r.file;
                ctx.next_context_id = ctx.next_context_id.max(r.context_id_floor);
            }
            let result = walk(body, ctx, engine);
            *engine.styles = old;
            if let Some(r) = old_rules {
                *engine.show_rules = r;
            }
            if let Some(g) = old_guards {
                *engine.active_guards = g;
            }
            engine.current_file = old_file;
            Content::styled(result?, styles.clone())
        }
        Content::Table(e) if e.resolved.is_none() => table(node, ctx, engine)?,
        Content::Located(e) if matches!(&e.body,Content::Table(t) if t.resolved.is_none()) => {
            table(node, ctx, engine)?
        }
        Content::Table(e) if e.resolved.is_some() => node.clone(),
        Content::TableCell(e) => realize_cell(e, ctx, engine)?,
        Content::Sequence(items) => Content::Sequence(
            items
                .iter()
                .map(|c| walk(c, ctx, engine))
                .collect::<SourceResult<Vec<_>>>()?
                .into(),
        ),
        Content::Block(e) => {
            let mut b = e.as_ref().clone();
            b.body = walk(&b.body, ctx, engine)?;
            Content::Block(Arc::new(b))
        }
        Content::Boxed(e) => {
            let mut b = e.as_ref().clone();
            b.body = walk(&b.body, ctx, engine)?;
            Content::Boxed(Arc::new(b))
        }
        Content::Realized(_)
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
        | Content::SetPage(_)
        | Content::PageRun(_)
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
        | Content::Dynamic(_) => return Ok(None),
    };
    Ok(Some(result))
}

fn default_args(
    styles: &crate::entities::style_chain::StyleChain,
) -> crate::entities::args::Args {
    let mut args =
        crate::entities::args::Args::positional(vec![Value::Content(Content::Empty)]);
    for (field, key) in [
        ("x", "cell.x"),
        ("y", "cell.y"),
        ("colspan", "cell.colspan"),
        ("rowspan", "cell.rowspan"),
        ("fill", "cell.fill"),
        ("stroke", "cell.stroke"),
        ("align", "cell.align"),
        ("breakable", "cell.breakable"),
    ] {
        if let Some(value) = styles.custom(key) {
            args.named.insert(field.into(), value.clone());
        }
    }
    args
}

fn with_defaults(
    e: &TableCellElem,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    padding: bool,
) -> SourceResult<TableCellElem> {
    if e.stage().is_some() {
        return Ok(e.clone());
    }
    let args = default_args(engine.styles);
    let Value::Content(Content::TableCell(defaults)) =
        crate::compiler::stdlib::native_table_cell(
            ctx,
            &args,
            engine.world,
            engine.current_file,
        )?
    else {
        unreachable!()
    };
    let mut p = e.properties();
    let d = defaults.properties();
    if p.presence.stroke && p.stroke.is_some() {
        p.stroke_inherited = d.stroke.clone();
    }
    if p.presence.x == CellFieldPresence::Omitted {
        p.x = d.x;
        p.presence.x = d.presence.x;
    }
    if p.presence.y == CellFieldPresence::Omitted {
        p.y = d.y;
        p.presence.y = d.presence.y;
    }
    if !p.presence.colspan {
        p.colspan = d.colspan;
        p.presence.colspan = d.presence.colspan;
    }
    if !p.presence.rowspan {
        p.rowspan = d.rowspan;
        p.presence.rowspan = d.presence.rowspan;
    }
    if p.presence.fill == CellFieldPresence::Omitted {
        p.fill = d.fill;
        p.presence.fill = d.presence.fill;
    }
    if !p.presence.stroke {
        p.stroke = d.stroke;
        p.presence.stroke = d.presence.stroke;
    }
    if p.presence.align == CellFieldPresence::Omitted {
        p.align = d.align;
        p.presence.align = d.presence.align;
    }
    if p.presence.breakable == CellFieldPresence::Omitted {
        p.breakable = d.breakable;
        p.presence.breakable = d.presence.breakable;
    }
    if p.presence.inset == CellFieldPresence::Omitted {
        if let Some(value) = engine.styles.custom("cell.inset") {
            p.presence.inset = if matches!(value, Value::Auto) {
                CellFieldPresence::Auto
            } else {
                CellFieldPresence::Explicit
            };
            p.inset = TableCellProperties::style_inset(value);
        }
    }
    if padding {
        p.colspan = Some(1);
        p.rowspan = Some(1);
        p.presence.colspan = true;
        p.presence.rowspan = true;
    }
    let mut c = e.clone();
    c.standalone = Some(Arc::new(TableCellStandaloneResolved {
        visual: c.body.clone(),
        stage: TableCellStage::Prepared,
        guards: vec![],
        properties: Some(p),
    }));
    Ok(c)
}

fn argument_defaults(
    body: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    match body.source_body() {
        Content::TableHeader(e) => {
            let mut h = e.as_ref().clone();
            h.body = group_defaults(&h.body, ctx, engine)?;
            Ok(Content::TableHeader(Arc::new(h)))
        }
        Content::TableFooter(e) => {
            let mut h = e.as_ref().clone();
            h.body = group_defaults(&h.body, ctx, engine)?;
            Ok(Content::TableFooter(Arc::new(h)))
        }
        Content::TableCell(e) => {
            let mut cell = with_defaults(e, ctx, engine, false)?;
            let origin = body.source_span();
            if cell.source_span.is_detached() && !origin.is_detached() {
                cell.source_span = origin;
            }
            Ok(Content::TableCell(Arc::new(cell)))
        }
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
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
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
        | Content::Dynamic(_) => {
            let Content::TableCell(e) =
                Content::table_cell(body.clone(), None, None, None, None)
            else {
                unreachable!()
            };
            let mut cell = with_defaults(&e, ctx, engine, false)?;
            cell.source_span = body.source_span();
            Ok(Content::TableCell(Arc::new(cell)))
        }
    }
}

fn group_defaults(
    body: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let product = if let Content::Sequence(items) = body.source_body() {
        Content::Sequence(
            items
                .iter()
                .map(|c| argument_defaults(c, ctx, engine))
                .collect::<SourceResult<Vec<_>>>()?
                .into(),
        )
    } else {
        argument_defaults(body, ctx, engine)?
    };
    Ok(product.with_source_span(body.source_span()))
}

fn retain_or_recompute_defaults<T, E>(
    first: T,
    self_style_matched: bool,
    recompute: impl FnOnce() -> Result<T, E>,
) -> Result<T, E> {
    if self_style_matched {
        recompute()
    } else {
        Ok(first)
    }
}

fn cell_style_candidate<'a>(
    cell: &TableCellElem,
    candidate: &'a std::cell::OnceCell<Content>,
    eligible: bool,
) -> Option<&'a Content> {
    eligible.then(|| candidate.get_or_init(|| Content::TableCell(Arc::new(cell.clone()))))
}

fn realize_cell(
    e: &TableCellElem,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if e.stage() == Some(TableCellStage::Complete) {
        return Ok(Content::TableCell(Arc::new(e.clone())));
    }
    let mut c = e.clone();
    if c.stage().is_none() {
        c = with_defaults(&c, ctx, engine, false)?;
        // Cell self show-set is applied before standalone fields are observed.
        let candidate = std::cell::OnceCell::new();
        let old = engine.styles.clone();
        let mut self_style_matched = false;
        for rule in engine.show_rules.iter() {
            if let crate::entities::show::Transformation::Style(styles) = &rule.transform
            {
                if cell_style_candidate(
                    &c,
                    &candidate,
                    !engine.active_guards.contains(&rule.id),
                )
                .is_some_and(|candidate| {
                    crate::compiler::eval::selector_matching::selector_matches(
                        candidate,
                        &rule.selector,
                    )
                }) {
                    self_style_matched = true;
                    *engine.styles =
                        TableCellProperties::chain_styles(engine.styles, styles);
                }
            }
        }
        let result = retain_or_recompute_defaults(c, self_style_matched, || {
            with_defaults(e, ctx, engine, false)
        });
        *engine.styles = old;
        c = result?;
    }
    if c.resolved.is_some() && c.stage() == Some(TableCellStage::Prepared) {
        let candidate = std::cell::OnceCell::new();
        let mut styles = engine.styles.clone();
        for rule in engine.show_rules.iter() {
            if let crate::entities::show::Transformation::Style(delta) = &rule.transform {
                if cell_style_candidate(
                    &c,
                    &candidate,
                    !engine.active_guards.contains(&rule.id),
                )
                .is_some_and(|candidate| {
                    crate::compiler::eval::selector_matching::selector_matches(
                        candidate,
                        &rule.selector,
                    )
                }) {
                    styles = TableCellProperties::chain_styles(&styles, delta);
                }
            }
        }
        let mut p = c.properties();
        if !c.presence.colspan {
            if let Some(Value::Int(n)) = styles.custom("cell.colspan") {
                p.colspan = Some(*n as usize);
                p.presence.colspan = true;
            }
        }
        if !c.presence.rowspan {
            if let Some(Value::Int(n)) = styles.custom("cell.rowspan") {
                p.rowspan = Some(*n as usize);
                p.presence.rowspan = true;
            }
        }
        c.standalone = Some(Arc::new(TableCellStandaloneResolved {
            visual: c.body.clone(),
            stage: TableCellStage::Prepared,
            guards: c.guards().to_vec(),
            properties: Some(p),
        }));
    }
    let rules = engine.show_rules.clone();
    let (product, applied) = crate::compiler::eval::rules::apply::apply_cell_rules_root(
        Content::TableCell(Arc::new(c)),
        &rules,
        ctx,
        engine,
    )
    .map_err(|mut errors| {
        for error in &mut errors {
            if error.span.is_detached() {
                error.span = e.source_span;
            }
        }
        errors
    })?;
    if !applied {
        return finish_cell_product(&product, ctx, engine);
    }
    // The route is local to this accepted recipe product, and is restored on error.
    let route = Route::extend(engine.route);
    let mut sink = TrackedMut::reborrow_mut(&mut *engine.sink);
    let mut child = Engine {
        world: engine.world,
        font_metrics: engine.font_metrics,
        route: route.track(),
        styles: &mut *engine.styles,
        show_rules: &mut *engine.show_rules,
        active_guards: &mut *engine.active_guards,
        current_file: engine.current_file,
        sink: &mut sink,
    };
    crate::entities::world_types::check_show_depth(child.route).map_err(
        |mut errors| {
            for error in &mut errors {
                if error.span.is_detached() {
                    error.span = e.source_span;
                }
            }
            errors
        },
    )?;
    walk(&product, ctx, &mut child)
}

fn finish_cell_product(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    match content {
        Content::Styled(s) => {
            let old = engine.styles.clone();
            *engine.styles = TableCellProperties::chain_styles(engine.styles, &s.styles);
            let result = finish_cell_product(&s.body, ctx, engine);
            *engine.styles = old;
            Ok(Content::styled(result?, s.styles.clone()))
        }
        Content::TableCell(e) => {
            let mut c = e.as_ref().clone();
            let body = walk(&c.body, ctx, engine)?;
            if let Some(r) = &c.resolved {
                let mut r = r.as_ref().clone();
                r.visual = body;
                r.stage = TableCellStage::Complete;
                c.resolved = Some(Arc::new(r));
            } else if let Some(r) = &c.standalone {
                let mut r = r.as_ref().clone();
                r.visual = body;
                r.stage = TableCellStage::Complete;
                c.standalone = Some(Arc::new(r));
            }
            Ok(Content::TableCell(Arc::new(c)))
        }
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
        | Content::SetPage(_)
        | Content::PageRun(_)
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
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => walk(content, ctx, engine),
    }
}

fn table(
    node: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let protected = matches!(node.source_body(), Content::Located(located)
        if ctx.recipe_protected_locations.contains(&located.location));
    let rules: Vec<_> = engine
        .show_rules
        .iter()
        .filter(|rule| !protected || !engine.active_guards.contains(&rule.id))
        .cloned()
        .collect();
    let mut applied: Vec<_> = rules
        .iter()
        .filter(|rule| {
            matches!(rule.transform, crate::entities::show::Transformation::Style(_))
                && crate::compiler::eval::selector_matching::selector_matches(
                    node,
                    &rule.selector,
                )
        })
        .map(|rule| rule.id)
        .collect();
    if let Some(rule) = rules.iter().rev().find(|rule| {
        !matches!(rule.transform, crate::entities::show::Transformation::Style(_))
            && crate::compiler::eval::selector_matching::selector_matches(
                node,
                &rule.selector,
            )
    }) {
        applied.push(rule.id);
    }
    let product = crate::compiler::eval::rules::apply::apply_show_rules_root(
        node.clone(),
        &rules,
        ctx,
        engine,
    )?;
    let guard_depth = engine.active_guards.len();
    engine.active_guards.extend(applied);
    let depth = ctx.recipe_protected_locations.len();
    if let Content::Realized(done) = product.source_body() {
        if let Content::Located(located) = done.original.source_body() {
            ctx.recipe_protected_locations.push(located.location);
        }
    }
    let result = product.map_content_pre(&mut |child| {
        match child {
            Content::Table(e) if e.resolved.is_none()=>Ok(Some(prepare_table(e,ctx,engine)?)),
            Content::Located(e) if matches!(&e.body,Content::Table(t) if t.resolved.is_none())=>{
                let Content::Table(t)=&e.body else{unreachable!()};
                Ok(Some(Content::Located(Arc::new(LocatedContent {location:e.location,body:prepare_table(t,ctx,engine)?}))))
            },
            Content::ShowRecipe(_) | Content::Styled(_) | Content::TableCell(_)=>Ok(Some(walk(child,ctx,engine)?)),
            Content::Source(_) | Content::Realized(_) | Content::Located(_) | Content::Empty | Content::Text(_) | Content::Space | Content::SpaceFromNewline | Content::Parbreak | Content::Sequence(_) | Content::HtmlElem(_) | Content::HtmlFrame(_) | Content::Par { .. } | Content::Heading(_) | Content::Title(_) | Content::Strong(_) | Content::Emph(_) | Content::Raw(_) | Content::ListItem(_) | Content::EnumItem(_) | Content::Link(_) | Content::Equation(_) | Content::MathSequence(_) | Content::MathIdent(_) | Content::MathText(_) | Content::MathFrac(_) | Content::MathAttach(_) | Content::MathPrimes(_) | Content::MathRoot(_) | Content::MathStretch(_) | Content::MathDelimited(_) | Content::MathLr(_) | Content::MathMid(_) | Content::MathOverbrace(_) | Content::MathUnderbrace(_) | Content::MathUnderbracket(_) | Content::MathUnderparen(_) | Content::MathUndershell(_) | Content::MathOverbracket(_) | Content::MathOverparen(_) | Content::MathOvershell(_) | Content::MathAlignPoint(_) | Content::Linebreak(_) | Content::MathMatrix(_) | Content::MathCases(_) | Content::MathAccent(_) | Content::MathCancel(_) | Content::MathUnderline(_) | Content::MathOverline(_) | Content::MathVec(_) | Content::MathClassOverride(_) | Content::MathLimitsOverride(_) | Content::MathUnderover(_) | Content::MathOp(_) | Content::MathStyled(_) | Content::Label(_) | Content::Ref(_) | Content::CounterDisplay(_) | Content::CounterUpdate(_) | Content::Outline(_) | Content::OutlineEntry(_) | Content::Figure(_) | Content::FigureCaption(_) | Content::Image(_) | Content::Shape(_) | Content::Curve(_) | Content::Transform(_) | Content::Grid(_) | Content::GridHeader(_) | Content::GridFooter(_) | Content::GridCell(_) | Content::SetPage(_) | Content::PageRun(_) | Content::Align(_) | Content::Place(_) | Content::Flush(_) | Content::Divider(_) | Content::Terms(_) | Content::TermItem(_) | Content::Quote(_) | Content::Document { .. } | Content::Asset { .. } | Content::SmartQuote(_) | Content::PdfAttach(_) | Content::PdfArtifact(_) | Content::Underline(_) | Content::Strike(_) | Content::Overline(_) | Content::SmallCaps { .. } | Content::Pad(_) | Content::Hide(_) | Content::HSpace(_) | Content::VSpace(_) | Content::Pagebreak(_) | Content::Colbreak(_) | Content::Stack(_) | Content::Boxed(_) | Content::Block(_) | Content::Bibliography(_) | Content::Cite(_) | Content::Footnote(_) | Content::FootnoteEntry(_) | Content::TableHeader(_) | Content::TableFooter(_) | Content::GridHLine(_) | Content::GridVLine(_) | Content::TableHLine(_) | Content::TableVLine(_) | Content::Table(_) | Content::Repeat(_) | Content::Columns(_) | Content::Metadata(_) | Content::State(_) | Content::StateUpdate(_) | Content::StateDisplay(_) | Content::CounterDisplayCallback(_) | Content::ContextBlock(_) | Content::Dynamic(_)=>Ok(None),
        }
    });
    ctx.recipe_protected_locations.truncate(depth);
    engine.active_guards.truncate(guard_depth);
    result
}

fn fold_align(cell: Option<Align2D>, table: Option<Align2D>) -> Option<Align2D> {
    match (cell, table) {
        (Some(c), Some(t)) => Some(Align2D { h: c.h.or(t.h), v: c.v.or(t.v) }),
        (c, t) => c.or(t),
    }
}

fn prepare_table(
    e: &TableElem,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let cols = e.columns.len().max(1);
    let mut logical = e.clone();
    logical.children = e
        .children
        .iter()
        .map(|c| argument_defaults(c, ctx, engine))
        .collect::<SourceResult<Vec<_>>>()?;
    logical.header = e
        .header
        .as_ref()
        .map(|c| argument_defaults(c, ctx, engine))
        .transpose()?;
    logical.footer = e
        .footer
        .as_ref()
        .map(|c| argument_defaults(c, ctx, engine))
        .transpose()?;
    let plan = (|| {
        let (cells, header_rows, footer_cells) =
            crate::compiler::layout::grid_placement::table_groups(&logical, cols)?;
        let min_rows = if e.rows_explicit { e.rows.len() } else { 0 };
        let placed = crate::compiler::layout::grid_placement::place_table_cells(
            &cells, cols, min_rows,
        )?;
        Ok::<_, Vec<crate::entities::source_result::SourceDiagnostic>>((
            placed,
            header_rows,
            footer_cells,
        ))
    })();
    let (mut placed, header_rows, footer_cells) = match plan {
        Ok(plan) => plan,
        Err(errors) => {
            return crate::compiler::eval::call_dispatch::trace_plugin::trace_table_realization(Err(errors), e.source_span, engine);
        }
    };
    let num_rows = placed.iter().map(|p| p.row + p.rowspan).max().unwrap_or(0);
    let footer_start = num_rows.saturating_sub(footer_cells.div_ceil(cols));
    let mut output = Vec::with_capacity(placed.len());
    for p in placed.drain(..) {
        let mut cell = match &p.body {
            Content::TableCell(c) => c.as_ref().clone(),
            body => {
                let Content::TableCell(c) =
                    Content::table_cell(body.clone(), None, None, None, None)
                else {
                    unreachable!()
                };
                let mut c = c.as_ref().clone();
                if matches!(body, Content::Empty) {
                    c.colspan = Some(1);
                    c.rowspan = Some(1);
                    c.presence.colspan = true;
                    c.presence.rowspan = true;
                }
                c
            }
        };
        cell = with_defaults(&cell, ctx, engine, matches!(p.body, Content::Empty))?;
        let properties = cell.properties();
        let fill = if properties.presence.fill == CellFieldPresence::Explicit {
            properties.fill.clone()
        } else {
            e.fill.clone()
        };
        let stroke = if properties.presence.stroke {
            properties.stroke.as_ref().map(|s| {
                TableCellProperties::fold_stroke(
                    &TableCellProperties::fold_stroke(
                        s,
                        properties.stroke_inherited.as_ref(),
                    ),
                    e.stroke.as_ref(),
                )
            })
        } else {
            e.stroke.clone()
        };
        let breakable = properties.breakable.unwrap_or_else(|| {
            (p.row..p.row + p.rowspan)
                .any(|row| matches!(e.rows.get(row), None | Some(TrackSizing::Auto)))
        });
        let guards = cell.guards().to_vec();
        cell.standalone = None;
        cell.resolved = Some(Arc::new(TableCellResolved {
            x: p.col,
            y: p.row,
            colspan: p.colspan,
            rowspan: p.rowspan,
            fill,
            stroke,
            align: fold_align(properties.align, e.align),
            inset: properties
                .inset
                .map(|s| {
                    crate::entities::sides::Sides::new(
                        s.left.unwrap_or(e.inset.left),
                        s.top.unwrap_or(e.inset.top),
                        s.right.unwrap_or(e.inset.right),
                        s.bottom.unwrap_or(e.inset.bottom),
                    )
                })
                .unwrap_or(e.inset),
            breakable,
            visual: cell.body.clone(),
            stage: TableCellStage::Prepared,
            guards,
        }));
        let result = realize_cell(&cell, ctx, engine)?;
        let visual = result;
        let mut r = cell.resolved.as_ref().unwrap().as_ref().clone();
        r.visual = visual;
        r.stage = TableCellStage::Complete;
        cell.resolved = Some(Arc::new(r));
        output.push(Content::TableCell(Arc::new(cell)));
    }
    let mut t = e.clone();
    t.resolved = Some(Arc::new(TableGridResolved {
        cells: output,
        num_cols: cols,
        num_rows,
        header_rows,
        footer_start,
        complete: true,
    }));
    t.caption = e.caption.as_ref().map(|c| walk(c, ctx, engine)).transpose()?;
    Ok(Content::Table(Arc::new(t)))
}

#[cfg(test)]
mod default_reuse_tests {
    use super::{cell_style_candidate, default_args, retain_or_recompute_defaults};

    #[test]
    fn style_candidate_is_lazy_and_reused_for_eligible_rules() {
        let crate::entities::content::Content::TableCell(cell) =
            crate::entities::content::Content::table_cell(
                crate::entities::content::Content::Empty,
                None,
                None,
                None,
                None,
            )
        else {
            unreachable!()
        };
        let candidate = std::cell::OnceCell::new();
        assert!(cell_style_candidate(&cell, &candidate, false).is_none());
        assert!(candidate.get().is_none(), "ineligible rule must not clone the cell");
        let first = cell_style_candidate(&cell, &candidate, true).unwrap();
        let second = cell_style_candidate(&cell, &candidate, true).unwrap();
        assert!(std::ptr::eq(first, second));
        assert!(cell_style_candidate(&cell, &candidate, false).is_none());
    }

    #[test]
    fn default_args_project_exact_eight_fields_in_declared_order() {
        let mut styles = crate::entities::style_chain::StyleChain::default_chain();
        let fields =
            ["x", "y", "colspan", "rowspan", "fill", "stroke", "align", "breakable"];
        for (n, field) in fields.iter().enumerate() {
            styles = styles.push_custom(
                format!("cell.{field}"),
                crate::entities::value::Value::Int(n as i64),
            );
        }
        styles = styles.push_custom("cell.inset", crate::entities::value::Value::Int(99));
        let args = default_args(&styles);
        assert_eq!(args.named.keys().map(|key| key.as_str()).collect::<Vec<_>>(), fields);
        for (n, field) in fields.iter().enumerate() {
            assert_eq!(
                args.named.get(*field),
                Some(&crate::entities::value::Value::Int(n as i64))
            );
        }
        assert!(args.named.get("inset").is_none());
    }

    #[test]
    fn unchanged_self_style_keeps_first_product_without_recomputing() {
        let mut calls = 0;
        let result: Result<i32, &'static str> =
            retain_or_recompute_defaults(7, false, || {
                calls += 1;
                Ok(9)
            });
        assert_eq!(result.unwrap(), 7);
        assert_eq!(calls, 0);
    }

    #[test]
    fn matched_self_style_recomputes_and_preserves_error() {
        let mut calls = 0;
        let result: Result<i32, &'static str> =
            retain_or_recompute_defaults(7, true, || {
                calls += 1;
                Err("invalid cell style")
            });
        assert_eq!(result.unwrap_err(), "invalid cell style");
        assert_eq!(calls, 1);
    }
}

#[cfg(test)]
mod v16_table_tests {
    use crate::entities::show::Selector;

    use crate::contracts::world::World;
    use crate::entities::{
        file_id::FileId,
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                ),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    use super::*;
    use crate::compiler::{layout::FixedMetrics, realize::realize_table_insertions};
    use crate::entities::{
        content::ShowRecipeContent,
        show::{ShowRule, Transformation},
        span::Span,
        style::Styles,
        style_chain::StyleChain,
        world_types::{Route, Sink},
    };
    use comemo::Track;
    fn recipe(on: bool, floor: u64) -> Value {
        let c = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: Content::Empty,
            textual: false,
            context_id_floor: floor,
            rules: Arc::from([]),
            styles: StyleChain::empty(),
            guards: vec![],
            file: FileId::from_raw(std::num::NonZeroU16::new(2).unwrap()),
        }));
        Value::Content(if on {
            c.with_source_span(Span::from_range(TestWorld::new("").main(), 1..2))
        } else {
            c
        })
    }
    fn capture(mode: &str, local: Option<Value>, bad_defaults: bool) -> (u64, bool) {
        let world = TestWorld::new("");
        let mut styles = StyleChain::default_chain()
            .push_custom("eval.context.recipe", recipe(false, 31));
        let mut content = Content::table_cell(Content::text("x"), None, None, None, None);
        if mode == "styled" {
            let mut delta = Styles::default();
            if let Some(v) = local {
                delta = delta.push_custom("eval.table.local.recipe", v);
            }
            content = Content::styled(Box::new(content), delta);
        } else if let Some(v) = local {
            styles = styles.push_custom("eval.table.local.recipe", v);
        }
        if bad_defaults {
            styles = styles.push_custom("cell.colspan", Value::Str("bad".into()));
        }
        let route = Route::root().with_id(world.main());
        let metrics = FixedMetrics;
        let mut sink = Sink::new();
        let mut rules: Arc<[ShowRule]> = Arc::from([ShowRule {
            id: 71,
            selector: Selector::Text("not-x".into()),
            transform: Transformation::Str("unused".into()),
        }]);
        let mut guards = vec![99];
        let mut ctx = EvalContext::new();
        let error;
        {
            let mut tracked = sink.track_mut();
            let mut engine = Engine {
                world: &world,
                font_metrics: &metrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut rules,
                active_guards: &mut guards,
                current_file: world.main(),
                sink: &mut tracked,
            };
            error = realize_table_insertions(content, &mut ctx, &mut engine).is_err();
            assert_eq!(engine.current_file, world.main());
            assert_eq!(engine.active_guards.as_slice(), &[99]);
            assert_eq!(engine.show_rules.len(), 1);
            assert_eq!(engine.show_rules[0].id, 71);
        }
        (ctx.next_context_id, error)
    }
    #[test]
    fn v16_table_snapshot_source_at_root() {
        assert_eq!(capture("root", Some(recipe(true, 77)), false), (77, false));
    }
    #[test]
    fn v16_table_snapshot_source_in_styled() {
        assert_eq!(capture("styled", Some(recipe(true, 77)), false), (77, false));
    }
    #[test]
    fn v16_table_snapshot_error_restores_context() {
        assert_eq!(capture("root", Some(recipe(true, 77)), true), (77, true));
    }
    #[test]
    fn v16_table_snapshot_local_precedes_general() {
        assert_eq!(capture("root", Some(recipe(true, 77)), false).0, 77);
        assert_eq!(capture("root", None, false).0, 31);
    }
    #[test]
    fn v16_table_snapshot_invalid_local_value_shadows_general() {
        assert_eq!(capture("root", Some(Value::Int(0)), false).0, 0);
    }
    #[test]
    fn v16_table_snapshot_nonrecipe_content_shadows_general() {
        assert_eq!(
            capture("root", Some(Value::Content(Content::text("not-a-recipe"))), false).0,
            0
        );
    }
}
