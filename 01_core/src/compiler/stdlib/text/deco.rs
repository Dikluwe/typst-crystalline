//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text/deco.md
//! @prompt-hash c14504e3
//! @layer L1
//! @updated 2026-08-13
//!
//! Decoração de texto: `underline`, `strike`, `overline`, `highlight`.
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::shapes::parse_color;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Color, Length};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

// ── Passo 284 — decoração textual (underline / strike / overline) ───────────
//
// Paridade vanilla `text/deco.rs`: três funções com `body` posicional
// obrigatório + cosméticos `stroke`/`offset`/`extent` opcionais (named).
// Atributos scope-out (`evade`, `background`, objecto Stroke rico) per
// diagnóstico P284 §A.1 + ADR-0054 graded.

/// Discriminador interno para escolher o construtor de `Content::*` no
/// helper partilhado `build_decoration`. Não é exposto em L0 — é apenas
/// um eixo de variação dentro de `text.rs`.
#[derive(Clone, Copy)]
enum DecoKind {
    Underline,
    Strike,
    Overline,
}

fn build_decoration(kind: DecoKind, args: &Args, fn_name: &str) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{fn_name}() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{fn_name}() exige body como argumento posicional"),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{fn_name}() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    let mut stroke: Option<Color> = None;
    let mut offset: Option<Length> = None;
    let mut extent: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "stroke" => {
                // Apenas Color (paint puro) — Stroke rico vanilla é
                // scope-out per diagnóstico §A.1. `none` desactiva default.
                stroke = match value {
                    Value::None => None,
                    v => match parse_color(v) {
                        Some(c) => Some(c),
                        None => {
                            return Err(vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "{fn_name}(stroke:) espera color ou none, recebeu {}",
                                    v.type_name()
                                ),
                            )])
                        }
                    },
                };
            }
            "offset" => {
                offset = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    Value::Int(i) => Some(Length::pt(*i as f64)),
                    Value::Float(f) => Some(Length::pt(*f)),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "{fn_name}(offset:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            "extent" => {
                extent = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    Value::Int(i) => Some(Length::pt(*i as f64)),
                    Value::Float(f) => Some(Length::pt(*f)),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "{fn_name}(extent:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            // P284 §A.1 scope-out: erros amigáveis para parâmetros vanilla
            // adiados, em vez de "argumento inesperado" genérico.
            "evade" | "background" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "{fn_name}({}:) não suportado neste passo \
                         (P284 §A.1 scope-out / ADR-0054 graded)",
                        key,
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("{fn_name}(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    // Modelo D (Lote 4 P319): construtores ergonómicos (body sem Box).
    let c = match kind {
        DecoKind::Underline => Content::underline(body, stroke, offset, extent),
        DecoKind::Strike => Content::strike(body, stroke, offset, extent),
        DecoKind::Overline => Content::overline(body, stroke, offset, extent),
    };
    Ok(Value::Content(c))
}

pub fn native_underline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Underline, args, "underline")
}

pub fn native_strike(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Strike, args, "strike")
}

pub fn native_overline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Overline, args, "overline")
}

// ── Passo 449 — `highlight(body, fill: color)` ──────────────────────────────
//
// Paridade vanilla `text/highlight.rs::HighlightElem`: fundo colorido por
// detrás do texto. Modelo minimal: `Content::Styled` com `Style::Highlight`.

/// Cor amarela padrão do Typst para highlight (`#ff236`).
fn default_highlight_color() -> Color {
    Color::rgba(255, 242, 54, 255)
}

/// `highlight(body, fill:?, radius:?, extent:?)` → content com fundo colorido.
/// P471: adiciona `radius: Length` (cantos arredondados) e `extent: Length`
/// (extensão horizontal). Cor default: amarelo vanilla `#fff236`.
pub fn native_highlight(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "highlight() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "highlight() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "highlight() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    let mut fill: Option<Color> = Some(default_highlight_color());
    let mut radius: Option<Length> = None;
    let mut extent: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "fill" => {
                fill = match value {
                    Value::None => None,
                    v => match parse_color(v) {
                        Some(c) => Some(c),
                        None => {
                            return Err(vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "highlight(fill:) espera color ou none, recebeu {}",
                                    v.type_name()
                                ),
                            )])
                        }
                    },
                };
            }
            "radius" => {
                radius = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "highlight(radius:) espera length ou none, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            "extent" => {
                extent = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "highlight(extent:) espera length ou none, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("highlight() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::highlight_full(body, fill, radius, extent)))
}
