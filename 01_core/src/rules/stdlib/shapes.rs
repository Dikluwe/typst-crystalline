//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 292ed749
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
use crate::entities::paint::Paint;
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
        .map(|c| Stroke { paint: Paint::Solid(c), thickness: 1.0, overhang: false });

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Paint::Solid(Color::rgb(0, 0, 0)), thickness: 1.0, overhang: false })
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
        .map(|c| Stroke { paint: Paint::Solid(c), thickness: 1.0, overhang: false });

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Paint::Solid(Color::rgb(0, 0, 0)), thickness: 1.0, overhang: false })
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
        .map(|c| Stroke { paint: Paint::Solid(c), thickness: 1.0, overhang: false });

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Paint::Solid(Color::rgb(0, 0, 0)), thickness: 1.0, overhang: false })
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
        stroke: Some(Stroke { paint: Paint::Solid(stroke_color), thickness: 1.0, overhang: false }),
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
/// Bbox calculada via `geometry::path_bbox` (analítica para CubicTo;
/// equivalente a min/max para LineTo-only — P277 consolidação DEBT-33).
pub fn native_polygon(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();

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
        parse_color(v).map(|c| Stroke { paint: Paint::Solid(c), thickness: 1.0, overhang: false })
    });

    // P277 — DEBT-33 CLOSED: usar geometry::path_bbox para bbox.
    // Para LineTo-only paths preserva bit-exact min/max behavior.
    let (min_x, min_y, max_x, max_y) =
        crate::entities::geometry::path_bbox(&path_items);
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

// ── Passo 293-294 — `curve(...)` constructor stdlib ─────────────────────
//
// **P293 H6 (descoberta empírica A.0.0)**: activação posterior de
// `PathItem::CubicTo` via stdlib novo `native_curve` (variant existia
// inerte desde P277).
//
// **P294 H1' (descoberta empírica A.0.0 N=2)**: vanilla typst NÃO tem
// variant `QuadraticTo` interno; converte q→c em construct-time via
// `control_q2c(p, c) = (p + 2c) / 3` em `lab/.../shapes.rs:215-220`.
// Cristalino adopta o mesmo padrão — sem variant novo, sem alteração
// de emit, hash `export.rs 66cb8ac3` preservado (11º passo consecutivo).
//
// Conversão exacta em `f64`: dado Bézier quadrática Q(t) com control
// único C, a Bézier cúbica equivalente B(t) usa control points:
//   C₁ = (P₀ + 2·C) / 3        // start + 2/3 do vector start→control
//   C₂ = (P₂ + 2·C) / 3        // end + 2/3 do vector end→control
// As duas curvas paramétricas são pointwise idênticas.

/// `curve(seg1, seg2, ...; fill?, stroke?)` → `Content::Shape { kind: Path }`
///
/// Cada segmento posicional é um array com tipo + coordenadas:
/// - `("move", [x, y])` → `PathItem::MoveTo`
/// - `("line", [x, y])` → `PathItem::LineTo`
/// - `("cubic", [c1x, c1y], [c2x, c2y], [ex, ey])` → `PathItem::CubicTo`
/// - `("quadratic", [cx, cy], [ex, ey])` → `PathItem::CubicTo` via
///   conversão q→c paridade vanilla (P294 H1').
/// - `("close",)` → `PathItem::ClosePath`
///
/// Divergência aproximada vs vanilla typst (`curve.move`/`curve.cubic`
/// scope methods): cristalino usa tuples descritivos por simplicidade
/// (proc macros `#elem(scope)` não materializados em L1).
pub fn native_curve(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();
    // P294: tracking de last_point para conversão q→c em "quadratic"
    // (paridade vanilla `Curve::last_point`). Arranca em (0,0) — caso
    // fronteira de quadratic sem move anterior é tratado consistentemente.
    let mut last_point: Point = Point::ZERO;

    for (i, val) in args.items.iter().enumerate() {
        let arr = match val {
            Value::Array(a) if !a.is_empty() => a,
            _ => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("curve(): argumento {} não é um array de segmento válido", i),
            )]),
        };

        let kind = match &arr[0] {
            Value::Str(s) => s.as_str(),
            _ => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("curve(): segmento {}: primeiro elemento deve ser string (kind)", i),
            )]),
        };

        match kind {
            "move" => {
                if arr.len() != 2 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("curve() 'move' requer 1 coordenada, segmento {}", i),
                    )]);
                }
                let (x, y) = extract_coordinate(&arr[1]).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() 'move' segmento {}: coordenada inválida", i),
                )])?;
                let target = Point { x: Pt(x), y: Pt(y) };
                path_items.push(PathItem::MoveTo(target));
                last_point = target;
            }
            "line" => {
                if arr.len() != 2 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("curve() 'line' requer 1 coordenada, segmento {}", i),
                    )]);
                }
                let (x, y) = extract_coordinate(&arr[1]).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() 'line' segmento {}: coordenada inválida", i),
                )])?;
                let target = Point { x: Pt(x), y: Pt(y) };
                path_items.push(PathItem::LineTo(target));
                last_point = target;
            }
            "cubic" => {
                if arr.len() != 4 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("curve() 'cubic' requer 3 coordenadas (c1, c2, end), segmento {}", i),
                    )]);
                }
                let (c1x, c1y) = extract_coordinate(&arr[1]).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() 'cubic' segmento {}: c1 inválida", i),
                )])?;
                let (c2x, c2y) = extract_coordinate(&arr[2]).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() 'cubic' segmento {}: c2 inválida", i),
                )])?;
                let (ex, ey) = extract_coordinate(&arr[3]).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() 'cubic' segmento {}: end inválida", i),
                )])?;
                let end = Point { x: Pt(ex), y: Pt(ey) };
                path_items.push(PathItem::CubicTo(
                    Point { x: Pt(c1x), y: Pt(c1y) },
                    Point { x: Pt(c2x), y: Pt(c2y) },
                    end,
                ));
                last_point = end;
            }
            "close" => {
                path_items.push(PathItem::ClosePath);
                // `close` não move last_point (paridade vanilla).
            }
            // P294 H1' (descoberta empírica A.0.0 N=2): paridade vanilla
            // — converte quadratic→cubic em construct-time, sem variant
            // novo. Fórmula `control_q2c(p, c) = (p + 2c) / 3` aplicada a
            // start e end (`lab/.../typst-layout/src/shapes.rs:215-220`).
            "quadratic" => {
                if arr.len() != 3 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("curve() 'quadratic' requer 2 coordenadas (control, end), segmento {}", i),
                    )]);
                }
                let (qx, qy) = extract_coordinate(&arr[1]).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() 'quadratic' segmento {}: control inválido", i),
                )])?;
                let (ex, ey) = extract_coordinate(&arr[2]).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() 'quadratic' segmento {}: end inválido", i),
                )])?;
                let p0x = last_point.x.0;
                let p0y = last_point.y.0;
                let c1x = (p0x + 2.0 * qx) / 3.0;
                let c1y = (p0y + 2.0 * qy) / 3.0;
                let c2x = (ex  + 2.0 * qx) / 3.0;
                let c2y = (ey  + 2.0 * qy) / 3.0;
                let end = Point { x: Pt(ex), y: Pt(ey) };
                path_items.push(PathItem::CubicTo(
                    Point { x: Pt(c1x), y: Pt(c1y) },
                    Point { x: Pt(c2x), y: Pt(c2y) },
                    end,
                ));
                last_point = end;
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("curve() kind '{}' desconhecido (esperado: move/line/cubic/close)", other),
                )]);
            }
        }
    }

    if path_items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "curve() requer pelo menos um segmento".to_string(),
        )]);
    }

    let fill   = args.named.get("fill").and_then(parse_color);
    let stroke = args.named.get("stroke").and_then(|v| {
        parse_color(v).map(|c| Stroke { paint: Paint::Solid(c), thickness: 1.0, overhang: false })
    });

    // P277 — `path_bbox` analítica reusada sem alteração.
    let (min_x, min_y, max_x, max_y) =
        crate::entities::geometry::path_bbox(&path_items);
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
