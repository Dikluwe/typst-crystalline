//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/query.md
//! @prompt-hash 733b0edf
//! @layer L1
//! @updated 2026-08-13
//!
//! Query, localização, metadados e selectors: metadata, query, locate, here,
//! target, selector. Fatiado de `foundations.rs` no Passo 1032.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::introspector::Introspector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

use super::selector::parse_selector_arg;
use crate::compiler::stdlib::{err, expect_no_named};

/// `metadata(value)` — embeber valor opaco no documento para query
/// via `Introspector::query_metadata`. P169 (M9 sub-passo 1).
///
/// Vanilla: `metadata(value)` em `introspection/metadata.rs`. Cristalino
/// minimal: 1 argumento posicional; sem named args; produz
/// `Content::metadata(Box<Value>)` que é zero-size em layout.
pub fn native_metadata(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Content(crate::entities::content::Content::metadata(v.clone()))),
        _ => err(format!("metadata() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `query(kind_str)` — consulta o `Introspector` da iteração de fixpoint
/// anterior por elementos do kind indicado.
///
/// **P175 (M9 sub-passo 5)**: forma original retornava `Value::Int(count)`.
/// **P179 upgrade**: retorna `Value::Array(Vec<Value::Location>)` com
/// as Locations dos elementos matched, em ordem de aparecimento.
pub fn native_query(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let selector = parse_selector_arg(&args.items, "query")?;
    let locations = ctx.introspector.query(&selector);
    // **P844** — devolve o `Content` do elemento encontrado (paridade
    // vanilla `query() -> array<content>`). Fallback para `Value::Location`
    // quando o introspector não tem o elemento registado.
    let values: Vec<Value> = locations
        .into_iter()
        .map(|loc| match ctx.introspector.elements.get(&loc) {
            Some(c) => Value::LocatedContent(c.clone(), loc),
            None => Value::Location(loc),
        })
        .collect();
    Ok(Value::Array(values))
}

/// **P208C (M9c)** — `locate(kind)` — retorna a **primeira** Location
/// de um elemento do `kind` indicado.
pub fn native_locate(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let selector = parse_selector_arg(&args.items, "locate")?;
    let first = ctx.introspector.query(&selector).first().copied();
    Ok(match first {
        Some(loc) => Value::Location(loc),
        None => Value::None,
    })
}

/// **P208B (M9c)** — `here()` — retorna a Location "actual" disponível
/// no `EvalContext`.
pub fn native_here(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if !args.items.is_empty() {
        return err(format!(
            "here() não aceita argumentos, recebeu {}",
            args.items.len()
        ));
    }
    match ctx.current_location {
        Some(loc) => Ok(Value::Location(loc)),
        None => err("here() chamado fora de contexto locatable — \
             current_location não populado (P208B: infra minimal; \
             captura automática no walk é deferred)"
            .to_string()),
    }
}

/// **P772w** — `target()` — devolve o alvo de exportação actual.
///
/// Paridade vanilla: devolve `"paged"`, `"html"` ou `"bundle"`. Cristalino
/// só produz PDF via layout paginado — devolve sempre `"paged"`.
///
/// **P821** — gate de contexto: fora de `#context` falha com
/// `can only be used when context is known` + hints.
pub fn native_target(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some((name, _)) = args.named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]);
    }
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "can only be used when context is known",
        )
        .with_hint("try wrapping this in a `context` expression")
        .with_hint(
            "the `context` expression should wrap everything that depends on this function",
        )]);
    }
    Ok(Value::Str(match ctx.target {
        crate::compiler::eval::EvalTarget::Paged => "paged".into(),
        crate::compiler::eval::EvalTarget::Html => "html".into(),
    }))
}
