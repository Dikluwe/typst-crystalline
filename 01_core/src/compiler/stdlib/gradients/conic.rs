//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/gradients/conic.md
//! @prompt-hash c951d322
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `gradient.conic(stops..., center: ?, angle: ?)` →
/// `Value::Gradient(Gradient::Conic)` per ADR-0089 P267.
///
/// Stops parsing paridade `native_gradient_linear`/`radial`.
/// Aceita `Color` directo (offset auto) ou `[Color, Ratio]`
/// array.
///
/// Named:
/// - `center: Array [Ratio, Ratio]` (default `(50%, 50%)`).
/// - `angle: Angle` (default `0deg`).
pub fn native_gradient_conic(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut stops = parse_stops(&args.items)?;
    if stops.len() < 2 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "a gradient must have at least two stops".to_string(),
        )]);
    }

    let center = match args.named.get("center") {
        Some(Value::Array(arr)) if arr.len() == 2 => {
            let x = parse_ratio(&arr[0], "gradient.conic", "center.x")?;
            let y = parse_ratio(&arr[1], "gradient.conic", "center.y")?;
            Axes::new(x, y)
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "gradient.conic(center): espera Array [Ratio, Ratio], recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Axes::new(Ratio(0.5), Ratio(0.5)),
    };

    let angle = match args.named.get("angle") {
        Some(Value::Angle(a)) => *a,
        Some(Value::Float(f)) => Angle::rad(*f),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "gradient.conic(angle): espera Angle ou Float, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Angle::rad(0.0),
    };

    // P270 — named arg `space`.
    let space = select_space(args, "gradient.conic", &stops)?;
    convert_stops_to_space(&mut stops, &space, args)?;

    // P273 — named arg `relative` cross-variant.
    let relative = parse_relative_named(args, "gradient.conic")?;

    for key in args.named.keys() {
        if key != "center" && key != "angle" && key != "space" && key != "relative" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("gradient.conic: argumento nomeado inesperado '{}' (esperado: center, angle, space, relative)", key),
            )]);
        }
    }

    use crate::entities::gradient::Conic;
    use std::sync::Arc;
    Ok(Value::Gradient(Gradient::Conic(Arc::new(Conic {
        stops: Arc::from(stops),
        center,
        angle,
        space,
        relative,
        anti_alias: true,
    }))))
}
