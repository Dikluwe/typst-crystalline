//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/gradients/linear.md
//! @prompt-hash 880ba36e
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `gradient.linear(stops..., angle: ?)` → `Value::Gradient(Gradient::Linear)`.
pub fn native_gradient_linear(
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

    let angle = match args.named.get("angle") {
        Some(Value::Angle(a)) => *a,
        Some(Value::Float(f)) => Angle::rad(*f),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "gradient.linear(angle): espera Angle ou Float, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => match args.named.get("dir") {
            Some(Value::Dir(Dir::LTR)) | None => Angle::rad(0.0),
            Some(Value::Dir(Dir::RTL)) => Angle::deg(180.0),
            Some(Value::Dir(Dir::TTB)) => Angle::deg(90.0),
            Some(Value::Dir(Dir::BTT)) => Angle::deg(270.0),
            Some(other) => {
                return gradient_error(format!(
                    "expected direction, found {}",
                    other.type_name()
                ))
            }
        },
    };

    // P270 — named arg `space` (ADR-0091 EM VIGOR).
    let space = select_space(args, "gradient.linear", &stops)?;
    convert_stops_to_space(&mut stops, &space, args)?;

    // P273 — named arg `relative` cross-variant.
    let relative = parse_relative_named(args, "gradient.linear")?;

    for key in args.named.keys() {
        if key != "angle"
            && key != "space"
            && key != "relative"
            && (key != "dir" || args.named.contains_key("angle"))
        {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("gradient.linear: argumento nomeado inesperado '{}' (esperado: angle, space, relative)", key),
            )]);
        }
    }

    use crate::entities::gradient::Linear;
    use std::sync::Arc;
    Ok(Value::Gradient(Gradient::Linear(Arc::new(Linear {
        stops: Arc::from(stops),
        angle,
        space,
        relative,
        anti_alias: true,
    }))))
}
