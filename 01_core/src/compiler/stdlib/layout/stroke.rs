//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout/stroke.md
//! @prompt-hash 4b86ff75
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::stroke`.

use super::*;

pub(crate) fn is_valid_miter_limit(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// `stroke(positional?, paint: ?, thickness: ?)` → `Value::Stroke` — Passos 227/1499
/// (ADR-0079 PROPOSTO Categoria A.1 Fase 5 Layout candidata).
///
/// Forma: `#stroke(thickness: 2pt)`, `#stroke(paint: red, thickness: 1pt)`
/// ou um único posicional conversível em `Stroke` (ex.: `#stroke(2pt)`).
///
/// **Defaults paridade vanilla**:
/// - `paint: Color` (named opcional; default `Color::rgb(0, 0, 0)` BLACK).
/// - `thickness: Length` (named opcional; default 1.0 pt).
///
/// **Validações**:
/// - Um posicional não pode ser misturado com named nem com outro posicional.
/// - No caminho named, `thickness > 0` (rejeita 0 e negativos).
/// - Named args restritos aos campos de stroke e à extensão `overhang`.
pub fn native_stroke(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::geometry::{
        DashLength, DashPattern, LineCap, LineJoin, Stroke, StrokeFields,
    };
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;

    if let Some(value) = args.items.first() {
        if args.items.len() != 1 || !args.named.is_empty() {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "stroke(): argumento inesperado".to_string(),
            )]);
        }
        return positional_stroke(value, args.span).map(Value::Stroke);
    }

    let paint = match args.named.get("paint") {
        Some(Value::Color(c)) => c.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke(paint): espera Color, recebeu {}", other.type_name()),
            )])
        }
        None => Color::rgb(0, 0, 0),
    };

    let thickness = match args.named.get("thickness") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "stroke(thickness): espera length, recebeu {}",
                        val.type_name()
                    ),
                )]
            })?;
            len.abs.to_pt()
        }
        None => 1.0,
    };

    let parse_name = |field: &str, choices: &[&str]| -> SourceResult<Option<&str>> {
        match args.named.get(field) {
            Some(Value::Str(value)) if choices.contains(&value.as_str()) => {
                Ok(Some(value.as_str()))
            }
            Some(Value::Str(value)) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke({field}): valor inesperado {:?}", value.as_str()),
            )]),
            Some(other) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke({field}): espera string, recebeu {}", other.type_name()),
            )]),
            None => Ok(None),
        }
    };
    let cap = match parse_name("cap", &["butt", "round", "square"])? {
        Some("round") => LineCap::Round,
        Some("square") => LineCap::Square,
        _ => LineCap::Butt,
    };
    let join = match parse_name("join", &["miter", "round", "bevel"])? {
        Some("round") => LineJoin::Round,
        Some("bevel") => LineJoin::Bevel,
        _ => LineJoin::Miter,
    };
    let miter_limit = match args.named.get("miter-limit") {
        Some(Value::Float(value)) if is_valid_miter_limit(*value) => *value,
        Some(Value::Int(value)) if *value > 0 => *value as f64,
        Some(Value::Float(_) | Value::Int(_)) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "stroke(miter-limit): deve ser finito e > 0".to_string(),
            )]);
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "stroke(miter-limit): espera número, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => 4.0,
    };

    fn dash_item(value: &Value) -> Option<DashLength> {
        match value {
            Value::Length(length) if length.abs.to_pt() >= 0.0 => {
                Some(DashLength::Length(length.abs.to_pt()))
            }
            Value::Str(value) if value.as_str() == "dot" => Some(DashLength::LineWidth),
            _ => None,
        }
    }
    fn dash_array(values: &[Value]) -> Option<Vec<DashLength>> {
        (!values.is_empty())
            .then(|| values.iter().map(dash_item).collect::<Option<Vec<_>>>())?
    }
    fn preset(name: &str) -> Option<Vec<DashLength>> {
        let l = DashLength::LineWidth;
        let pt = |value| DashLength::Length(value);
        Some(match name {
            "dotted" => vec![l, pt(2.0)],
            "densely-dotted" => vec![l, pt(1.0)],
            "loosely-dotted" => vec![l, pt(4.0)],
            "dashed" => vec![pt(3.0), pt(3.0)],
            "densely-dashed" => vec![pt(3.0), pt(2.0)],
            "loosely-dashed" => vec![pt(3.0), pt(6.0)],
            "dash-dotted" => vec![pt(3.0), pt(2.0), l, pt(2.0)],
            "densely-dash-dotted" => vec![pt(3.0), pt(1.0), l, pt(1.0)],
            "loosely-dash-dotted" => vec![pt(3.0), pt(4.0), l, pt(4.0)],
            _ => return None,
        })
    }
    let dash = match args.named.get("dash") {
        None | Some(Value::None) => None,
        Some(Value::Str(name)) => {
            preset(name.as_str()).map(|array| DashPattern { array, phase: 0.0 })
        }
        Some(Value::Array(values)) => {
            dash_array(values).map(|array| DashPattern { array, phase: 0.0 })
        }
        Some(Value::Dict(dict)) => {
            let array = match dict.get("array") {
                Some(Value::Array(values)) => dash_array(values),
                _ => None,
            };
            let phase = match dict.get("phase") {
                None => Some(0.0),
                Some(Value::Length(value)) => Some(value.abs.to_pt()),
                _ => None,
            };
            array.zip(phase).map(|(array, phase)| DashPattern { array, phase })
        }
        _ => None,
    };
    if args.named.contains_key("dash")
        && dash.is_none()
        && !matches!(args.named.get("dash"), Some(Value::None))
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "stroke(dash): espera preset, array ou dict (array:, phase:)".to_string(),
        )]);
    }

    if thickness <= 0.0 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("stroke(thickness): deve ser > 0 (recebeu {})", thickness),
        )]);
    }

    // P252 — overhang opcional; default vanilla `true` quando ausente.
    let overhang = match args.named.get("overhang") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke(overhang): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => true,
    };

    for key in args.named.keys() {
        if !["paint", "thickness", "cap", "join", "dash", "miter-limit", "overhang"]
            .contains(&key.as_str())
        {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke(): argumento nomeado inesperado '{}'", key),
            )]);
        }
    }

    Ok(Value::Stroke(Stroke {
        paint: Paint::Solid(paint),
        thickness,
        cap,
        join,
        dash,
        miter_limit,
        specified: StrokeFields {
            paint: args.named.contains_key("paint"),
            thickness: args.named.contains_key("thickness"),
            cap: args.named.contains_key("cap"),
            join: args.named.contains_key("join"),
            dash: args.named.contains_key("dash"),
            miter_limit: args.named.contains_key("miter-limit"),
        },
        overhang,
    }))
}

/// P1499 — conversão do único argumento posicional de stroke().
///
/// Diferentemente do caminho named legado, conserva os slots automáticos
/// da linguagem, aceita espessuras não positivas e valida a forma Dict
/// conforme a conversão Stroke do vanilla. O render decide separadamente
/// como tratar esses comprimentos.
pub(crate) fn positional_stroke(
    value: &Value,
    span: Span,
) -> SourceResult<crate::entities::geometry::Stroke> {
    use crate::entities::geometry::{DashLength, DashPattern, LineCap, LineJoin, Stroke};
    use crate::entities::paint::Paint;

    fn invalid(span: Span, field: &str, value: &Value) -> Vec<SourceDiagnostic> {
        vec![SourceDiagnostic::error(
            span,
            format!(
                "stroke({field}): tipo não conversível, recebeu {}",
                value.type_name()
            ),
        )]
    }

    fn dash_item(value: &Value, span: Span) -> SourceResult<DashLength> {
        match value {
            Value::Length(length) => Ok(DashLength::Length(length.abs.to_pt())),
            Value::Str(name) if name.as_str() == "dot" => Ok(DashLength::LineWidth),
            other => Err(invalid(span, "dash", other)),
        }
    }

    fn dash_array(values: &[Value], span: Span) -> SourceResult<Vec<DashLength>> {
        values.iter().map(|item| dash_item(item, span)).collect()
    }

    fn dash_preset(name: &str) -> Option<Vec<DashLength>> {
        let l = DashLength::LineWidth;
        let pt = DashLength::Length;
        Some(match name {
            "solid" => vec![],
            "dotted" => vec![l, pt(2.0)],
            "densely-dotted" => vec![l, pt(1.0)],
            "loosely-dotted" => vec![l, pt(4.0)],
            "dashed" => vec![pt(3.0), pt(3.0)],
            "densely-dashed" => vec![pt(3.0), pt(2.0)],
            "loosely-dashed" => vec![pt(3.0), pt(6.0)],
            "dash-dotted" => vec![pt(3.0), pt(2.0), l, pt(2.0)],
            "densely-dash-dotted" => vec![pt(3.0), pt(1.0), l, pt(1.0)],
            "loosely-dash-dotted" => vec![pt(3.0), pt(4.0), l, pt(4.0)],
            _ => return None,
        })
    }

    fn dash(value: &Value, span: Span) -> SourceResult<Option<DashPattern>> {
        let pattern = match value {
            Value::None => return Ok(None),
            Value::Str(name) => DashPattern {
                array: dash_preset(name.as_str())
                    .ok_or_else(|| invalid(span, "dash", value))?,
                phase: 0.0,
            },
            Value::Array(values) => {
                DashPattern { array: dash_array(values, span)?, phase: 0.0 }
            }
            Value::Dict(dict) => {
                for key in dict.keys() {
                    if !["array", "phase"].contains(&key.as_str()) {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("stroke(dash): chave inesperada '{key}'"),
                        )]);
                    }
                }
                let array = match dict.get("array") {
                    Some(Value::Array(values)) => dash_array(values, span)?,
                    Some(other) => return Err(invalid(span, "dash.array", other)),
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "stroke(dash): falta array".to_string(),
                        )]);
                    }
                };
                let phase = match dict.get("phase") {
                    Some(Value::Length(length)) => length.abs.to_pt(),
                    Some(other) => return Err(invalid(span, "dash.phase", other)),
                    None => 0.0,
                };
                DashPattern { array, phase }
            }
            other => return Err(invalid(span, "dash", other)),
        };
        Ok(Some(pattern))
    }

    if let Value::Stroke(stroke) = value {
        return Ok(stroke.clone());
    }

    let mut stroke = Stroke { overhang: true, ..Stroke::default() };
    match value {
        Value::Length(length) => {
            stroke.thickness = length.abs.to_pt();
            stroke.specified.thickness = true;
        }
        Value::Color(color) => {
            stroke.paint = Paint::Solid(color.clone());
            stroke.specified.paint = true;
        }
        Value::Gradient(gradient) => {
            stroke.paint = Paint::Gradient(gradient.clone());
            stroke.specified.paint = true;
        }
        Value::Tiling(tiling) => {
            stroke.paint = Paint::Tiling(tiling.as_ref().clone());
            stroke.specified.paint = true;
        }
        Value::Dict(dict) => {
            for key in dict.keys() {
                if !["paint", "thickness", "cap", "join", "dash", "miter-limit"]
                    .contains(&key.as_str())
                {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("stroke(): chave inesperada '{key}'"),
                    )]);
                }
            }
            match dict.get("paint") {
                None | Some(Value::Auto) => {}
                Some(Value::Color(color)) => {
                    stroke.paint = Paint::Solid(color.clone());
                    stroke.specified.paint = true;
                }
                Some(Value::Gradient(gradient)) => {
                    stroke.paint = Paint::Gradient(gradient.clone());
                    stroke.specified.paint = true;
                }
                Some(Value::Tiling(tiling)) => {
                    stroke.paint = Paint::Tiling(tiling.as_ref().clone());
                    stroke.specified.paint = true;
                }
                Some(other) => return Err(invalid(span, "paint", other)),
            }
            match dict.get("thickness") {
                None | Some(Value::Auto) => {}
                Some(Value::Length(length)) => {
                    stroke.thickness = length.abs.to_pt();
                    stroke.specified.thickness = true;
                }
                Some(other) => return Err(invalid(span, "thickness", other)),
            }
            match dict.get("cap") {
                None | Some(Value::Auto) => {}
                Some(Value::Str(name)) => {
                    stroke.cap = match name.as_str() {
                        "butt" => LineCap::Butt,
                        "round" => LineCap::Round,
                        "square" => LineCap::Square,
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!(
                                    "stroke(cap): valor inesperado {:?}",
                                    name.as_str()
                                ),
                            )])
                        }
                    };
                    stroke.specified.cap = true;
                }
                Some(other) => return Err(invalid(span, "cap", other)),
            }
            match dict.get("join") {
                None | Some(Value::Auto) => {}
                Some(Value::Str(name)) => {
                    stroke.join = match name.as_str() {
                        "miter" => LineJoin::Miter,
                        "round" => LineJoin::Round,
                        "bevel" => LineJoin::Bevel,
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!(
                                    "stroke(join): valor inesperado {:?}",
                                    name.as_str()
                                ),
                            )])
                        }
                    };
                    stroke.specified.join = true;
                }
                Some(other) => return Err(invalid(span, "join", other)),
            }
            match dict.get("dash") {
                None | Some(Value::Auto) => {}
                Some(value) => {
                    stroke.dash = dash(value, span)?;
                    stroke.specified.dash = true;
                }
            }
            match dict.get("miter-limit") {
                None | Some(Value::Auto) => {}
                Some(Value::Float(limit)) => {
                    stroke.miter_limit = *limit;
                    stroke.specified.miter_limit = true;
                }
                Some(Value::Int(limit)) => {
                    stroke.miter_limit = *limit as f64;
                    stroke.specified.miter_limit = true;
                }
                Some(other) => return Err(invalid(span, "miter-limit", other)),
            }
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "stroke(): tipo não conversível em Stroke: {}",
                    other.type_name()
                ),
            )]);
        }
    }
    Ok(stroke)
}
