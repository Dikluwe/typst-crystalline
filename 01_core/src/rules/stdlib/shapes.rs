//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de formas geométricas (rect, ellipse, circle, line, polygon).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::content::Content;
use crate::entities::geometry::{PathItem, ShapeKind, Stroke};
use crate::entities::layout_types::{Color, Point, Pt};
use crate::entities::span::Span;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── Primitivas geométricas (Passo 76) ────────────────────────────────────────

/// Converte um `Value` em `Color`.
///
/// Suporta nomes de cor conhecidos (`Value::Str`) e `Value::Color` directo.
/// Valores hex (`#rrggbb`) ficam para passo futuro — o parser real de cores
/// Typst requer um lexer dedicado.
pub(super) fn parse_color(val: &Value) -> Option<Color> {
    match val {
        Value::Color(c) => Some(*c),
        Value::Str(s) => match s.as_str() {
            "red"   => Some(Color::rgb(255, 0,   0)),
            "green" => Some(Color::rgb(0,   128, 0)),
            "blue"  => Some(Color::rgb(0,   0,   255)),
            "black" => Some(Color::rgb(0,   0,   0)),
            "white" => Some(Color::rgb(255, 255, 255)),
            _       => None,
        },
        _ => None,
    }
}

/// `rect(width?, height?, fill?, stroke?)` → `Content::Shape { kind: Rect, ... }`.
///
/// Fallback determinístico: sem `fill` nem `stroke` → stroke preta de 1pt.
/// Este é o único local onde este fallback existe — nem o layouter nem o
/// exportador têm permissão para inventar cores ou espessuras.
pub fn native_rect(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em rect(): '{}'", key),
            )]);
        }
    }

    let width  = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill   = args.named.get("fill").and_then(parse_color);

    let parsed_stroke: Option<Stroke> = args.named.get("stroke")
        .and_then(parse_color)
        .map(|c| Stroke { paint: c, thickness: 1.0 });

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind:   ShapeKind::Rect,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}

/// `ellipse(width?, height?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, ... }`.
///
/// Mesmo padrão de fallback que `native_rect`.
pub fn native_ellipse(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em ellipse(): '{}'", key),
            )]);
        }
    }

    let width  = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill   = args.named.get("fill").and_then(parse_color);

    let parsed_stroke: Option<Stroke> = args.named.get("stroke")
        .and_then(parse_color)
        .map(|c| Stroke { paint: c, thickness: 1.0 });

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Ellipse,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}

/// `circle(radius?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, width==height }`.
///
/// `radius` em pt. Converte para `width = height = radius * 2`.
pub fn native_circle(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["radius", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em circle(): '{}'", key),
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

    let (width, height) = match args.named.get("radius") {
        Some(r) => {
            let diameter = Value::Float(extract_pt(r) * 2.0);
            (Some(Box::new(diameter.clone())), Some(Box::new(diameter)))
        }
        None => (None, None),
    };

    let fill = args.named.get("fill").and_then(parse_color);

    let parsed_stroke: Option<Stroke> = args.named.get("stroke")
        .and_then(parse_color)
        .map(|c| Stroke { paint: c, thickness: 1.0 });

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Ellipse,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}

/// `line(dx?, dy?, stroke?)` → `Content::Shape { kind: Line, ... }`.
///
/// `dx`/`dy`: Float ou Length em pt. Omitidos → 0.0 (linha degenerada, válida).
/// Stroke preta por omissão — linhas não têm fill.
pub fn native_line(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["dx", "dy", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em line(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i)   => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }

    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);

    let stroke_color = args.named.get("stroke")
        .and_then(parse_color)
        .unwrap_or(Color::rgb(0, 0, 0)); // preto por omissão

    Ok(Value::Content(Content::Shape {
        kind:   ShapeKind::Line { dx, dy },
        width:  None,
        height: None,
        fill:   None,
        stroke: Some(Stroke { paint: stroke_color, thickness: 1.0 }),
    }))
}

/// Extrai um par de coordenadas (x, y) de um `Value::Array` com dois elementos numéricos.
fn extract_coordinate(val: &Value) -> Option<(f64, f64)> {
    match val {
        Value::Array(arr) if arr.len() == 2 => {
            let x = arr[0].cast_float()?;
            let y = arr[1].cast_float()?;
            Some((x, y))
        }
        _ => None,
    }
}

/// `polygon(pt1, pt2, ...; fill?, stroke?)` → `Content::Shape { kind: Path, ... }`.
///
/// Cada argumento posicional é um array `[x, y]` em pontos tipográficos.
/// A bounding box é calculada pelos pontos de controlo (DEBT-33).
pub fn native_polygon(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for (i, val) in args.items.iter().enumerate() {
        let (x, y) = extract_coordinate(val)
            .ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("polygon(): argumento {} não é uma coordenada válida", i),
            )])?;

        if i == 0 {
            path_items.push(PathItem::MoveTo(Point { x: Pt(x), y: Pt(y) }));
        } else {
            path_items.push(PathItem::LineTo(Point { x: Pt(x), y: Pt(y) }));
        }

        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    if path_items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "polygon() requer pelo menos um ponto".to_string(),
        )]);
    }

    path_items.push(PathItem::ClosePath);

    let fill   = args.named.get("fill").and_then(parse_color);
    let stroke = args.named.get("stroke").and_then(|v| {
        parse_color(v).map(|c| Stroke { paint: c, thickness: 1.0 })
    });

    let width  = if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
    let height = if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
    }))
}
