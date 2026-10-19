//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/gradients/radial.md
//! @prompt-hash 0211ff85
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `gradient.radial(stops..., center: ?, radius: ?)` →
/// `Value::Gradient(Gradient::Radial)` per ADR-0088 P264.
///
/// Stops parsing paridade `native_gradient_linear`. Aceita
/// `Color` directo (offset auto) ou `[Color, Ratio]` array.
///
/// Named:
/// - `center: Array [Ratio, Ratio]` (default `(50%, 50%)`).
/// - `radius: Ratio` (default `50%`).
pub fn native_gradient_radial(
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

    let center =
        match args.named.get("center") {
            Some(Value::Array(arr)) if arr.len() == 2 => {
                let x = parse_ratio(&arr[0], "gradient.radial", "center.x")?;
                let y = parse_ratio(&arr[1], "gradient.radial", "center.y")?;
                Axes::new(x, y)
            }
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(center): espera Array [Ratio, Ratio], recebeu {}",
                    other.type_name()),
        )])
            }
            None => Axes::new(Ratio(0.5), Ratio(0.5)),
        };

    let radius = match args.named.get("radius") {
        Some(Value::Ratio(r)) => *r,
        Some(Value::Float(f)) => Ratio(*f),
        Some(Value::Int(i)) => Ratio(*i as f64),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "gradient.radial(radius): espera Ratio/Float, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Ratio(0.5),
    };
    if radius.0 < 0.0 || radius.0 > 1.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(radius): {} fora de [0, 1]", radius.0),
        )]);
    }

    // P269 — named args focal_* (paridade vanilla RadialGradient).
    let focal_center = match args.named.get("focal-center") {
        Some(Value::Array(arr)) if arr.len() == 2 => {
            let x = parse_ratio(&arr[0], "gradient.radial", "focal_center.x")?;
            let y = parse_ratio(&arr[1], "gradient.radial", "focal_center.y")?;
            Axes::new(x, y)
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                "gradient.radial(focal_center): espera Array [Ratio, Ratio], recebeu {}",
                other.type_name()
            ),
            )])
        }
        None => center, // default vanilla: focal_center = center
    };

    let focal_radius = match args.named.get("focal-radius") {
        Some(Value::Ratio(r)) => *r,
        Some(Value::Float(f)) => Ratio(*f),
        Some(Value::Int(i)) => Ratio(*i as f64),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "gradient.radial(focal_radius): espera Ratio/Float, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Ratio(0.0), // default vanilla: focal_radius = 0%
    };

    // Validação vanilla §1: focal_radius > radius → erro.
    if focal_radius.0 > radius.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "gradient.radial(focal_radius): {} > radius {}",
                focal_radius.0, radius.0
            ),
        )]);
    }

    // Validação vanilla §2: focal circle deve estar dentro do outer circle.
    // dist(focal_center, center)² >= (radius - focal_radius)² → erro.
    let dx = focal_center.x.0 - center.x.0;
    let dy = focal_center.y.0 - center.y.0;
    let dist_sq = dx * dx + dy * dy;
    let max_dist = radius.0 - focal_radius.0;
    if dist_sq >= max_dist * max_dist {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "gradient.radial: focal circle deve estar dentro do outer circle".to_string(),
        )]);
    }

    // P270 — named arg `space`.
    let space = select_space(args, "gradient.radial", &stops)?;
    convert_stops_to_space(&mut stops, &space, args)?;

    for key in args.named.keys() {
        if key != "center"
            && key != "radius"
            && key != "focal-center"
            && key != "focal-radius"
            && key != "space"
            && key != "relative"
        {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("gradient.radial: argumento nomeado inesperado '{}' (esperado: center, radius, focal_center, focal_radius, space, relative)", key),
            )]);
        }
    }

    // P273 — named arg `relative` cross-variant.
    let relative = parse_relative_named(args, "gradient.radial")?;

    // P269 + P270 + P273 — construção full com focal_* + space + relative.
    use crate::entities::gradient::Radial;
    use std::sync::Arc;
    Ok(Value::Gradient(Gradient::Radial(Arc::new(Radial {
        stops: Arc::from(stops),
        center,
        radius,
        focal_center,
        focal_radius,
        space,
        relative,
        anti_alias: true,
    }))))
}
