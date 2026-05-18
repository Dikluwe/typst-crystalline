//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 68fc3823
//! @layer L1
//! @updated 2026-05-15
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

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::axes::Axes;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::color::ColorSpace;
use crate::entities::gradient::{Gradient, GradientStop};
use crate::entities::layout_types::{Angle, Ratio};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

/// Constrói o módulo `gradient` como `Value::Dict` (paridade
/// `make_calc_module`). Acesso `gradient.linear(...)` /
/// `gradient.radial(...)` via `eval_field_access` sobre Dict.
pub fn make_gradient_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("linear".into(),
        Value::Func(Func::native("gradient.linear", native_gradient_linear)));
    // P264 — Radial activa per ADR-0088.
    dict.insert("radial".into(),
        Value::Func(Func::native("gradient.radial", native_gradient_radial)));
    // P267 — Conic activa per ADR-0089 (cluster Gradient 3/3 completo).
    dict.insert("conic".into(),
        Value::Func(Func::native("gradient.conic", native_gradient_conic)));
    Value::Dict(dict)
}

/// `gradient.linear(stops..., angle: ?)` → `Value::Gradient(Gradient::Linear)`.
pub fn native_gradient_linear(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    let stops = parse_stops(&args.items)?;
    if stops.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "gradient.linear: pelo menos 1 stop requerido".to_string(),
        )]);
    }

    let angle = match args.named.get("angle") {
        Some(Value::Angle(a)) => *a,
        Some(Value::Float(f)) => Angle::rad(*f),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.linear(angle): espera Angle ou Float, recebeu {}",
                    other.type_name()),
        )]),
        None => Angle::rad(0.0),
    };

    // P270 — named arg `space` (ADR-0091 EM VIGOR).
    let space = parse_space_named(args, "gradient.linear")?;

    // P273 — named arg `relative` cross-variant.
    let relative = parse_relative_named(args, "gradient.linear")?;

    for key in args.named.keys() {
        if key != "angle" && key != "space" && key != "relative" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("gradient.linear: argumento nomeado inesperado '{}' (esperado: angle, space, relative)", key),
            )]);
        }
    }

    use std::sync::Arc;
    use crate::entities::gradient::Linear;
    Ok(Value::Gradient(Gradient::Linear(Arc::new(Linear {
        stops: Arc::from(stops),
        angle,
        space,
        relative,
    }))))
}

/// P273 — Parser do named arg `relative` cross-variant (ADR-0091 §"Anotação
/// cumulativa P273").
///
/// Aceita `Value::Str("self" | "parent" | "auto")`. Default (sem named arg)
/// = `None` (Auto = `Self_` resolved). ADR-0064 §Caso A (`Smart<T>` →
/// `Option<T>` cristalino).
fn parse_relative_named(args: &Args, fn_name: &str)
    -> SourceResult<Option<crate::entities::gradient::RelativeTo>>
{
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
                Span::detached(),
                format!(
                    "{fn_name}(space): '{other}' inválido (esperado: oklab, oklch, srgb, luma, linear-rgb, hsl, hsv, cmyk)"
                ),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{fn_name}(space): espera Str, recebeu {}", other.type_name()),
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
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    let stops = parse_stops(&args.items)?;
    if stops.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "gradient.radial: pelo menos 1 stop requerido".to_string(),
        )]);
    }

    let center = match args.named.get("center") {
        Some(Value::Array(arr)) if arr.len() == 2 => {
            let x = parse_ratio(&arr[0], "gradient.radial", "center.x")?;
            let y = parse_ratio(&arr[1], "gradient.radial", "center.y")?;
            Axes::new(x, y)
        }
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(center): espera Array [Ratio, Ratio], recebeu {}",
                    other.type_name()),
        )]),
        None => Axes::new(Ratio(0.5), Ratio(0.5)),
    };

    let radius = match args.named.get("radius") {
        Some(Value::Ratio(r)) => *r,
        Some(Value::Float(f)) => Ratio(*f),
        Some(Value::Int(i)) => Ratio(*i as f64),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(radius): espera Ratio/Float, recebeu {}",
                    other.type_name()),
        )]),
        None => Ratio(0.5),
    };
    if radius.0 < 0.0 || radius.0 > 1.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(radius): {} fora de [0, 1]", radius.0),
        )]);
    }

    // P269 — named args focal_* (paridade vanilla RadialGradient).
    let focal_center = match args.named.get("focal_center") {
        Some(Value::Array(arr)) if arr.len() == 2 => {
            let x = parse_ratio(&arr[0], "gradient.radial", "focal_center.x")?;
            let y = parse_ratio(&arr[1], "gradient.radial", "focal_center.y")?;
            Axes::new(x, y)
        }
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(focal_center): espera Array [Ratio, Ratio], recebeu {}",
                    other.type_name()),
        )]),
        None => center,  // default vanilla: focal_center = center
    };

    let focal_radius = match args.named.get("focal_radius") {
        Some(Value::Ratio(r)) => *r,
        Some(Value::Float(f)) => Ratio(*f),
        Some(Value::Int(i)) => Ratio(*i as f64),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(focal_radius): espera Ratio/Float, recebeu {}",
                    other.type_name()),
        )]),
        None => Ratio(0.0),  // default vanilla: focal_radius = 0%
    };

    // Validação vanilla §1: focal_radius > radius → erro.
    if focal_radius.0 > radius.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.radial(focal_radius): {} > radius {}",
                    focal_radius.0, radius.0),
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

    for key in args.named.keys() {
        if key != "center" && key != "radius"
            && key != "focal_center" && key != "focal_radius"
            && key != "space" && key != "relative"
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
    use std::sync::Arc;
    use crate::entities::gradient::Radial;
    Ok(Value::Gradient(Gradient::Radial(Arc::new(Radial {
        stops: Arc::from(stops),
        center,
        radius,
        focal_center,
        focal_radius,
        space,
        relative,
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
            format!("{}({}): espera Ratio/Float, recebeu {}",
                    fn_name, field, other.type_name()),
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
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    let stops = parse_stops(&args.items)?;
    if stops.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "gradient.conic: pelo menos 1 stop requerido".to_string(),
        )]);
    }

    let center = match args.named.get("center") {
        Some(Value::Array(arr)) if arr.len() == 2 => {
            let x = parse_ratio(&arr[0], "gradient.conic", "center.x")?;
            let y = parse_ratio(&arr[1], "gradient.conic", "center.y")?;
            Axes::new(x, y)
        }
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.conic(center): espera Array [Ratio, Ratio], recebeu {}",
                    other.type_name()),
        )]),
        None => Axes::new(Ratio(0.5), Ratio(0.5)),
    };

    let angle = match args.named.get("angle") {
        Some(Value::Angle(a)) => *a,
        Some(Value::Float(f)) => Angle::rad(*f),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.conic(angle): espera Angle ou Float, recebeu {}",
                    other.type_name()),
        )]),
        None => Angle::rad(0.0),
    };

    // P270 — named arg `space`.
    let space = parse_space_named(args, "gradient.conic")?;

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

    use std::sync::Arc;
    use crate::entities::gradient::Conic;
    Ok(Value::Gradient(Gradient::Conic(Arc::new(Conic {
        stops: Arc::from(stops),
        center,
        angle,
        space,
        relative,
    }))))
}

// Tests para `native_gradient_linear` + `native_gradient_radial`
// + `native_gradient_conic` em `01_core/src/rules/stdlib/mod.rs`
// tests module (usa `null_ctx!` + `null_world()` + `test_file_id()`).
