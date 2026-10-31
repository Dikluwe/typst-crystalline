//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/curve.md
//! @prompt-hash 06a19a8c
//! @layer L1
//! @updated 2026-09-30

use super::*;

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
            thickness: crate::entities::layout_types::Length::pt(1.0),
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
        origin: crate::entities::elements::shape::ShapeOrigin::Geometry,
        rect_radius: None,
        rect_stroke: None,
        relative_line: None,
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
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        let point = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert!(matches!(
            call(native_curve_move, Args::positional(vec![point])).unwrap(),
            Value::Content(Content::Curve(_))
        ));
    }
}
