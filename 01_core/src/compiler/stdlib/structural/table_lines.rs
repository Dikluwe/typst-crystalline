//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/table_lines.md
//! @prompt-hash b6b79355
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de linha de tabela e grelha: `grid.hline`/`grid.vline`,
//! `table.hline`/`table.vline`, e o traço por omissão.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Align2D, Color, HAlign, VAlign};
use crate::entities::paint::Paint;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// Stroke padrão para `grid.hline`/`grid.vline`/`table.hline`/`table.vline`
/// quando `stroke` é omitido: 1pt preto com overhang vanilla (`true`).
pub(super) fn default_hline_stroke() -> Stroke {
    Stroke {
        paint: Paint::Solid(Color::rgb(0, 0, 0)),
        thickness: 1.0,
        overhang: true,
    }
}

/// Helper P512 — extrai `start`/`end` comuns a hline/vline.
fn extract_line_range(
    args: &Args,
    fn_name: &str,
) -> SourceResult<(usize, Option<usize>)> {
    let start = match args.named.get("start") {
        Some(v) => match v {
            Value::Int(n) if *n >= 0 => *n as usize,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}(start): espera int >= 0, recebeu {}",
                        fn_name,
                        other.type_name()
                    ),
                )])
            }
        },
        None => 0,
    };
    let end = match args.named.get("end") {
        Some(v) => match v {
            Value::Auto | Value::None => None,
            Value::Int(n) if *n >= 0 => Some(*n as usize),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}(end): espera int >= 0 ou auto, recebeu {}",
                        fn_name,
                        other.type_name()
                    ),
                )])
            }
        },
        None => None,
    };
    Ok((start, end))
}

/// `grid.hline(start, end, stroke, position)` — Passo 512.
pub fn native_grid_hline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "grid.hline")?;
    // **P739A** — `stroke: none` aceite (paridade vanilla, medido): a linha
    // não é desenhada. Zero-thickness proibido (hairline em PDF — P726).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(crate::compiler::stdlib::layout::extract_stroke(v, "grid.hline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position = match args.named.get("position") {
        Some(Value::Str(s)) => s.clone(),
        Some(Value::Align(Align2D { v: Some(VAlign::Top), .. })) => {
            EcoString::from("top")
        }
        Some(Value::Align(Align2D { v: Some(VAlign::Bottom), .. })) => {
            EcoString::from("bottom")
        }
        Some(Value::Auto) | Some(Value::None) | None => EcoString::from("auto"),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "grid.hline(position): espera str, alignment ou auto, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    if !matches!(position.as_str(), "top" | "bottom" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid.hline(position): deve ser 'top', 'bottom' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::grid_hline(start, end, 0, stroke, position)))
}

/// `grid.vline(start, end, stroke, position)` — Passo 512.
pub fn native_grid_vline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "grid.vline")?;
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(crate::compiler::stdlib::layout::extract_stroke(v, "grid.vline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position = match args.named.get("position") {
        Some(Value::Str(s)) => s.clone(),
        Some(Value::Align(Align2D { h: Some(HAlign::Left), .. })) => {
            EcoString::from("left")
        }
        Some(Value::Align(Align2D { h: Some(HAlign::Right), .. })) => {
            EcoString::from("right")
        }
        Some(Value::Auto) | Some(Value::None) | None => EcoString::from("left"),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "grid.vline(position): espera str, alignment ou auto, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    if !matches!(position.as_str(), "left" | "right" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid.vline(position): deve ser 'left', 'right' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::grid_vline(start, end, 0, stroke, position)))
}

/// `table.hline(start, end, stroke, position)` — Passo 512.
pub fn native_table_hline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "table.hline")?;
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(crate::compiler::stdlib::layout::extract_stroke(v, "table.hline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position =
        match args.named.get("position") {
            Some(Value::Str(s)) => s.clone(),
            Some(Value::Align(Align2D { v: Some(VAlign::Top), .. })) => {
                EcoString::from("top")
            }
            Some(Value::Align(Align2D { v: Some(VAlign::Bottom), .. })) => {
                EcoString::from("bottom")
            }
            Some(Value::Auto) | Some(Value::None) | None => EcoString::from("auto"),
            Some(other) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "table.hline(position): espera str, alignment ou auto, recebeu {}",
                    other.type_name()
                ),
            )]),
        };
    if !matches!(position.as_str(), "top" | "bottom" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table.hline(position): deve ser 'top', 'bottom' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::table_hline(start, end, 0, stroke, position)))
}

/// `table.vline(start, end, stroke, position)` — Passo 512.
pub fn native_table_vline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "table.vline")?;
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(crate::compiler::stdlib::layout::extract_stroke(v, "table.vline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position =
        match args.named.get("position") {
            Some(Value::Str(s)) => s.clone(),
            Some(Value::Align(Align2D { h: Some(HAlign::Left), .. })) => {
                EcoString::from("left")
            }
            Some(Value::Align(Align2D { h: Some(HAlign::Right), .. })) => {
                EcoString::from("right")
            }
            Some(Value::Auto) | Some(Value::None) | None => EcoString::from("left"),
            Some(other) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "table.vline(position): espera str, alignment ou auto, recebeu {}",
                    other.type_name()
                ),
            )]),
        };
    if !matches!(position.as_str(), "left" | "right" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table.vline(position): deve ser 'left', 'right' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::table_vline(start, end, 0, stroke, position)))
}

