//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/query.md
//! @prompt-hash 0bf9739d
//! @layer L1
//! @updated 2026-08-13
//!
//! Query, localização, metadados e selectors: metadata, query, locate, here,
//! target, selector. Fatiado de `foundations.rs` no Passo 1032.

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::{
    native_figure, native_heading, native_metadata as native_metadata_func, native_table,
};
use crate::entities::args::Args;
use crate::entities::element_kind::ElementKind;
use crate::entities::file_id::FileId;
use crate::entities::introspector::Introspector;
use crate::entities::label::Label;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

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
        .map(|loc| match ctx.introspector.element_at(loc) {
            Some(c) => Value::Content(c.clone()),
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
    Ok(Value::Str("paged".into()))
}

/// **P504** — `selector(target)` — converte um kind string, label
/// string ou função nativa de elemento num `Value::Selector`.
pub fn native_selector(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use std::ptr::fn_addr_eq;

    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] if s.len() >= 2 && s.starts_with('<') && s.ends_with('>') => {
            let name = &s[1..s.len() - 1];
            Ok(Value::Selector(Selector::Label(crate::entities::label::Label(
                name.to_string(),
            ))))
        }
        [Value::Str(kind_str)] => match ElementKind::from_name(kind_str.as_str()) {
            Some(kind) => Ok(Value::Selector(Selector::Kind(kind))),
            None => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("selector(): kind '{}' não reconhecido", kind_str),
            )]),
        },
        [Value::Func(f)] => {
            let addr = f.native_fn_addr().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    "selector(): função nativa esperada".to_string(),
                )]
            })?;
            if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                Ok(Value::Selector(Selector::Kind(ElementKind::Heading)))
            } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                Ok(Value::Selector(Selector::Kind(ElementKind::Figure)))
            } else {
                Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "selector(): função nativa não suportada como selector".to_string(),
                )])
            }
        }
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("selector(): argumento inválido ({})", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("selector() requer 1 argumento, recebeu {}", args.items.len()),
        )]),
    }
}

/// **P844** — Mapeia uma função nativa de elemento para o `ElementKind`
/// correspondente (selector por tipo de elemento, paridade vanilla
/// `locate(heading)`).
fn element_kind_of_native_func(
    f: &crate::entities::func::Func,
) -> Option<crate::entities::element_kind::ElementKind> {
    use std::ptr::fn_addr_eq;

    let addr = f.native_fn_addr()?;
    if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
        Some(ElementKind::Heading)
    } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
        Some(ElementKind::Figure)
    } else if fn_addr_eq(addr, native_table as fn(_, _, _, _) -> _) {
        Some(ElementKind::Table)
    } else if fn_addr_eq(addr, native_metadata_func as fn(_, _, _, _) -> _) {
        Some(ElementKind::Metadata)
    } else {
        None
    }
}

/// **P209B (M9c)** — Parse selector arg para `native_query` +
/// `native_locate`.
fn parse_selector_arg(
    items: &[Value],
    func_name: &str,
) -> SourceResult<Selector> {
    let msg = |s: String| -> SourceResult<Selector> {
        Err(vec![SourceDiagnostic::error(Span::detached(), s)])
    };
    match items {
        [Value::Str(s)] if s.len() >= 2 && s.starts_with('<') && s.ends_with('>') => {
            let name = &s[1..s.len() - 1];
            Ok(Selector::Label(Label(name.to_string())))
        }
        [Value::Str(kind_str)] => match ElementKind::from_name(kind_str.as_str()) {
            Some(kind) => Ok(Selector::Kind(kind)),
            None => msg(format!(
                "{}(): kind '{}' não reconhecido (válidos: \
                     heading, figure, citation, metadata, state, \
                     state_update, outline, bibliography, equation, \
                     counter_update, table, list, enum, par, link, raw, \
                     quote, footnote). Para label, use `<nome>` syntax.",
                func_name, kind_str
            )),
        },
        [Value::Location(loc)] => {
            Ok(Selector::Location(*loc))
        }
        [Value::Selector(sel)] => {
            Ok(sel.clone())
        }
        [Value::Label(l)] => {
            Ok(Selector::Label(l.clone()))
        }
        [Value::Func(f)] => {
            match element_kind_of_native_func(f) {
                Some(kind) => Ok(Selector::Kind(kind)),
                None => msg(
                    "only element functions can be used as selectors".to_string(),
                ),
            }
        }
        [other] => msg(format!(
            "{}() requer string ou location, recebeu {}. \
             Tipos suportados: \"kind\", \"<label>\", \
             Value::Location. (Regex requer P209D; And/Or \
             ainda só Rust API.)",
            func_name,
            other.type_name()
        )),
        _ => msg(format!(
            "{}() requer 1 argumento (selector), recebeu {}",
            func_name,
            items.len()
        )),
    }
}
