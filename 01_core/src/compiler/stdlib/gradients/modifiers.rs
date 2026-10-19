//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/gradients/modifiers.md
//! @prompt-hash 3f8aa234
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn rebuild_gradient(
    gradient: &Gradient,
    stops: Vec<GradientStop>,
    anti_alias: bool,
) -> Gradient {
    use crate::entities::gradient::{Conic, Linear, Radial};
    use std::sync::Arc;
    match gradient {
        Gradient::Linear(v) => Gradient::Linear(Arc::new(Linear {
            stops: Arc::from(stops),
            angle: v.angle,
            space: v.space.clone(),
            relative: v.relative,
            anti_alias,
        })),
        Gradient::Radial(v) => Gradient::Radial(Arc::new(Radial {
            stops: Arc::from(stops),
            center: v.center,
            radius: v.radius,
            focal_center: v.focal_center,
            focal_radius: v.focal_radius,
            space: v.space.clone(),
            relative: v.relative,
            anti_alias,
        })),
        Gradient::Conic(v) => Gradient::Conic(Arc::new(Conic {
            stops: Arc::from(stops),
            center: v.center,
            angle: v.angle,
            space: v.space.clone(),
            relative: v.relative,
            anti_alias,
        })),
    }
}

pub(crate) fn native_gradient_sharp(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.keys().any(|key| key != "smoothness") {
        return gradient_error("unexpected argument");
    }
    let [Value::Gradient(gradient), steps] = args.items.as_slice() else {
        return gradient_error("gradient.sharp expects self and steps");
    };
    let Value::Int(steps) = steps else {
        return gradient_error("expected integer, found non-integer");
    };
    if *steps < 2 {
        return gradient_error("sharp gradients must have at least two stops");
    }
    let smoothness = match args.named.get("smoothness") {
        None => 0.0,
        Some(Value::Ratio(v)) => v.0,
        Some(other) => {
            return gradient_error(format!("expected ratio, found {}", other.type_name()))
        }
    };
    if !(0.0..=1.0).contains(&smoothness) {
        return gradient_error("smoothness must be between 0 and 1");
    }
    let n = *steps as usize;
    let colors: Vec<_> = (0..n)
        .flat_map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let color = match gradient {
                Gradient::Linear(v) => v.sample(t),
                Gradient::Radial(v) => v.sample(t),
                Gradient::Conic(v) => v.sample(t),
            };
            [color.clone(), color]
        })
        .collect();
    let progress = smoothness / (4.0 * n as f64);
    let mut stops = Vec::with_capacity(n * 2);
    for (index, color) in colors.into_iter().enumerate() {
        let i = index / 2;
        let mut offset =
            if index % 2 == 0 { i as f64 / n as f64 } else { (i + 1) as f64 / n as f64 };
        if index % 2 == 0 && index > 0 {
            offset += progress;
        }
        if index % 2 == 1 && index + 1 < n * 2 {
            offset -= progress;
        }
        let stop = GradientStop::new(color, Ratio(offset));
        if stops.last() != Some(&stop) {
            stops.push(stop);
        }
    }
    Ok(Value::Gradient(rebuild_gradient(gradient, stops, false)))
}

pub(crate) fn native_gradient_repeat(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.keys().any(|key| key != "mirror") {
        return gradient_error("unexpected argument");
    }
    let [Value::Gradient(gradient), repetitions] = args.items.as_slice() else {
        return gradient_error("gradient.repeat expects self and repetitions");
    };
    let Value::Int(repetitions) = repetitions else {
        return gradient_error("expected integer, found non-integer");
    };
    if *repetitions < 1 {
        return gradient_error("must repeat at least once");
    }
    let mirror = match args.named.get("mirror") {
        None => false,
        Some(Value::Bool(v)) => *v,
        Some(other) => {
            return gradient_error(format!(
                "expected boolean, found {}",
                other.type_name()
            ))
        }
    };
    let source = resolved_stops(gradient);
    let n = *repetitions as usize;
    let mut stops = Vec::new();
    for i in 0..n {
        let mut part: Vec<_> = source
            .iter()
            .map(|stop| {
                let r = stop.offset.unwrap().0;
                let offset = if i % 2 == 1 && mirror {
                    (i as f64 + 1.0 - r) / n as f64
                } else {
                    (i as f64 + r) / n as f64
                };
                GradientStop::new(stop.color.clone(), Ratio(offset))
            })
            .collect();
        if i % 2 == 1 && mirror {
            part.reverse();
        }
        for stop in part {
            if stops.last() != Some(&stop) {
                stops.push(stop);
            }
        }
    }
    let anti_alias = match gradient {
        Gradient::Linear(v) => v.anti_alias,
        Gradient::Radial(v) => v.anti_alias,
        Gradient::Conic(v) => v.anti_alias,
    };
    Ok(Value::Gradient(rebuild_gradient(gradient, stops, anti_alias)))
}
