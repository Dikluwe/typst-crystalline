//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/gradients/sampling.md
//! @prompt-hash 0fdd0560
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn sample_gradient(gradient: &Gradient, value: &Value) -> SourceResult<Value> {
    let t = match value {
        Value::Ratio(r) => r.0,
        Value::Angle(a) => {
            a.to_rad().rem_euclid(std::f64::consts::TAU) / std::f64::consts::TAU
        }
        other => {
            return gradient_error(format!(
                "expected ratio or angle, found {}",
                other.type_name()
            ))
        }
    };
    Ok(Value::Color(match gradient {
        Gradient::Linear(v) => {
            let t = v
                .effective_offsets_precise()
                .into_iter()
                .zip(v.effective_offsets())
                .find_map(|(precise, public)| (t == f64::from(public)).then_some(precise))
                .unwrap_or(t);
            v.sample_precise(t)
        }
        Gradient::Radial(v) => {
            let t = v
                .effective_offsets_precise()
                .into_iter()
                .zip(v.effective_offsets())
                .find_map(|(precise, public)| (t == f64::from(public)).then_some(precise))
                .unwrap_or(t);
            v.sample_precise(t)
        }
        Gradient::Conic(v) => v.sample(t as f32),
    }))
}

pub(crate) fn native_gradient_sample(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return gradient_error("unexpected named argument");
    }
    match args.items.as_slice() {
        [Value::Gradient(g), t] => sample_gradient(g, t),
        [] | [Value::Gradient(_)] => gradient_error("missing argument: t"),
        [other, ..] if !matches!(other, Value::Gradient(_)) => {
            gradient_error(format!("expected gradient, found {}", other.type_name()))
        }
        _ => gradient_error("unexpected argument"),
    }
}

pub(crate) fn native_gradient_samples(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return gradient_error("unexpected named argument");
    }
    let Some(Value::Gradient(g)) = args.items.first() else {
        return match args.items.first() {
            Some(v) => {
                gradient_error(format!("expected gradient, found {}", v.type_name()))
            }
            None => gradient_error("missing argument: self"),
        };
    };
    let values = args.items[1..]
        .iter()
        .map(|t| sample_gradient(g, t))
        .collect::<SourceResult<Vec<_>>>()?;
    Ok(Value::Array(values))
}

pub(super) fn resolved_stops(gradient: &Gradient) -> Vec<GradientStop> {
    let (stops, offsets): (&[GradientStop], Vec<f64>) = match gradient {
        Gradient::Linear(v) => (&v.stops, v.effective_offsets_precise()),
        Gradient::Radial(v) => (&v.stops, v.effective_offsets_precise()),
        Gradient::Conic(v) => (&v.stops, v.effective_offsets_precise()),
    };
    stops
        .iter()
        .zip(offsets)
        .map(|(stop, offset)| GradientStop::new(stop.color.clone(), Ratio(offset)))
        .collect()
}
