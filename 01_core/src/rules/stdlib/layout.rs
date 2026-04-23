//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de layout (align, place, grid, page).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use super::expect_no_named;
use crate::entities::file_id::FileId;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::layout_types::{Align2D, TrackSizing};
use crate::entities::span::Span;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── Align / Place (Passo 82) ────────────────────────────────────────────────

/// `align(alignment, body)` → `Content::Align`.
///
/// `alignment` aceita `Value::Align` (sintaxe preferida pós-Passo 84.5,
/// ex: `align(center + bottom, ...)`) ou `Value::Str` (sintaxe legacy,
/// ex: `align("center", ...)`) — ver DEBT-36 (encerrado).
/// `body` é o primeiro argumento posicional do tipo Content.
pub fn native_align(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let alignment = extract_alignment(args, Align2D::default());

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "align() exige um bloco de conteúdo".to_string())])?;

    Ok(Value::Content(Content::Align {
        alignment,
        body: Box::new(body),
    }))
}

/// `place(alignment, dx?, dy?, scope?, body)` → `Content::Place`.
///
/// `dx`/`dy` em pt deslocam o conteúdo a partir da posição alinhada.
/// `scope` (Passo 84.6, encerra DEBT-37): `"column"` (default — ancora à
/// célula activa de Grid, ou à página fora de Grid) ou `"parent"` (ancora
/// sempre à página). Aceita string ou omissão.
pub fn native_place(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["dx", "dy", "scope"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em place(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f)  => *f,
            Value::Int(i)    => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }

    // Default "top-left" para Place, "left" (default Align2D) para Align.
    let alignment = extract_alignment(
        args,
        Align2D { h: Some(crate::entities::layout_types::HAlign::Left),
                  v: Some(crate::entities::layout_types::VAlign::Top) },
    );

    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);

    let scope = match args.named.get("scope") {
        Some(Value::Str(s)) => match s.as_str() {
            "column" => crate::entities::layout_types::PlaceScope::Column,
            "parent" => crate::entities::layout_types::PlaceScope::Parent,
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("place(): scope deve ser \"column\" ou \"parent\", recebeu \"{}\"", other),
            )]),
        },
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("place(): scope deve ser string, recebeu {}", other.type_name()),
        )]),
        None => crate::entities::layout_types::PlaceScope::default(),
    };

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "place() exige um bloco de conteúdo".to_string())])?;

    Ok(Value::Content(Content::Place {
        alignment,
        dx,
        dy,
        scope,
        body: Box::new(body),
    }))
}

/// Helper Passo 84.5: extrai alinhamento do primeiro argumento posicional
/// que case com `Value::Align` ou `Value::Str`. Sintaxe preferida `Value::Align`,
/// sintaxe legacy via `Align2D::from_string`. Caso nenhum case, retorna `default`.
fn extract_alignment(args: &Args, default: Align2D) -> Align2D {
    args.items.iter()
        .find_map(|v| match v {
            Value::Align(a) => Some(*a),
            Value::Str(s)   => Some(Align2D::from_string(s.as_str())),
            _ => None,
        })
        .unwrap_or(default)
}

// ── Grid (Passo 80) ────────────────────────────────────────────────────────

fn parse_track_sizing(val: &Value) -> Option<TrackSizing> {
    match val {
        Value::Float(f)    => Some(TrackSizing::Fixed(*f)),
        Value::Length(l)   => Some(TrackSizing::Fixed(l.abs.to_pt())),
        Value::Fraction(fr) => Some(TrackSizing::Fraction(*fr)),
        Value::Auto        => Some(TrackSizing::Auto),
        Value::Str(s) if s.as_str() == "auto" => Some(TrackSizing::Auto),
        _ => None,
    }
}

fn extract_tracks(val: Option<&Value>) -> Vec<TrackSizing> {
    match val {
        Some(Value::Array(arr)) => arr.iter().filter_map(parse_track_sizing).collect(),
        // `grid(rows: 3)` ou `grid(columns: 3)` → 3 tracks Auto (Passo 83).
        Some(Value::Int(n)) if *n > 0 => vec![TrackSizing::Auto; *n as usize],
        Some(v) => parse_track_sizing(v).into_iter().collect(),
        None    => vec![],
    }
}

/// `grid(columns?, rows?, ...cells)` → `Content::Grid`.
pub fn native_grid(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["columns", "rows"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em grid(): '{}'", key),
            )]);
        }
    }
    let mut columns = extract_tracks(args.named.get("columns"));
    let mut rows    = extract_tracks(args.named.get("rows"));
    // Defaults Passo 83 — `rows` omitido ou tuplo vazio → uma linha Auto repetida
    // para todas as linhas geradas pelo número de cells. Idem para columns.
    if columns.is_empty() {
        columns = vec![TrackSizing::Auto];
    }
    if rows.is_empty() {
        rows = vec![TrackSizing::Auto];
    }
    let cells: Vec<Content> = args.items.iter()
        .filter_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .collect();
    Ok(Value::Content(Content::Grid { columns, rows, cells }))
}

/// `#set page(width: w, height: h, margin: m)` — configura as dimensões da página (Passo 81).
pub fn native_page(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    fn extract_pt(val: &Value) -> Option<f64> {
        match val {
            Value::Length(l) => Some(l.abs.to_pt()),
            Value::Float(f)  => Some(*f),
            Value::Int(i)    => Some(*i as f64),
            _                => None,
        }
    }

    for key in args.named.keys() {
        if !["width", "height", "margin"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em page(): '{}'", key),
            )]);
        }
    }

    let width  = args.named.get("width") .and_then(extract_pt);
    let height = args.named.get("height").and_then(extract_pt);
    let margin = args.named.get("margin").and_then(extract_pt);

    Ok(Value::Content(Content::SetPage { width, height, margin }))
}

