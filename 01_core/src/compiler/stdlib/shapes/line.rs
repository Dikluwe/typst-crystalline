//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/shapes/line.md
//! @prompt-hash 9caf3baf
//! @layer L1
//! @updated 2026-09-30

use super::*;

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

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        assert!(matches!(
            call(native_line, Args::positional(vec![])).unwrap(),
            Value::Content(Content::Shape(_))
        ));
    }
}
