//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/gradients.md
//! @prompt-hash 98969965
//! @layer L1
//! @updated 2026-06-24
//!
//! Stdlib `gradient.linear` — P262 (ADR-0087 Gradient Linear-only).
//!
//! Constrói `Value::Gradient(Gradient::Linear)` a partir de stops
//! variadic positional + named `angle: Angle`.
//!
//! Aceita formatos de stop:
//! - `Color` directo (offset = None; auto-spacing).
//! - `Array [Color, Ratio]` (offset explícito).
//!
//! ColorSpace fixo Oklab (scope-out ADR-0087 — paridade vanilla
//! default). Interpolação L1 via `Linear::sample(t)`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::axes::Axes;
use crate::entities::color::ColorSpace;
use crate::entities::dir::Dir;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::gradient::{Gradient, GradientStop};
use crate::entities::layout_types::{Angle, Ratio};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Devolve o valor associado a `field` no tipo `gradient` (P736).
///
/// `gradient` é `Value::Type(Type::Gradient)` no scope global (paridade
/// vanilla — medido: `type(gradient)` → `type`); os fields resolvem-se
/// por field access em `Value::Type` (`eval/bindings.rs`), que delega
/// aqui. `None` para campo inexistente — o chamador emite
/// "type gradient does not contain field `<f>`" (verbatim vanilla).
///
/// Histórico: até P736 era `make_gradient_module() -> Value`
/// (`Value::Dict` com os 3 constructors; acesso via field access em Dict).
pub fn gradient_type_field(field: &str) -> Option<Value> {
    Some(match field {
        "linear" => Value::Func(Func::native("linear", native_gradient_linear)),
        // P264 — Radial activa per ADR-0088.
        "radial" => Value::Func(Func::native("radial", native_gradient_radial)),
        // P267 — Conic activa per ADR-0089 (cluster Gradient 3/3 completo).
        "conic" => Value::Func(Func::native("conic", native_gradient_conic)),
        "kind" => Value::Func(Func::native("kind", native_gradient_kind)),
        "stops" => Value::Func(Func::native("stops", native_gradient_stops)),
        "space" => Value::Func(Func::native("space", native_gradient_space)),
        "relative" => Value::Func(Func::native("relative", native_gradient_relative)),
        "angle" => Value::Func(Func::native("angle", native_gradient_angle)),
        "center" => Value::Func(Func::native("center", native_gradient_center)),
        "radius" => Value::Func(Func::native("radius", native_gradient_radius)),
        "focal-center" => {
            Value::Func(Func::native("focal-center", native_gradient_focal_center))
        }
        "focal-radius" => {
            Value::Func(Func::native("focal-radius", native_gradient_focal_radius))
        }
        "sample" => Value::Func(Func::native("sample", native_gradient_sample)),
        "samples" => Value::Func(Func::native("samples", native_gradient_samples)),
        "sharp" => Value::Func(Func::native("sharp", native_gradient_sharp)),
        "repeat" => Value::Func(Func::native("repeat", native_gradient_repeat)),
        _ => return None,
    })
}

pub fn is_gradient_instance_method(method: &str) -> bool {
    matches!(
        method,
        "kind"
            | "stops"
            | "space"
            | "relative"
            | "angle"
            | "center"
            | "radius"
            | "focal-center"
            | "focal-radius"
            | "sample"
            | "samples"
            | "sharp"
            | "repeat"
    )
}

pub(crate) fn dispatch_gradient_method(
    gradient: &Gradient,
    method: &str,
    mut args: Args,
    ctx: &mut EvalContext,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    args.items.insert(0, Value::Gradient(gradient.clone()));
    match method {
        "kind" => native_gradient_kind(ctx, &args, world, current_file),
        "stops" => native_gradient_stops(ctx, &args, world, current_file),
        "space" => native_gradient_space(ctx, &args, world, current_file),
        "relative" => native_gradient_relative(ctx, &args, world, current_file),
        "angle" => native_gradient_angle(ctx, &args, world, current_file),
        "center" => native_gradient_center(ctx, &args, world, current_file),
        "radius" => native_gradient_radius(ctx, &args, world, current_file),
        "focal-center" => native_gradient_focal_center(ctx, &args, world, current_file),
        "focal-radius" => native_gradient_focal_radius(ctx, &args, world, current_file),
        "sample" => native_gradient_sample(ctx, &args, world, current_file),
        "samples" => native_gradient_samples(ctx, &args, world, current_file),
        "sharp" => native_gradient_sharp(ctx, &args, world, current_file),
        "repeat" => native_gradient_repeat(ctx, &args, world, current_file),
        _ => unreachable!("filtrado por is_gradient_instance_method"),
    }
}

fn gradient_error(message: impl Into<String>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), message.into())])
}

fn gradient_self<'a>(args: &'a Args, name: &str) -> SourceResult<&'a Gradient> {
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
    let (stops, offsets): (&[GradientStop], Vec<f32>) = match g {
        Gradient::Linear(v) => (&v.stops, v.effective_offsets()),
        Gradient::Radial(v) => (&v.stops, v.effective_offsets()),
        Gradient::Conic(v) => (&v.stops, v.effective_offsets()),
    };
    Value::Array(
        stops
            .iter()
            .zip(offsets)
            .map(|(stop, offset)| {
                Value::Array(vec![
                    Value::Color(stop.color),
                    Value::Ratio(Ratio(offset as f64)),
                ])
            })
            .collect(),
    )
});

gradient_accessor!(native_gradient_space, "gradient.space", |g: &Gradient| {
    let space = match g {
        Gradient::Linear(v) => v.space,
        Gradient::Radial(v) => v.space,
        Gradient::Conic(v) => v.space,
    };
    let name = match space {
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

fn sample_gradient(gradient: &Gradient, value: &Value) -> SourceResult<Value> {
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
        Gradient::Linear(v) => v.sample_precise(t),
        Gradient::Radial(v) => v.sample(t as f32),
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

fn resolved_stops(gradient: &Gradient) -> Vec<GradientStop> {
    let (stops, offsets): (&[GradientStop], Vec<f32>) = match gradient {
        Gradient::Linear(v) => (&v.stops, v.effective_offsets()),
        Gradient::Radial(v) => (&v.stops, v.effective_offsets()),
        Gradient::Conic(v) => (&v.stops, v.effective_offsets()),
    };
    stops
        .iter()
        .zip(offsets)
        .map(|(stop, offset)| GradientStop::new(stop.color, Ratio(offset as f64)))
        .collect()
}

fn rebuild_gradient(
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
            space: v.space,
            relative: v.relative,
            anti_alias,
        })),
        Gradient::Radial(v) => Gradient::Radial(Arc::new(Radial {
            stops: Arc::from(stops),
            center: v.center,
            radius: v.radius,
            focal_center: v.focal_center,
            focal_radius: v.focal_radius,
            space: v.space,
            relative: v.relative,
            anti_alias,
        })),
        Gradient::Conic(v) => Gradient::Conic(Arc::new(Conic {
            stops: Arc::from(stops),
            center: v.center,
            angle: v.angle,
            space: v.space,
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
            [color, color]
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
        if stops.last().copied() != Some(stop) {
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
                GradientStop::new(stop.color, Ratio(offset))
            })
            .collect();
        if i % 2 == 1 && mirror {
            part.reverse();
        }
        for stop in part {
            if stops.last().copied() != Some(stop) {
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
    let space = parse_space_named(args, "gradient.linear")?;
    for stop in &mut stops {
        stop.color = stop.color.to_space(space);
    }

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

/// P273 — Parser do named arg `relative` cross-variant (ADR-0091 §"Anotação
/// cumulativa P273").
///
/// Aceita `Value::Str("self" | "parent" | "auto")`. Default (sem named arg)
/// = `None` (Auto = `Self_` resolved). ADR-0064 §Caso A (`Smart<T>` →
/// `Option<T>` cristalino).
fn parse_relative_named(
    args: &Args,
    fn_name: &str,
) -> SourceResult<Option<crate::entities::gradient::RelativeTo>> {
    use crate::entities::gradient::RelativeTo;
    match args.named.get("relative") {
        None => Ok(None),
        Some(Value::Str(s)) => match s.as_str() {
            "self"   => Ok(Some(RelativeTo::Self_)),
            "parent" => Ok(Some(RelativeTo::Parent)),
            "auto"   => Ok(None),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{fn_name}(relative): '{other}' inválido (esperado: self, parent, auto)"
                ),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{fn_name}(relative): espera Str, recebeu {}", other.type_name()),
        )]),
    }
}

/// P270 — Parser do named arg `space` cross-variant (ADR-0091 EM VIGOR).
///
/// Aceita `Value::Str("oklab" | "oklch" | "srgb" | "luma" | "linear-rgb"
/// | "hsl" | "hsv" | "cmyk")`. Default (sem named arg) = `ColorSpace::Oklab`
/// (preserva P262/P264/P267 behavior bit-exact).
fn parse_space_named(args: &Args, fn_name: &str) -> SourceResult<ColorSpace> {
    match args.named.get("space") {
        None => Ok(ColorSpace::Oklab),
        Some(Value::Auto) => Ok(ColorSpace::Oklab),
        Some(Value::Func(f)) => match f.name() {
            Some("rgb") => Ok(ColorSpace::Srgb),
            Some("luma") => Ok(ColorSpace::Luma),
            Some("cmyk") => Ok(ColorSpace::Cmyk),
            Some("oklab") => Ok(ColorSpace::Oklab),
            Some("oklch") => Ok(ColorSpace::Oklch),
            Some("linear-rgb") => Ok(ColorSpace::LinearRgb),
            Some("hsl") => Ok(ColorSpace::Hsl),
            Some("hsv") => Ok(ColorSpace::Hsv),
            _ => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{fn_name}(space): função '{:?}' não é um espaço de cor suportado", f.name()),
            )]),
        },
        Some(Value::Str(s)) => match s.as_str() {
            "oklab"      => Ok(ColorSpace::Oklab),
            "oklch"      => Ok(ColorSpace::Oklch),
            "srgb"       => Ok(ColorSpace::Srgb),
            "luma"       => Ok(ColorSpace::Luma),
            "linear-rgb" => Ok(ColorSpace::LinearRgb),
            "hsl"        => Ok(ColorSpace::Hsl),
            "hsv"        => Ok(ColorSpace::Hsv),
            "cmyk"       => Ok(ColorSpace::Cmyk),
            other => Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{fn_name}(space): '{other}' inválido (esperado: oklab, oklch, srgb, luma, linear-rgb, hsl, hsv, cmyk)"
                ),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{fn_name}(space): espera Str ou ColorSpace, recebeu {}", other.type_name()),
        )]),
    }
}

/// Parse cada item posicional para `GradientStop`.
///
/// Aceita:
/// - `Value::Color(c)` → `GradientStop::unspaced(c)` (offset auto).
/// - `Value::Array([Color, Ratio])` → `GradientStop::new(c, r)`.
fn parse_stops(items: &[Value]) -> SourceResult<Vec<GradientStop>> {
    let mut stops = Vec::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        let stop = match item {
            Value::Color(c) => GradientStop::unspaced(*c),
            Value::Array(arr) if arr.len() == 2 => {
                let c = match &arr[0] {
                    Value::Color(c) => *c,
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("gradient.linear: stop {}: primeiro elemento deve ser Color, recebeu {}",
                                i, other.type_name()),
                    )]),
                };
                let r = match &arr[1] {
                    Value::Ratio(r) => *r,
                    Value::Float(f) => Ratio(*f),
                    Value::Int(i64v) => Ratio(*i64v as f64),
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("gradient.linear: stop {}: segundo elemento deve ser Ratio/Float, recebeu {}",
                                i, other.type_name()),
                    )]),
                };
                if r.0 < 0.0 || r.0 > 1.0 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("gradient.linear: stop {}: offset {} fora de [0, 1]", i, r.0),
                    )]);
                }
                GradientStop::new(c, r)
            }
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("gradient.linear: stop {} deve ser Color ou [Color, Ratio], recebeu {}",
                        i, other.type_name()),
            )]),
        };
        stops.push(stop);
    }
    let any_offset = stops.iter().any(|stop| stop.offset.is_some());
    if any_offset {
        if stops.iter().any(|stop| stop.offset.is_none()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "either all stops must have an offset or none of them can",
            )]);
        }
        let offsets: Vec<_> = stops.iter().map(|stop| stop.offset.unwrap().0).collect();
        if offsets.windows(2).any(|pair| pair[1] < pair[0]) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "offsets must be in monotonic order",
            )]);
        }
        if offsets.first().copied() != Some(0.0) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "first stop must have an offset of 0",
            )]);
        }
        if offsets.last().copied() != Some(1.0) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "last stop must have an offset of 100%",
            )]);
        }
    }
    Ok(stops)
}

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
    let space = parse_space_named(args, "gradient.radial")?;
    for stop in &mut stops {
        stop.color = stop.color.to_space(space);
    }

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

/// Helper privado partilhado: parse `Value::Ratio` ou `Float`
/// para `Ratio`.
fn parse_ratio(val: &Value, fn_name: &str, field: &str) -> SourceResult<Ratio> {
    match val {
        Value::Ratio(r) => Ok(*r),
        Value::Float(f) => Ok(Ratio(*f)),
        Value::Int(i) => Ok(Ratio(*i as f64)),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}({}): espera Ratio/Float, recebeu {}",
                fn_name,
                field,
                other.type_name()
            ),
        )]),
    }
}

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
    let space = parse_space_named(args, "gradient.conic")?;
    for stop in &mut stops {
        stop.color = stop.color.to_space(space);
    }

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

// Tests para `native_gradient_linear` + `native_gradient_radial`
// + `native_gradient_conic` em `01_core/src/compiler/stdlib/mod.rs`
// tests module (usa `null_ctx!` + `null_world()` + `test_file_id()`).
