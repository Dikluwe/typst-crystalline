//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-23
//!
//! Armos de markup (`Strong`, `Emph`, `Heading`, `Raw`, `Link`, `ListItem`,
//! `EnumItem`) extraídos do dispatcher `eval_expr` no Passo 96.2 conforme
//! ADR-0037 Regra 4. Assinaturas simplificadas no Passo 109 (ADR-0044)
//! via `Engine<'_>`.

use comemo::TrackedMut;
use ecow::EcoString;

use crate::entities::ast::AstNode;
use crate::entities::ast::markup;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::layout_types::TextStyle;
use crate::entities::source_result::SourceResult;
use crate::entities::style_chain::{StyleChain, StyleDelta};
use crate::entities::value::Value;
use crate::rules::scopes::Scopes;

use super::{eval_markup_body, rules, EvalContext};

/// Helper para chamar `eval_markup_body` com um `local_styles` modificado
/// por um `StyleDelta` (Passo 109, ADR-0044). Reconstrói Engine localmente
/// para o corpo, retorna o Content avaliado.
fn eval_body_with_delta(
    node: &crate::entities::syntax_node::SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    delta: StyleDelta,
) -> SourceResult<Content> {
    let mut local_styles = engine.styles.push(delta);
    let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
    let mut local_engine = Engine {
        world: engine.world,
        route: engine.route,
        styles: &mut local_styles,
        show_rules: &mut *engine.show_rules,
        active_guards: &mut *engine.active_guards,
        current_file: engine.current_file,
        figure_numbering: &mut *engine.figure_numbering,
        sink: &mut local_sink,
    };
    eval_markup_body(node, scopes, ctx, &mut local_engine)
}

pub(super) fn eval_strong(
    strong: markup::Strong<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Capturar bold no estilo activo para que os Text filhos carreguem bold=true.
    let delta = StyleDelta { bold: Some(true), italic: None, size: None, ..StyleDelta::empty() };
    let body = eval_body_with_delta(strong.body().to_untyped(), scopes, ctx, engine, delta)?;
    let content = Content::strong(body);
    Ok(Value::Content(rules::intercept_content(content, ctx, engine)?))
}

pub(super) fn eval_emph(
    emph: markup::Emph<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Capturar italic no estilo activo para que os Text filhos carreguem italic=true.
    let delta = StyleDelta { bold: None, italic: Some(true), size: None, ..StyleDelta::empty() };
    let body = eval_body_with_delta(emph.body().to_untyped(), scopes, ctx, engine, delta)?;
    let content = Content::emph(body);
    Ok(Value::Content(rules::intercept_content(content, ctx, engine)?))
}

pub(super) fn eval_heading(
    heading: markup::Heading<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let level = heading.depth().get() as u8;
    // Capturar bold no estilo para que os Text filhos do heading carreguem bold=true.
    let delta = StyleDelta { bold: Some(true), italic: None, size: None, ..StyleDelta::empty() };
    let body = eval_body_with_delta(heading.body().to_untyped(), scopes, ctx, engine, delta)?;
    // Intercepção eager — show rules aplicadas imediatamente após criação (Passo 68).
    let content = Content::heading(level, body);
    Ok(Value::Content(rules::intercept_content(content, ctx, engine)?))
}

pub(super) fn eval_raw(raw: markup::Raw<'_>) -> SourceResult<Value> {
    // Raw não tem método text() — raw.lines() itera nós Text (SyntaxKind::Text)
    // tanto para inline como para block. RawTrimmed são apenas whitespace/newlines.
    let text: EcoString = raw.lines()
        .map(|l| l.get())
        .collect::<Vec<_>>()
        .join("\n")
        .into();
    let lang  = raw.lang().map(|l| EcoString::from(l.get()));
    let block = raw.block();
    Ok(Value::Content(Content::raw(text, lang, block)))
}

pub(super) fn eval_link(link: markup::Link<'_>, styles: &StyleChain) -> SourceResult<Value> {
    let url = link.get().to_string();
    let style = TextStyle::from(styles);
    Ok(Value::Content(Content::link(url.clone(), Content::Text(url.into(), style))))
}

pub(super) fn eval_list_item(
    item: markup::ListItem<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let body = eval_markup_body(item.body().to_untyped(), scopes, ctx, engine)?;
    Ok(Value::Content(Content::list_item(body)))
}

pub(super) fn eval_enum_item(
    item: markup::EnumItem<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let number = item.number().map(|n| n as u32);
    let body = eval_markup_body(item.body().to_untyped(), scopes, ctx, engine)?;
    Ok(Value::Content(Content::enum_item(number, body)))
}
