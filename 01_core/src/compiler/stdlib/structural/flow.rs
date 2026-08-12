//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/flow.md
//! @prompt-hash 23318058
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de fluxo de bloco: `par`, `quote`, `footnote`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `quote(body, attribution: ?, block: false, quotes: true)` — emite
/// `Content::Quote`. Body posicional obrigatório (content ou string);
/// outros argumentos via named.
pub fn native_quote(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "quote() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "quote() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let mut attribution: Option<Content> = None;
    let mut block: bool = false;
    let mut quotes: bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "attribution" => {
                attribution = match value {
                    Value::Content(c) => Some(c.clone()),
                    Value::Str(s) => Some(Content::text(s.as_str())),
                    Value::None => None,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                            "quote(attribution:) espera content/string/none, recebeu {}",
                            other.type_name()
                        ),
                        )])
                    }
                };
            }
            "block" => match value {
                Value::Bool(b) => block = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "quote(block:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "quotes" => match value {
                Value::Bool(b) => quotes = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "quote(quotes:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("quote(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::quote(body, attribution, block, quotes)))
}

// ── P806 — `par(body, leading:?)` ────────────────────────────────────────

/// `par(body, leading:?)` → body como `Content` (parágrafo implícito).
///
/// **P806** (achado #12 de P798) — o vanilla tem `ParElem` invocável
/// (`#par[...]`); o cristalino não tem `Content::Par` (parágrafos são texto
/// plano em `Sequence` — `element_kind.rs`), logo o body é devolvido
/// **directamente**: o caso standalone é idêntico ao vanilla. A quebra de
/// fluxo block-level do vanilla (mid-paragraph) é limitação registada no L0
/// (`stdlib/structural.md` §`native_par`).
///
/// `leading:` (Length) é aplicado via `Content::Styled` com o custom
/// `"par.leading"` (mesmo canal do `#set par(leading:)`, F-5b/P373).
/// Propriedades vanilla conhecidas mas não honradas (`justify`, `spacing`,
/// `linebreaks`, `first-line-indent`, `hanging-indent`,
/// `justification-limits`) são aceites e ignoradas em silêncio (funções
/// nativas não têm acesso ao `Sink` — limitação registada).
pub fn native_par(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected content, found {}", other.type_name()),
            )])
        }
        None => {
            // Literal vanilla (`typst-library/src/model/par.rs` — medido:
            // `#par()` → "missing argument: body").
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: body".to_string(),
            )])
        }
    };

    let mut leading: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "leading" => match value {
                Value::Length(l) => leading = Some(*l),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "par(leading:) espera length, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            // Aceites e ignoradas (ver docstring — limitação registada).
            "justify" | "spacing" | "linebreaks" | "first-line-indent"
            | "hanging-indent" | "justification-limits" => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("argumento nomeado inesperado em par(): '{}'", other),
                )])
            }
        }
    }

    match leading {
        Some(l) => Ok(Value::Content(Content::Styled(
            Box::new(body),
            crate::entities::style::Styles::new()
                .push_custom("par.leading", Value::Length(l)),
        ))),
        None => Ok(Value::Content(body)),
    }
}

// ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table minimal ────────────────

/// `footnote(body, numbering:?)` — emite `Content::Footnote { body, numbering }`.
/// Body posicional obrigatório (content ou string).
/// P502: `numbering` named opcional; uso no marcador scope-out per ADR-0054.
pub fn native_footnote(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "footnote() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "footnote() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let mut numbering: Option<EcoString> = None;
    for (k, v) in args.named.iter() {
        match k.as_str() {
            "numbering" => {
                numbering = match v {
                    Value::Str(s) => Some(s.clone()),
                    Value::None => None,
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "footnote(numbering:): espera string, recebeu {}",
                                v.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("footnote(): argumento nomeado '{}' não suportado (cosméticos scope-out per ADR-0054 graded)", other),
                )]);
            }
        }
    }

    Ok(Value::Content(Content::footnote_with_numbering(body, numbering)))
}

// ── Passo 296 — `accent()` + `cancel()` math (P-math-accent-cancel) ──────
//
// **A.0.0 N=4 (HIV)**: features ausentes apesar de Tabela A.4
// marcar `parcial`. Materialização from-scratch. Padrão "variant
// rico" N=4 **preservado** (A.2 → (a) minimal; sem cosméticos
// `size`/`length`/`inverted`/`cross`/`angle`/`stroke`).

