//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/shapes.md
//! @prompt-hash 8a488ef2
//! @layer L1
//! @updated 2026-06-24
//!
//! Funções nativas de formas geométricas (rect, square, ellipse, circle, line, polygon).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::corners::Corners;
use crate::entities::elements::curve::{CloseMode, CurvePoint, CurveSegment};
use crate::entities::elements::shape::ShapeElem;
use crate::entities::file_id::FileId;
use crate::entities::geometry::{FillRule, PathItem, ShapeKind, Stroke};
use crate::entities::layout_types::{Color, Length, Point, Pt};
use crate::entities::paint::Paint;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Primitivas geométricas (Passo 76) ────────────────────────────────────────

/// Extrai somente uma cor já tipada pelo avaliador.
pub(super) fn parse_color(val: &Value) -> Option<Color> {
    match val {
        Value::Color(c) => Some(c.clone()),
        _ => None,
    }
}

/// Converte um `Value` em `Paint` (fill de shape).
pub(super) fn parse_paint(val: &Value, span: Span) -> SourceResult<Option<Paint>> {
    match val {
        Value::Color(c) => Ok(Some(Paint::Solid(c.clone()))),
        Value::Tiling(t) => Ok(Some(Paint::Tiling((**t).clone()))),
        Value::Gradient(g) => Ok(Some(Paint::Gradient(g.clone()))),
        Value::None => Ok(None),
        Value::Str(_) => Err(vec![SourceDiagnostic::error(
            span,
            "expected color, gradient, tiling, or none, found string".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected color, gradient, tiling, or none, found {}",
                other.type_name()
            ),
        )]),
    }
}

/// Preserva o stroke rico já produzido pelo avaliador.
fn parse_shape_stroke(val: &Value, fn_name: &str) -> SourceResult<Stroke> {
    if let Some(color) = parse_color(val) {
        return Ok(Stroke {
            paint: Paint::Solid(color),
            thickness: 1.0,
            overhang: true,
            ..Stroke::default()
        });
    }
    if matches!(val, Value::Str(_)) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "expected length, color, gradient, tiling, dictionary, or stroke, found string"
                .to_string(),
        )]);
    }
    crate::compiler::stdlib::layout::extract_stroke(val, fn_name, "stroke")
}

/// `rect(width?, height?, fill?, stroke?, radius?)` → `Content::Shape`.
///
/// Fallback determinístico: sem `fill` nem `stroke` → stroke preta de 1pt.
/// Este é o único local onde este fallback existe — nem o layouter nem o
/// exportador têm permissão para inventar cores ou espessuras.
pub fn native_rect(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke", "radius"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em rect(): '{}'", key),
            )]);
        }
    }

    let width = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    // A forma por-lado é aceita pela superfície existente, mas a entidade
    // uniforme ainda não a representa; preserva o scope-out anterior.
    let parsed_stroke = match args.named.get("stroke") {
        Some(Value::Dict(_)) => None,
        Some(value) => Some(parse_shape_stroke(value, "rect")?),
        None => None,
    };

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    let radius = match args.named.get("radius") {
        Some(val) => {
            crate::compiler::stdlib::layout::extract_corners_length_value(val, "rect")?
        }
        None => Corners::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::shape_with_radius(
        ShapeKind::Rect,
        width,
        height,
        fill,
        final_stroke,
        radius,
    )))
}

/// `square(width, height: auto, fill?, stroke?, radius?)` → `Content::Shape { kind: Rect, ... }`.
///
/// Helper morfológico sobre `Rect`: `square(w)` é equivalente a
/// `rect(width: w, height: w)`. Se `height` for omitido, assume o valor de
/// `width`. O fallback de stroke preta 1pt segue o mesmo determinismo de
/// `native_rect`.
pub fn native_square(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke", "radius"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em square(): '{}'", key),
            )]);
        }
    }

    let width = if let Some(w) = args.items.first() {
        Some(Box::new(w.clone()))
    } else {
        args.named.get("width").cloned().map(Box::new)
    };

    let Some(width) = width else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "square() requer um argumento de largura",
        )]);
    };

    let height = args
        .named
        .get("height")
        .cloned()
        .map(Box::new)
        .unwrap_or_else(|| width.clone());

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "square"))
        .transpose()?;

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    let radius = match args.named.get("radius") {
        Some(val) => {
            crate::compiler::stdlib::layout::extract_corners_length_value(val, "square")?
        }
        None => Corners::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::shape_with_radius(
        ShapeKind::Rect,
        Some(width),
        Some(height),
        fill,
        final_stroke,
        radius,
    )))
}

/// `ellipse(width?, height?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, ... }`.
///
/// Mesmo padrão de fallback que `native_rect`.
pub fn native_ellipse(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em ellipse(): '{}'", key),
            )]);
        }
    }

    let width = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "ellipse"))
        .transpose()?;

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::shape(
        ShapeKind::Ellipse,
        width,
        height,
        fill,
        final_stroke,
    )))
}

/// `circle(radius?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, width==height }`.
///
/// `radius` em pt. Converte para `width = height = radius * 2`.
pub fn native_circle(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["radius", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em circle(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i) => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0, // neutro: N16[β] — Value não-numérico retorna 0.0 na extracção de dimensões de polígono
        }
    }

    let (width, height) = match args.named.get("radius") {
        Some(r) => {
            let diameter = Value::Float(extract_pt(r) * 2.0);
            (Some(Box::new(diameter.clone())), Some(Box::new(diameter)))
        }
        None => (None, None),
    };

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "circle"))
        .transpose()?;

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::shape(
        ShapeKind::Ellipse,
        width,
        height,
        fill,
        final_stroke,
    )))
}

/// `line(dx?, dy?, stroke?)` → `Content::Shape { kind: Line, ... }`.
///
/// `dx`/`dy`: Float ou Length em pt. Omitidos → 0.0 (linha degenerada, válida).
/// Stroke preta por omissão — linhas não têm fill.
pub fn native_line(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["dx", "dy", "stroke", "start", "end", "length", "angle"]
            .contains(&key.as_str())
        {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em line(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i) => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0, // neutro: N16[β] — Value não-numérico retorna 0.0 na extracção de dimensões de linha
        }
    }

    let stroke = match args.named.get("stroke") {
        Some(value) => parse_shape_stroke(value, "line")?,
        None => Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        },
    };

    // **P739B** — `start:`/`end:` (paridade vanilla — medido:
    // `line(start: (0pt, 0pt), end: (50pt, 50pt))` → exit 0). Interface
    // legada `dx`/`dy` mantida.
    // **P804** — `length:`/`angle:` (paridade vanilla `LineElem`): só
    // respeitados se `end` for `none` (medido: `line(length: 3cm,
    // end: (1cm, 1cm))` compila no vanilla com `length` ignorado). Sem
    // `end`: `dx = cos(angle)·length`, `dy = sin(angle)·length`
    // (vanilla `layout_line`); default `length: 30pt`, `angle: 0deg`.
    let has_polar_extent = args.named.contains_key("length")
        || args.named.contains_key("angle")
        || args.named.contains_key("start");
    let parse_line_coordinate = |value: &Value, name: &str| -> SourceResult<(f64, f64)> {
        if let Value::Array(values) = value {
            if values.len() != 2 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "array must contain exactly two items",
                )]);
            }
        }
        extract_coordinate(value).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                format!("line({name}): espera array de 2 coordenadas"),
            )]
        })
    };
    let (start_x, start_y, end_x, end_y, absolute_path) = match args.named.get("end") {
        Some(end_v) => {
            if args.named.contains_key("dx") || args.named.contains_key("dy") {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "line(): 'end' não pode ser combinado com 'dx'/'dy'".to_string(),
                )]);
            }
            let (ex, ey) = parse_line_coordinate(end_v, "end")?;
            let (sx, sy) = match args.named.get("start") {
                Some(start_v) => parse_line_coordinate(start_v, "start")?,
                None => (0.0, 0.0),
            };
            // `length`/`angle` presentes com `end` são ignorados (paridade
            // vanilla: "only respected if end is none", medido em P804).
            (sx, sy, ex, ey, true)
        }
        None if has_polar_extent => {
            // ── P804 — caminho `length`/`angle` (sem `end`) ──────────────
            if args.named.contains_key("dx") || args.named.contains_key("dy") {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "line(): 'length'/'angle' não pode ser combinado com 'dx'/'dy'"
                        .to_string(),
                )]);
            }
            let (sx, sy) = match args.named.get("start") {
                Some(start_v) => parse_line_coordinate(start_v, "start")?,
                None => (0.0, 0.0),
            };
            let length = match args.named.get("length") {
                Some(v @ (Value::Float(_) | Value::Int(_) | Value::Length(_))) => {
                    extract_pt(v)
                }
                Some(Value::Ratio(_)) => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "line(length): percentagem não é suportada (scope-out) — o native não tem a região para resolver ratio".to_string(),
                    )])
                }
                Some(other) => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected length, found {}", other.type_name()),
                    )])
                }
                None => 30.0, // default vanilla `Abs::pt(30.0)`
            };
            let angle_rad = match args.named.get("angle") {
                Some(Value::Angle(a)) => a.to_rad(),
                Some(other) => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected angle, found {}", other.type_name()),
                    )])
                }
                None => 0.0,
            };
            let dx = angle_rad.cos() * length;
            let dy = angle_rad.sin() * length;
            (sx, sy, sx + dx, sy + dy, true)
        }
        None => {
            let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
            let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);
            (0.0, 0.0, dx, dy, false)
        }
    };

    if !absolute_path {
        return Ok(Value::Content(Content::shape(
            ShapeKind::Line { dx: end_x, dy: end_y },
            None,
            None,
            None,
            Some(stroke),
        )));
    }

    let width = 0.0_f64.max(start_x).max(end_x);
    let height = 0.0_f64.max(start_y).max(end_y);
    Ok(Value::Content(Content::shape(
        ShapeKind::Path(vec![
            PathItem::MoveTo(Point { x: Pt(start_x), y: Pt(start_y) }),
            PathItem::LineTo(Point { x: Pt(end_x), y: Pt(end_y) }),
        ]),
        Some(Box::new(Value::Length(Length::pt(width)))),
        Some(Box::new(Value::Length(Length::pt(height)))),
        None,
        Some(stroke),
    )))
}

/// Extrai um par de coordenadas (x, y) de um `Value::Array` com dois elementos numéricos.
fn extract_coordinate(val: &Value) -> Option<(f64, f64)> {
    match val {
        Value::Array(arr) if arr.len() == 2 => {
            let x = coord_component(&arr[0])?;
            let y = coord_component(&arr[1])?;
            Some((x, y))
        }
        _ => None,
    }
}

/// Componente de coordenada de vértice/segmento.
///
/// **P732** — aceita `Length` (paridade vanilla: os vértices de `polygon`
/// são `Rel<Length>`; a componente `em` e `Ratio` ficam scope-out, mesmo
/// precedente de P513 em `curve`) além de `Float`/`Int` em pt (interface
/// cristalina legada, partilhada com `curve`).
fn coord_component(val: &Value) -> Option<f64> {
    match val {
        Value::Length(l) => Some(l.abs.to_pt()),
        _ => val.cast_float(),
    }
}

/// Vértice de `polygon`: só `Length` (paridade vanilla — **P734**).
///
/// O vanilla exige `Rel<Length>` nos vértices e rejeita números nus
/// ("expected relative length, found integer/float", medido). `Ratio`
/// (`50%`) é scope-out: o vanilla aceita (resolve no layout contra o
/// contentor), mas o constructor cristalino corre em tempo de eval, sem
/// dimensão de referência — erro explícito de scope-out. A restrição não
/// se aplica a `curve` (interface por tuples, divergência intencional
/// documentada — o vanilla rejeita tuples: "expected content, found array").
fn extract_vertex(val: &Value, index: usize) -> SourceResult<(f64, f64)> {
    let arr = match val {
        Value::Array(a) if a.len() == 2 => a,
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("polygon(): argumento {} não é uma coordenada válida", index),
            )]);
        }
    };
    let x = vertex_component(&arr[0])?;
    let y = vertex_component(&arr[1])?;
    Ok((x, y))
}

fn vertex_component(val: &Value) -> SourceResult<f64> {
    match val {
        Value::Length(l) => Ok(l.abs.to_pt()),
        // Paridade verbatim do vanilla (medido): "integer", não "int".
        Value::Int(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "expected relative length, found integer".to_string(),
        )]),
        Value::Ratio(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "polygon(): coordenada ratio (50%) não é resolvível em tempo de eval — scope-out (o vanilla aceita)".to_string(),
        )]),
        // **P741** — pré-P842 o caminho real do utilizador: `50%` chegava
        // como `Value::Relative` com abs zero. Desde P842 (#32) chega como
        // `Value::Ratio` (braço acima); este braço fica para `Relative`
        // construídos por outras vias. A sonda de P741 mediu o vanilla:
        // o ratio resolve contra o contentor em tempo de layout (x contra a
        // largura, y contra a altura — medido exacto: `50%` ≡ `50pt` num
        // box de 100pt, diff 0.0000%). O cristalino não tem altura de
        // contentor inline disponível no emit do shape (Boxed usa
        // `unconstrained_height`) — scope-out reforçado, custo no
        // relatório do passo. Antes deste braço, `Relative` caía no
        // `other` e produzia a mensagem absurda "expected relative
        // length, found relative length".
        Value::Relative(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "polygon(): coordenada relativa (50%) não é resolvível em tempo de eval — scope-out (o vanilla resolve contra o contentor no layout)".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected relative length, found {}", other.type_name()),
        )]),
    }
}

/// Extrai um `CurvePoint` de um `Value::Array` de 2 elementos.
///
/// Aceita `Value::Length`, `Value::Float` ou `Value::Int` (este último
/// convertido para pt). `Value::Relative`/`Ratio` ficam fora do scope
/// minimal do Passo 513.
fn extract_curve_point(
    val: &Value,
    fn_name: &str,
    arg_name: &str,
) -> SourceResult<CurvePoint> {
    let arr = match val {
        Value::Array(a) if a.len() == 2 => a,
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}: {} deve ser um array de 2 elementos", fn_name, arg_name),
            )]);
        }
    };

    let x = extract_curve_length(&arr[0], fn_name, arg_name, "x")?;
    let y = extract_curve_length(&arr[1], fn_name, arg_name, "y")?;
    Ok(CurvePoint { x, y })
}

fn extract_curve_length(
    val: &Value,
    fn_name: &str,
    arg_name: &str,
    coord: &str,
) -> SourceResult<Length> {
    match val {
        Value::Length(l) => Ok(*l),
        Value::Float(f) => Ok(Length::pt(*f)),
        Value::Int(i) => Ok(Length::pt(*i as f64)),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}: {}.{} deve ser length, float ou int", fn_name, arg_name, coord),
        )]),
    }
}

/// `polygon(pt1, pt2, ...; fill?, stroke?)` → `Content::Shape { kind: Path, ... }`.
///
/// Cada argumento posicional é um array `[x, y]` de `Length` — desde o
/// Passo 734 só `Length` é aceite (paridade vanilla: "expected relative
/// length, found integer/float"; ver `extract_vertex`).
/// Bbox calculada via `geometry::path_bbox` (analítica para CubicTo;
/// equivalente a min/max para LineTo-only — P277 consolidação DEBT-33).
pub fn native_polygon(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();

    for (i, val) in args.items.iter().enumerate() {
        // **P734** — `extract_vertex`: só `Length` (paridade vanilla);
        // números nus rejeitados com a mensagem do vanilla.
        let (x, y) = extract_vertex(val, i)?;

        if i == 0 {
            path_items.push(PathItem::MoveTo(Point { x: Pt(x), y: Pt(y) }));
        } else {
            path_items.push(PathItem::LineTo(Point { x: Pt(x), y: Pt(y) }));
        }
    }

    if path_items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "polygon() requer pelo menos um ponto".to_string(),
        )]);
    }

    path_items.push(PathItem::ClosePath);

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();
    let fill_rule = match args.named.get("fill-rule") {
        None => FillRule::NonZero,
        Some(Value::Str(value)) if value.as_str() == "non-zero" => FillRule::NonZero,
        Some(Value::Str(value)) if value.as_str() == "even-odd" => FillRule::EvenOdd,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("polygon(fill-rule): valor inesperado {}", other.type_name()),
            )]);
        }
    };
    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "polygon"))
        .transpose()?;

    // P732 — fallback determinístico (paridade vanilla `Smart::Auto`,
    // lab/typst-original/crates/typst-layout/src/shapes.rs:336-339),
    // idêntico ao de `native_curve` (P727): sem fill nem stroke → stroke
    // preta de 1pt; com fill sem stroke → sem stroke. Sem ele o polígono
    // renderiza página em branco (path no PDF sem operador de pintura —
    // medido: 0 px não-brancos no cristalino vs 898 px no vanilla).
    let stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    // P277 — DEBT-33 CLOSED: usar geometry::path_bbox para bbox.
    // Para LineTo-only paths preserva bit-exact min/max behavior.
    let (min_x, min_y, max_x, max_y) = crate::entities::geometry::path_bbox(&path_items);
    let width =
        if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
    let height =
        if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };

    Ok(Value::Content(Content::Shape(Arc::new(ShapeElem {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
        fill_rule,
    }))))
}

// ── P1473 — `polygon.regular(fill?, stroke?, size?, vertices?)` ─────────────

/// `polygon.regular(fill?, stroke?, size?, vertices?)` — constructor
/// qualificado que gera um polígono regular circunscrito num círculo de
/// diâmetro `size` (default `1em`), com `vertices` lados (default `3`, `u64`).
///
/// Algoritmo byte-a-byte da fonte pinada
/// (`lab/typst-original/crates/typst-library/src/visualize/polygon.rs:53-88`):
/// mesma fórmula de ângulo (offset `PI*(1/2 − 1/vertices)` + rotação
/// `(i/vertices)·2π`, equivalente a `Angle::from_ratio` = `ratio·TAU`), mesma
/// normalização de bbox por `horizontal_offset`/`vertical_offset` (fold com
/// acumulador inicial `(radius, radius)`, **não** o primeiro ponto). O loop
/// `0..=vertices` (N+1, inclusive) fecha a forma repetindo o primeiro vértice
/// no fim — mesmo comportamento do vanilla (medido: `vertices: 3` devolve 4
/// pares). Reutiliza `PathItem::MoveTo`/`LineTo`/`ClosePath`, `parse_paint`,
/// `parse_shape_stroke`, `path_bbox` e o fallback determinístico P732 já
/// usados por `native_polygon` (mesmo `Content::Shape(ShapeKind::Path(...))`),
/// sem criar `PolygonRegularElem` nem variante nova de `Content`.
///
/// **Decisão de implementação (P1473, Agente A):** `size` default `1em` não é
/// resolvível em tempo de eval — esta função, como `native_polygon`/
/// `native_curve`, não tem acesso a `engine.styles` (assinatura nativa de 4
/// argumentos, sem `Engine`). Resolve o default contra o font-size root do
/// crate (`entities::style_chain::StyleChain::default_chain()`: `size:
/// Some(11.0)`), mesmo valor usado como default em todo o cristalino quando
/// não há `engine` disponível. `size:` explícito aceita `Length`/`Float`/
/// `Int`; só a componente `.abs` de `Length` é usada (paridade com
/// `vertex_component`/`native_rect` — `em` explícito não resolve em tempo de
/// eval, mesma limitação pré-existente).
pub fn native_polygon_regular(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Fonte pinada: `regular` não tem posicionais — qualquer positional é erro.
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }
    for key in args.named.keys() {
        if !matches!(key.as_str(), "fill" | "stroke" | "size" | "vertices") {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("unexpected argument: {key}"),
            )]);
        }
    }

    // Default root font-size do crate quando `engine.styles` não está
    // disponível (ver nota de decisão acima) — `entities/style_chain.rs`
    // `StyleChain::default_chain()`.
    const DEFAULT_ROOT_SIZE_PT: f64 = 11.0;

    let size_pt = match args.named.get("size") {
        None => DEFAULT_ROOT_SIZE_PT,
        Some(Value::Length(l)) => l.abs.to_pt(),
        Some(Value::Float(f)) => *f,
        Some(Value::Int(i)) => *i as f64,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected length, found {}", other.type_name()),
            )]);
        }
    };

    let vertices: u64 = match args.named.get("vertices") {
        None => 3,
        Some(Value::Int(n)) if *n < 0 => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "number must be at least zero".to_string(),
            )]);
        }
        Some(Value::Int(n)) => *n as u64,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected integer, found {}", other.type_name()),
            )]);
        }
    };

    let radius = size_pt / 2.0;
    let angle = |i: f64| -> f64 {
        let offset = std::f64::consts::PI * (0.5 - 1.0 / vertices as f64);
        let rotation = (i / vertices as f64) * std::f64::consts::TAU;
        offset + rotation
    };

    let (horizontal_offset, vertical_offset) = (0..=vertices)
        .map(|v| {
            let a = angle(v as f64);
            ((radius * libm::cos(a)) + radius, (radius * libm::sin(a)) + radius)
        })
        .fold((radius, radius), |(min_x, min_y), (v_x, v_y)| {
            (min_x.min(v_x), min_y.min(v_y))
        });

    let points: Vec<(f64, f64)> = (0..=vertices)
        .map(|v| {
            let a = angle(v as f64);
            let x = (radius * libm::cos(a)) + radius - horizontal_offset;
            let y = (radius * libm::sin(a)) + radius - vertical_offset;
            (x, y)
        })
        .collect();

    let mut path_items: Vec<PathItem> = Vec::with_capacity(points.len() + 1);
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            path_items.push(PathItem::MoveTo(Point { x: Pt(*x), y: Pt(*y) }));
        } else {
            path_items.push(PathItem::LineTo(Point { x: Pt(*x), y: Pt(*y) }));
        }
    }
    path_items.push(PathItem::ClosePath);

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();
    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "polygon.regular"))
        .transpose()?;

    // P732 — mesmo fallback determinístico de `native_polygon`: sem fill nem
    // stroke → stroke preta de 1pt; com fill sem stroke → sem stroke.
    let stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    let (min_x, min_y, max_x, max_y) = crate::entities::geometry::path_bbox(&path_items);
    let width =
        if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
    let height =
        if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };

    Ok(Value::Content(Content::Shape(Arc::new(ShapeElem {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
        fill_rule: FillRule::NonZero,
    }))))
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

/// Converte segmentos de `CurveElem` para `PathItem` absolutos.
///
/// P513: componente `em` em `Length` não é resolvível em tempo de eval
/// (sem font-size). Usa-se apenas a componente absoluta `abs` — paridade
/// vanilla para coordenadas puramente absolutas; `em` é scope-out deste
/// passo (pode ser adicionado quando `CurveElem` passar a transportar
/// font-size ou quando `native_curve` for convertido para layout-time).
fn close_path(path_items: &mut Vec<PathItem>, last_point: &mut Point, mode: CloseMode) {
    let Some(move_index) = path_items
        .iter()
        .rposition(|item| matches!(item, PathItem::MoveTo(_)))
    else {
        return;
    };
    let PathItem::MoveTo(start) = path_items[move_index] else { unreachable!() };
    let segments = &path_items[move_index + 1..];
    if segments.is_empty() {
        return;
    }
    if mode == CloseMode::Smooth {
        let start_control = match segments.first() {
            Some(PathItem::CubicTo(c1, _, _)) => Point {
                x: Pt(2.0 * start.x.0 - c1.x.0),
                y: Pt(2.0 * start.y.0 - c1.y.0),
            },
            _ => start,
        };
        let last_control = match segments.last() {
            Some(PathItem::CubicTo(_, c2, end)) => Point {
                x: Pt(2.0 * end.x.0 - c2.x.0),
                y: Pt(2.0 * end.y.0 - c2.y.0),
            },
            _ => *last_point,
        };
        path_items.push(PathItem::CubicTo(last_control, start_control, start));
    }
    path_items.push(PathItem::ClosePath);
    *last_point = start;
}

fn curve_segments_to_path_items(
    segments: &[CurveSegment],
    last_point: &mut Point,
    path_items: &mut Vec<PathItem>,
) {
    for seg in segments {
        match seg {
            CurveSegment::Move(p) => {
                let target = Point { x: Pt(p.x.abs.to_pt()), y: Pt(p.y.abs.to_pt()) };
                path_items.push(PathItem::MoveTo(target));
                *last_point = target;
            }
            CurveSegment::Line(p) => {
                let target = Point { x: Pt(p.x.abs.to_pt()), y: Pt(p.y.abs.to_pt()) };
                path_items.push(PathItem::LineTo(target));
                *last_point = target;
            }
            CurveSegment::Cubic(c1, c2, end) => {
                let target = Point { x: Pt(end.x.abs.to_pt()), y: Pt(end.y.abs.to_pt()) };
                path_items.push(PathItem::CubicTo(
                    Point { x: Pt(c1.x.abs.to_pt()), y: Pt(c1.y.abs.to_pt()) },
                    Point { x: Pt(c2.x.abs.to_pt()), y: Pt(c2.y.abs.to_pt()) },
                    target,
                ));
                *last_point = target;
            }
            CurveSegment::Quad(c, end) => {
                let p0x = last_point.x.0;
                let p0y = last_point.y.0;
                let qx = c.x.abs.to_pt();
                let qy = c.y.abs.to_pt();
                let ex = end.x.abs.to_pt();
                let ey = end.y.abs.to_pt();
                let c1x = (p0x + 2.0 * qx) / 3.0;
                let c1y = (p0y + 2.0 * qy) / 3.0;
                let c2x = (ex + 2.0 * qx) / 3.0;
                let c2y = (ey + 2.0 * qy) / 3.0;
                let target = Point { x: Pt(ex), y: Pt(ey) };
                path_items.push(PathItem::CubicTo(
                    Point { x: Pt(c1x), y: Pt(c1y) },
                    Point { x: Pt(c2x), y: Pt(c2y) },
                    target,
                ));
                *last_point = target;
            }
            CurveSegment::Close(mode) => {
                close_path(path_items, last_point, *mode);
            }
        }
    }
}

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
/// P513: também aceita `Content::Curve(...)` como argumentos posicionais,
/// concatenando os seus segmentos ao path final.
pub fn native_curve(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();
    // P294: tracking de last_point para conversão q→c em "quadratic"
    // (paridade vanilla `Curve::last_point`). Arranca em (0,0) — caso
    // fronteira de quadratic sem move anterior é tratado consistentemente.
    let mut last_point: Point = Point::ZERO;

    for (i, val) in args.items.iter().enumerate() {
        if let Value::Content(Content::Curve(e)) = val {
            curve_segments_to_path_items(&e.segments, &mut last_point, &mut path_items);
            continue;
        }

        // P803 — erro de tipo paridade vanilla: argumento que não é
        // `Content::Curve` nem tuplo legado (array com 1º elemento string)
        // → `expected content, found {type}` (vanilla medido:
        // `#curve((0pt, 0pt))` → "expected content, found array").
        let arr = match val {
            Value::Array(a)
                if matches!((a.is_empty(), a.first()), (false, Some(Value::Str(_)))) =>
            {
                a
            }
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected content, found {}", val.type_name()),
                )])
            }
        };

        let kind = match &arr[0] {
            Value::Str(s) => s.as_str(),
            _ => unreachable!("guard acima garante arr[0] string"),
        };

        match kind {
            "move" => {
                if arr.len() != 2 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'move' requer 1 coordenada, segmento {}", i),
                    )]);
                }
                let (x, y) = extract_coordinate(&arr[1]).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'move' segmento {}: coordenada inválida", i),
                    )]
                })?;
                let target = Point { x: Pt(x), y: Pt(y) };
                path_items.push(PathItem::MoveTo(target));
                last_point = target;
            }
            "line" => {
                if arr.len() != 2 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'line' requer 1 coordenada, segmento {}", i),
                    )]);
                }
                let (x, y) = extract_coordinate(&arr[1]).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'line' segmento {}: coordenada inválida", i),
                    )]
                })?;
                let target = Point { x: Pt(x), y: Pt(y) };
                path_items.push(PathItem::LineTo(target));
                last_point = target;
            }
            "cubic" => {
                if arr.len() != 4 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'cubic' requer 3 coordenadas (c1, c2, end), segmento {}", i),
                    )]);
                }
                let (c1x, c1y) = extract_coordinate(&arr[1]).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'cubic' segmento {}: c1 inválida", i),
                    )]
                })?;
                let (c2x, c2y) = extract_coordinate(&arr[2]).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'cubic' segmento {}: c2 inválida", i),
                    )]
                })?;
                let (ex, ey) = extract_coordinate(&arr[3]).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'cubic' segmento {}: end inválida", i),
                    )]
                })?;
                let end = Point { x: Pt(ex), y: Pt(ey) };
                path_items.push(PathItem::CubicTo(
                    Point { x: Pt(c1x), y: Pt(c1y) },
                    Point { x: Pt(c2x), y: Pt(c2y) },
                    end,
                ));
                last_point = end;
            }
            "close" => {
                // O tuple legado P293 representa apenas `ClosePath`; o
                // default Smooth de P1226 pertence a `curve.close()`.
                close_path(&mut path_items, &mut last_point, CloseMode::Straight);
            }
            // P294 H1' (descoberta empírica A.0.0 N=2): paridade vanilla
            // — converte quadratic→cubic em construct-time, sem variant
            // novo. Fórmula `control_q2c(p, c) = (p + 2c) / 3` aplicada a
            // start e end (`lab/.../typst-layout/src/shapes.rs:215-220`).
            "quadratic" => {
                if arr.len() != 3 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'quadratic' requer 2 coordenadas (control, end), segmento {}", i),
                    )]);
                }
                let (qx, qy) = extract_coordinate(&arr[1]).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'quadratic' segmento {}: control inválido", i),
                    )]
                })?;
                let (ex, ey) = extract_coordinate(&arr[2]).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("curve() 'quadratic' segmento {}: end inválido", i),
                    )]
                })?;
                let p0x = last_point.x.0;
                let p0y = last_point.y.0;
                let c1x = (p0x + 2.0 * qx) / 3.0;
                let c1y = (p0y + 2.0 * qy) / 3.0;
                let c2x = (ex + 2.0 * qx) / 3.0;
                let c2y = (ey + 2.0 * qy) / 3.0;
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
                    args.span,
                    format!("curve() kind '{}' desconhecido (esperado: move/line/cubic/close)", other),
                )]);
            }
        }
    }

    if path_items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "curve() requer pelo menos um segmento".to_string(),
        )]);
    }

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();
    let fill_rule = match args.named.get("fill-rule") {
        None => FillRule::NonZero,
        Some(Value::Str(value)) if value.as_str() == "non-zero" => FillRule::NonZero,
        Some(Value::Str(value)) if value.as_str() == "even-odd" => FillRule::EvenOdd,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("curve(fill-rule): valor inesperado {}", other.type_name()),
            )]);
        }
    };
    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "curve"))
        .transpose()?;

    // P727 — fallback determinístico (paridade vanilla `Smart::Auto`,
    // lab/typst-original/crates/typst-layout/src/shapes.rs:126-129):
    // sem fill nem stroke → stroke preta de 1pt; com fill sem stroke →
    // sem stroke. Idêntico ao fallback de `native_rect` — sem ele a
    // curva renderiza página em branco (path no PDF sem pintura).
    let stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    // P277 — `path_bbox` analítica reusada sem alteração.
    let (min_x, min_y, max_x, max_y) = crate::entities::geometry::path_bbox(&path_items);
    let width =
        if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
    let height =
        if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };

    Ok(Value::Content(Content::Shape(Arc::new(ShapeElem {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
        fill_rule,
    }))))
}

/// `curve.move(point)` → `Content::Curve` com segmento `Move`.
pub fn native_curve_move(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "curve.move() requer um ponto".to_string(),
        )]);
    }
    let p = extract_curve_point(&args.items[0], "curve.move", "point")?;
    Ok(Value::Content(Content::curve_move(p.x, p.y)))
}

/// `curve.line(point)` → `Content::Curve` com segmento `Line`.
pub fn native_curve_line(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "curve.line() requer um ponto".to_string(),
        )]);
    }
    let p = extract_curve_point(&args.items[0], "curve.line", "point")?;
    Ok(Value::Content(Content::curve_line(p.x, p.y)))
}

/// `curve.cubic(c1, c2, end)` → `Content::Curve` com segmento `Cubic`.
pub fn native_curve_cubic(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.items.len() != 3 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "curve.cubic() requer 3 pontos (control1, control2, end)".to_string(),
        )]);
    }
    let c1 = extract_curve_point(&args.items[0], "curve.cubic", "control1")?;
    let c2 = extract_curve_point(&args.items[1], "curve.cubic", "control2")?;
    let e = extract_curve_point(&args.items[2], "curve.cubic", "end")?;
    Ok(Value::Content(Content::curve_cubic(c1.x, c1.y, c2.x, c2.y, e.x, e.y)))
}

/// `curve.quad(control, end)` → `Content::Curve` com segmento `Quad`.
pub fn native_curve_quad(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.items.len() != 2 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "curve.quad() requer 2 pontos (control, end)".to_string(),
        )]);
    }
    let c = extract_curve_point(&args.items[0], "curve.quad", "control")?;
    let e = extract_curve_point(&args.items[1], "curve.quad", "end")?;
    Ok(Value::Content(Content::curve_quad(c.x, c.y, e.x, e.y)))
}

/// `curve.close()` → `Content::Curve` com segmento `Close`.
pub fn native_curve_close(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "curve.close() não aceita argumentos".to_string(),
        )]);
    }
    let mode = match args.named.get("mode") {
        None => CloseMode::Smooth,
        Some(Value::Str(value)) if value.as_str() == "smooth" => CloseMode::Smooth,
        Some(Value::Str(value)) if value.as_str() == "straight" => CloseMode::Straight,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("curve.close(mode): valor inesperado {}", other.type_name()),
            )]);
        }
    };
    Ok(Value::Content(Content::curve_close(mode)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::{Angle, Length};
    use indexmap::IndexMap;

    #[test]
    fn p1455_parse_color_rejeita_catalogo_string_hardcoded() {
        for name in [
            "red", "green", "blue", "black", "white", "yellow", "cyan", "magenta",
            "orange", "purple", "gray", "grey", "silver", "maroon", "navy", "olive",
            "teal", "lime", "aqua",
        ] {
            assert_eq!(parse_color(&Value::Str(name.into())), None, "string {name}");
        }
        let typed = Color::rgb(1, 2, 3);
        assert_eq!(parse_color(&Value::Color(typed.clone())), Some(typed));
    }

    fn ctx() -> EvalContext {
        EvalContext::new()
    }
    fn tfid() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            tfid()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    // ── P1043: Pares de independência testcase() para shapes.rs:395 e shapes.rs:791 ─

    #[test]
    fn p1043_line_length_only_isolada() {
        // 395 - C1=T, C2=F: args.named contains length && not angle -> length/angle branch
        let mut c = ctx();
        let mut named = IndexMap::default();
        named.insert("length".into(), Value::Length(Length::pt(20.0)));
        let args = Args {
            occurrences: None,
            items: vec![],
            named,
            span: Span::detached(),
        };
        let res = native_line(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_line_angle_only_isolada() {
        // 395 - C1=F, C2=T: args.named contains angle && not length -> length/angle branch
        let mut c = ctx();
        let mut named = IndexMap::default();
        named.insert("angle".into(), Value::Angle(Angle::deg(90.0)));
        let args = Args {
            occurrences: None,
            items: vec![],
            named,
            span: Span::detached(),
        };
        let res = native_line(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_line_default_end_isolada() {
        // 395 - C1=F, C2=F: args.named has neither length nor angle -> default end branch
        let mut c = ctx();
        let args = Args {
            occurrences: None,
            items: vec![],
            named: IndexMap::default(),
            span: Span::detached(),
        };
        let res = native_line(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_curve_array_valid_cmd_isolada() {
        // 791 - C1=T, C2=T: !a.is_empty() && matches!(a[0], Value::Str(_)) -> processa comando
        let mut c = ctx();
        let args = Args::positional(vec![Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Length(Length::ZERO), Value::Length(Length::ZERO)]),
        ])]);
        let res = native_curve(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_curve_array_empty_isolada() {
        // 791 - C1=F, C2=_: a.is_empty() -> Err(expected content, found array)
        let mut c = ctx();
        let args = Args::positional(vec![Value::Array(vec![])]);
        let res = native_curve(&mut c, &args, &NullWorld::default(), tfid());
        assert!(res.is_err());
    }

    #[test]
    fn p1043_curve_array_non_str_first_isolada() {
        // 791 - C1=T, C2=F: !a.is_empty() && !matches!(a[0], Str) -> Err(expected content, found array)
        let mut c = ctx();
        let args = Args::positional(vec![Value::Array(vec![Value::Int(10)])]);
        let res = native_curve(&mut c, &args, &NullWorld::default(), tfid());
        assert!(res.is_err());
    }
}

/// P1473 (Agente B) — contrato RED de `polygon.regular`. Fonte:
/// `00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/compiler/stdlib/shapes.md` §P1473. Escrito sem ver a
/// implementação (isolamento Agente A/B) — usa `eval_expression` (avaliação
/// de expressões Typst) como oráculo de observáveis de língua; a fórmula
/// geométrica é reproduzida aqui em `f64` puro (transcrita byte-a-byte do
/// L0, que por sua vez transcreve `visualize/polygon.rs:53-88` da fonte
/// pinada), não importada da implementação ainda inexistente.
#[cfg(test)]
mod p1473_tests {
    use super::*;
    use crate::compiler::eval::eval_expression;
    use crate::entities::source_result::SourceResult;

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1473).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    fn eval(src: &str) -> SourceResult<Value> {
        eval_expression(&NullWorld::default(), src).0
    }

    fn expect_ok(src: &str) -> Value {
        match eval(src) {
            Ok(v) => v,
            Err(errors) => panic!("{src}: esperava Ok, obteve Err({errors:?})"),
        }
    }

    fn expect_err_message(src: &str) -> String {
        match eval(src) {
            Err(errors) => {
                assert_eq!(
                    errors.len(),
                    1,
                    "{src}: esperava exactamente 1 diagnóstico, obteve {errors:?}"
                );
                errors[0].message.clone()
            }
            Ok(v) => panic!("{src}: esperava Err, obteve Ok({v:?})"),
        }
    }

    /// Extrai os pontos (`MoveTo`/`LineTo`, em pt) de um `Content::Shape`
    /// com `ShapeKind::Path` — ignora `ClosePath`/`CubicTo` (não usados por
    /// `polygon.regular`). Devolve `None` se o valor não é a forma esperada.
    fn extract_path_points(value: &Value) -> Option<Vec<(f64, f64)>> {
        let Value::Content(Content::Shape(elem)) = value else {
            return None;
        };
        let ShapeKind::Path(items) = &elem.kind else {
            return None;
        };
        Some(
            items
                .iter()
                .filter_map(|item| match item {
                    PathItem::MoveTo(p) | PathItem::LineTo(p) => {
                        Some((p.x.val(), p.y.val()))
                    }
                    _ => None,
                })
                .collect(),
        )
    }

    /// Fórmula pinada (transcrita do L0, que transcreve
    /// `visualize/polygon.rs:53-88` byte-a-byte): `vertices+1` pontos
    /// (`0..=vertices` inclusive fecha a forma repetindo o primeiro no
    /// fim), normalizados pelo mínimo `x`/`y` observado (incluindo o
    /// candidato-semente `radius` do fold do vanilla).
    fn regular_polygon_points(size_pt: f64, vertices: u64) -> Vec<(f64, f64)> {
        let radius = size_pt / 2.0;
        let angle = |v: u64| -> f64 {
            let offset = std::f64::consts::PI * (0.5 - 1.0 / vertices as f64);
            let rotation = 2.0 * std::f64::consts::PI * (v as f64 / vertices as f64);
            offset + rotation
        };
        let raw: Vec<(f64, f64)> = (0..=vertices)
            .map(|v| {
                let a = angle(v);
                (radius * libm::cos(a) + radius, radius * libm::sin(a) + radius)
            })
            .collect();
        let (h_off, v_off) = raw
            .iter()
            .fold((radius, radius), |(mnx, mny), &(x, y)| (mnx.min(x), mny.min(y)));
        raw.iter().map(|&(x, y)| (x - h_off, y - v_off)).collect()
    }

    fn assert_points_close(actual: &[(f64, f64)], expected: &[(f64, f64)]) {
        assert_eq!(actual.len(), expected.len(), "contagem de pontos diverge");
        for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (a.0 - e.0).abs() < 1e-6 && (a.1 - e.1).abs() < 1e-6,
                "vértice {i}: actual={a:?} expected={e:?}"
            );
        }
    }

    #[test]
    fn p1473_polygon_regular_type_is_function() {
        assert_eq!(expect_ok("type(polygon.regular) == function"), Value::Bool(true));
    }

    #[test]
    fn p1473_polygon_regular_defaults_close_the_shape_with_vertices_plus_one_points() {
        // size default 1em (simbólico — sem dimensão de referência em
        // tempo de eval, não comparável a pt aqui); vertices default 3 —
        // o loop 0..=vertices fecha a forma repetindo o primeiro vértice
        // (medido no L0: 4 pares, o último igual ao primeiro).
        let value = expect_ok("polygon.regular()");
        let points = extract_path_points(&value)
            .unwrap_or_else(|| panic!("esperava Content::Shape(Path), obteve {value:?}"));
        assert_eq!(points.len(), 4, "vertices(3)+1 pontos");
        assert_points_close(&[points[0]], &[points[3]]);
    }

    #[test]
    fn p1473_polygon_regular_explicit_args_match_pinned_formula() {
        let value = expect_ok("polygon.regular(size: 100pt, vertices: 5)");
        let points = extract_path_points(&value)
            .unwrap_or_else(|| panic!("esperava Content::Shape(Path), obteve {value:?}"));
        let expected = regular_polygon_points(100.0, 5);
        assert_points_close(&points, &expected);
        assert_points_close(&[points[0]], &[points[5]]);
    }

    #[test]
    fn p1473_polygon_regular_unknown_named() {
        assert_eq!(
            expect_err_message("polygon.regular(foo: 1)"),
            "unexpected argument: foo"
        );
    }

    #[test]
    fn p1473_polygon_regular_unexpected_positional() {
        assert_eq!(expect_err_message("polygon.regular(1)"), "unexpected argument");
    }

    #[test]
    fn p1473_polygon_regular_negative_vertices() {
        assert_eq!(
            expect_err_message("polygon.regular(vertices: -1)"),
            "number must be at least zero"
        );
    }

    #[test]
    fn p1473_polygon_regular_float_vertices() {
        assert_eq!(
            expect_err_message("polygon.regular(vertices: 2.5)"),
            "expected integer, found float"
        );
    }

    #[test]
    fn p1473_polygon_parent_keeps_function_kind_and_direct_call() {
        assert_eq!(expect_ok("type(polygon) == function"), Value::Bool(true));
        eval("polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))")
            .unwrap_or_else(|e| panic!("chamada directa de polygon() regrediu: {e:?}"));
    }
}
