//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/gradients/accessors.md
//! @prompt-hash 359a9b9b
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn gradient_error(message: impl Into<String>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), message.into())])
}

pub(super) fn gradient_self<'a>(
    args: &'a Args,
    name: &str,
) -> SourceResult<&'a Gradient> {
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected named argument",
        )]);
    }
    match args.items.first() {
        Some(Value::Gradient(g)) => Ok(g),
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected gradient, found {}", other.type_name()),
        )]),
        None => {
            Err(vec![SourceDiagnostic::error(Span::detached(), "missing argument: self")])
        }
    }
    .and_then(|g| {
        if args.items.len() == 1 {
            Ok(g)
        } else {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{name}: unexpected argument"),
            )])
        }
    })
}

macro_rules! gradient_accessor {
    ($fn_name:ident, $label:literal, $body:expr) => {
        pub(crate) fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: FileId,
        ) -> SourceResult<Value> {
            let gradient = gradient_self(args, $label)?;
            Ok(($body)(gradient))
        }
    };
}

gradient_accessor!(native_gradient_kind, "gradient.kind", |g: &Gradient| {
    gradient_type_field(match g {
        Gradient::Linear(_) => "linear",
        Gradient::Radial(_) => "radial",
        Gradient::Conic(_) => "conic",
    })
    .unwrap()
});

gradient_accessor!(native_gradient_stops, "gradient.stops", |g: &Gradient| {
    let (stops, offsets): (&[GradientStop], Vec<f64>) = match g {
        Gradient::Linear(v) => (&v.stops, v.effective_offsets_precise()),
        Gradient::Radial(v) => (&v.stops, v.effective_offsets_precise()),
        Gradient::Conic(v) => (&v.stops, v.effective_offsets_precise()),
    };
    Value::Array(
        stops
            .iter()
            .zip(offsets)
            .map(|(stop, offset)| {
                Value::Array(vec![
                    Value::Color(stop.color.clone()),
                    Value::Ratio(Ratio(offset)),
                ])
            })
            .collect(),
    )
});

gradient_accessor!(native_gradient_space, "gradient.space", |g: &Gradient| {
    let space = match g {
        Gradient::Linear(v) => v.space.clone(),
        Gradient::Radial(v) => v.space.clone(),
        Gradient::Conic(v) => v.space.clone(),
    };
    let name = match space {
        ColorSpace::Spot(colorant) => return Value::SpotColorant(colorant),
        ColorSpace::Srgb => "rgb",
        ColorSpace::Luma => "luma",
        ColorSpace::LinearRgb => "linear-rgb",
        ColorSpace::Oklab => "oklab",
        ColorSpace::Oklch => "oklch",
        ColorSpace::Cmyk => "cmyk",
        ColorSpace::Hsl => "hsl",
        ColorSpace::Hsv => "hsv",
    };
    crate::compiler::stdlib::color::color_type_field(name).unwrap()
});

gradient_accessor!(native_gradient_relative, "gradient.relative", |g: &Gradient| {
    let relative = match g {
        Gradient::Linear(v) => v.relative,
        Gradient::Radial(v) => v.relative,
        Gradient::Conic(v) => v.relative,
    };
    match relative {
        None => Value::Auto,
        Some(crate::entities::gradient::RelativeTo::Self_) => Value::Str("self".into()),
        Some(crate::entities::gradient::RelativeTo::Parent) => {
            Value::Str("parent".into())
        }
    }
});

gradient_accessor!(native_gradient_angle, "gradient.angle", |g: &Gradient| match g {
    Gradient::Linear(v) => Value::Angle(v.angle),
    Gradient::Radial(_) => Value::None,
    Gradient::Conic(v) => Value::Angle(v.angle),
});
gradient_accessor!(native_gradient_center, "gradient.center", |g: &Gradient| match g {
    Gradient::Linear(_) => Value::None,
    Gradient::Radial(v) =>
        Value::Array(vec![Value::Ratio(v.center.x), Value::Ratio(v.center.y)]),
    Gradient::Conic(v) =>
        Value::Array(vec![Value::Ratio(v.center.x), Value::Ratio(v.center.y)]),
});
gradient_accessor!(native_gradient_radius, "gradient.radius", |g: &Gradient| match g {
    Gradient::Radial(v) => Value::Ratio(v.radius),
    _ => Value::None,
});
gradient_accessor!(
    native_gradient_focal_center,
    "gradient.focal-center",
    |g: &Gradient| match g {
        Gradient::Radial(v) => Value::Array(vec![
            Value::Ratio(v.focal_center.x),
            Value::Ratio(v.focal_center.y)
        ]),
        _ => Value::None,
    }
);
gradient_accessor!(
    native_gradient_focal_radius,
    "gradient.focal-radius",
    |g: &Gradient| match g {
        Gradient::Radial(v) => Value::Ratio(v.focal_radius),
        _ => Value::None,
    }
);
