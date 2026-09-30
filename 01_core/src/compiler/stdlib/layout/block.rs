//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout/block.md
//! @prompt-hash 6be9203a
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::block`.

use super::*;

/// Coage `Value::Str` para `Parity` (`"even"` / `"odd"`).
/// Outros tipos ou strings → erro hard.
pub(crate) fn extract_parity(value: &Value) -> SourceResult<Parity> {
    match value {
        Value::Str(s) => match s.as_str() {
            "even" => Ok(Parity::Even),
            "odd" => Ok(Parity::Odd),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "pagebreak(to:) deve ser \"even\" ou \"odd\", recebeu \"{}\"",
                    other
                ),
            )]),
        },
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("pagebreak(to:) deve ser string, recebeu {}", other.type_name()),
        )]),
    }
}

/// `block(body, width: ?, height: ?, inset: ?, breakable: true)` →
/// `Content::Block`.
///
/// **Atributos** (subset Fase 1 per ADR-0054 graded):
/// - `body` posicional obrigatório (Content ou Str).
/// - `width: Length` ou `Float`/`Int` (interpretado em pt). Ausente == auto.
/// - `height` análogo.
/// - `inset: Length` (uniforme nos 4 lados; refino futuro para Sides
///   completo via dict).
/// - `breakable: bool` (default `true`).
///
/// **Scope-out** (refino futuro): outset, fill, stroke, radius, clip,
/// spacing, above/below, sticky.
pub fn native_block(
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
                "block() espera content ou string como primeiro argumento, recebeu {}",
                other.type_name()
            ),
            )])
        }
        // Body opcional em vanilla; aceitamos ausência como Empty.
        None => Content::Empty,
    };

    let mut width: Option<crate::entities::rel::Rel<Length>> = None;
    let mut height: Option<crate::entities::elements::block::BlockHeight> = None;
    let mut inset_val: Option<&Value> = None;
    let mut breakable: bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                width = Some(extract_block_width(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(width:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "height" => {
                height = Some(extract_block_height(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            // **P475** — inset: Length | Relative | Dict per-side.
            "inset" => { inset_val = Some(value); }
            "breakable" => match value {
                Value::Bool(b) => breakable = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(breakable:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            // P231 — aceitar outset/radius/clip (parsing pós-loop).
            // P247 — aceitar fill/stroke (parsing pós-loop paralelo).
            // P250 — aceitar spacing/above/below/sticky (parsing pós-loop).
            "outset" | "radius" | "clip" | "fill" | "stroke"
                | "spacing" | "above" | "below" | "sticky" => {},
            other => return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height negativos rejeitados.
    for (label, len) in [("width", width)]
        .into_iter()
        .filter_map(|(l, opt)| opt.map(|len| (l, len)))
    {
        if len.rel < 0.0 || len.abs.abs.0 < 0.0 || len.abs.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "block({}:): valor negativo não suportado neste passo (P156G)",
                    label
                ),
            )]);
        }
    }
    if let Some(value) = height {
        let valid = match value {
            crate::entities::elements::block::BlockHeight::Relative(value) => {
                value.rel >= 0.0 && value.abs.abs.0 >= 0.0 && value.abs.em >= 0.0
            }
            crate::entities::elements::block::BlockHeight::Fractional(weight) => {
                weight.is_finite() && weight >= 0.0
            }
        };
        if !valid {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "block(height:): valor negativo não suportado neste passo (P1373)",
            )]);
        }
    }

    // **P475** — inset: Length | Relative (abs) | Dict per-side.
    let inset = match inset_val {
        Some(val) => extract_sides_from_value(val, "block", "inset")?,
        None => Sides::uniform(Length::ZERO),
    };

    // P231 — outset. **P475** — aceita Dict per-side além de Length uniforme.
    let outset = match args.named.get("outset") {
        Some(val) => extract_sides_from_value(val, "block", "outset")?,
        None => Sides::uniform(Length::ZERO),
    };
    // P242 — radius `Corners<Length>` (refino face P231 `Option<Length>`).
    // Aceita Length uniforme OR Dict por canto via helper centralizado.
    let radius = match args.named.get("radius") {
        Some(val) => extract_corners_length_value(val, "block")?,
        None => crate::entities::corners::Corners::uniform(Length::ZERO),
    };
    let clip = match args.named.get("clip") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(clip): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P247 — fill: aceita Value::Color directo (paridade pattern Grid/
    // Table inline). Refino futuro para Paint enum quando ADR dedicada.
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };

    // P247 — stroke: reusa `extract_stroke` (helper pré-existente
    // P227 stdlib/layout.rs:351). Aceita Length/Color/Stroke shorthand.
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(extract_stroke(val, "block", "stroke")?),
    };

    // P250 — 4 named args novos: spacing/above/below (Length opcionais;
    // negativos rejeitados) + sticky (Bool). Defaults None×3 + false
    // preservam output literal pre-P250.
    let extract_block_length = |key: &str| -> SourceResult<Option<Length>> {
        match args.named.get(key) {
            None => Ok(None),
            Some(val) => {
                let len = extract_length(val).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "block({}:) espera length, recebeu {}",
                            key,
                            val.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("block({}:): valor negativo não suportado (P250)", key),
                    )]);
                }
                Ok(Some(len))
            }
        }
    };
    let spacing = extract_block_length("spacing")?;
    let above = extract_block_length("above")?;
    let below = extract_block_length("below")?;
    let sticky = match args.named.get("sticky") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(sticky): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    Ok(Value::Content(Content::Block(std::sync::Arc::new(
        crate::entities::elements::block::BlockElem {
            body,
            width,
            height,
            inset,
            breakable,
            outset,
            radius,
            clip,
            fill,
            stroke,
            spacing,
            above,
            below,
            sticky,
        },
    ))))
}
