//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Armos de markup (`Strong`, `Emph`, `Heading`, `Raw`, `Link`, `ListItem`,
//! `EnumItem`) extraídos do dispatcher `eval_expr` no Passo 96.2 conforme
//! ADR-0037 Regra 4 (dispatcher com armos de uma linha).

use std::sync::Arc;

use comemo::{Tracked, TrackedMut};
use ecow::EcoString;

use crate::entities::ast::AstNode;
use crate::entities::file_id::FileId;
use crate::entities::ast::markup;
use crate::entities::content::Content;
use crate::entities::layout_types::TextStyle;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source_result::SourceResult;
use crate::entities::style_chain::{StyleChain, StyleDelta};
use crate::entities::value::Value;
use crate::entities::world_types::{Route, Sink};
use crate::rules::scopes::Scopes;

use super::{eval_markup_body, rules, EvalContext};

pub(super) fn eval_strong<'r>(
    strong: markup::Strong<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    // Capturar bold no estilo activo para que os Text filhos carreguem bold=true.
    // Atomização Passo 94: styles local ao corpo, não muta o caller.
    let mut local_styles = styles.push(StyleDelta { bold: Some(true), italic: None, size: None , ..StyleDelta::empty() });
    let body = eval_markup_body(strong.body().to_untyped(), scopes, ctx, route, &mut local_styles, show_rules, active_guards, current_file, figure_numbering, sink)?;
    let content = Content::strong(body);
    Ok(Value::Content(rules::intercept_content(content, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?))
}

pub(super) fn eval_emph<'r>(
    emph: markup::Emph<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    // Capturar italic no estilo activo para que os Text filhos carreguem italic=true.
    let mut local_styles = styles.push(StyleDelta { bold: None, italic: Some(true), size: None , ..StyleDelta::empty() });
    let body = eval_markup_body(emph.body().to_untyped(), scopes, ctx, route, &mut local_styles, show_rules, active_guards, current_file, figure_numbering, sink)?;
    let content = Content::emph(body);
    Ok(Value::Content(rules::intercept_content(content, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?))
}

pub(super) fn eval_heading<'r>(
    heading: markup::Heading<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    let level = heading.depth().get() as u8;
    // Capturar bold no estilo para que os Text filhos do heading carreguem bold=true.
    let mut local_styles = styles.push(StyleDelta { bold: Some(true), italic: None, size: None , ..StyleDelta::empty() });
    let body  = eval_markup_body(heading.body().to_untyped(), scopes, ctx, route, &mut local_styles, show_rules, active_guards, current_file, figure_numbering, sink)?;
    // Intercepção eager — show rules aplicadas imediatamente após criação (Passo 68).
    let content = Content::heading(level, body);
    Ok(Value::Content(rules::intercept_content(content, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?))
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

pub(super) fn eval_list_item<'r>(
    item: markup::ListItem<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    let body = eval_markup_body(item.body().to_untyped(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?;
    Ok(Value::Content(Content::list_item(body)))
}

pub(super) fn eval_enum_item<'r>(
    item: markup::EnumItem<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    let number = item.number().map(|n| n as u32);
    let body   = eval_markup_body(item.body().to_untyped(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?;
    Ok(Value::Content(Content::enum_item(number, body)))
}
