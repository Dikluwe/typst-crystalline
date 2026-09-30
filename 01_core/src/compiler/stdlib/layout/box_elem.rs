//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout/box_elem.md
//! @prompt-hash 8fd88d68
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::box_elem`.

use super::*;

/// `box(body, width: ?, height: ?, inset: ?, baseline: ?)` →
/// `Content::Boxed`.
///
/// **Atributos** (subset Fase 2 per ADR-0054 graded; padrão variant
/// rico reusado de `block` em P156G):
/// - `body` posicional opcional (Content ou Str; ausente → Empty).
/// - `width: Length`/`Float`/`Int` (em pt). Ausente == content-based.
/// - `height` análogo. Ausente == auto.
/// - `inset: Length` uniforme (refino futuro para Sides via dict).
/// - `baseline: Length` ajuste vertical (default zero).
///
/// **Scope-out** (refino futuro): outset, fill, stroke, radius, clip,
/// stroke-overhang.
///
/// Distinção material face a `block`: posicionamento **inline** vs
/// structural.
pub fn native_box(
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
                args.span,
                format!(
                    "box() espera content ou string como primeiro argumento, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Content::Empty, // body opcional (vanilla aceita)
    };

    let mut width: Option<Length> = None;
    let mut width_fraction: Option<f64> = None;
    let mut height: Option<Length> = None;
    let mut inset_val: Option<&Value> = None;
    let mut baseline: Length = Length::ZERO;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                let (absolute, fractional) = extract_box_width(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "box(width:) espera length ou fraction, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                width = absolute;
                width_fraction = fractional;
            }
            "height" => {
                height = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("box(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            // **P475** — inset: Length | Relative | Dict per-side.
            "inset" => { inset_val = Some(value); }
            "baseline" => {
                baseline = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("box(baseline:) espera length, recebeu {}", value.type_name()),
                )])?;
            }
            // P231 — aceitar outset/radius/clip (parsing pós-loop paridade block).
            // P247 — aceitar fill/stroke paralelo block.
            "outset" | "radius" | "clip" | "fill" | "stroke" => {},
            other => return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height negativos rejeitados (baseline negativo ACEITE).
    for (label, len) in [("width", width), ("height", height)]
        .iter()
        .filter_map(|(l, opt)| opt.map(|len| (*l, len)))
        .collect::<Vec<_>>()
    {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "box({}:): valor negativo não suportado neste passo (P156H)",
                    label
                ),
            )]);
        }
    }

    // **P475** — inset: Length | Relative (abs) | Dict per-side.
    let inset = match inset_val {
        Some(val) => extract_sides_from_value(val, "box", "inset")?,
        None => Sides::uniform(Length::ZERO),
    };

    // P231 — outset. **P475** — aceita Dict per-side além de Length uniforme.
    let outset = match args.named.get("outset") {
        Some(val) => extract_sides_from_value(val, "box", "outset")?,
        None => Sides::uniform(Length::ZERO),
    };
    // P242 — radius `Corners<Length>` paralelo block. Aceita Length
    // uniforme OR Dict por canto via helper centralizado.
    let radius = match args.named.get("radius") {
        Some(val) => extract_corners_length_value(val, "box")?,
        None => crate::entities::corners::Corners::uniform(Length::ZERO),
    };
    let clip = match args.named.get("clip") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(clip): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P247 — fill paralelo block.
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };

    // P247 — stroke reusa `extract_stroke` paralelo block.
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(extract_stroke(val, "box", "stroke")?),
    };

    Ok(Value::Content(Content::Boxed(std::sync::Arc::new(
        crate::entities::elements::boxed::BoxedElem {
            body,
            width,
            width_fraction,
            height,
            inset,
            baseline,
            outset,
            radius,
            clip,
            fill,
            stroke,
        },
    ))))
}
