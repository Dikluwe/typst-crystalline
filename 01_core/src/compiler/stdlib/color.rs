//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/color.md
//! @prompt-hash f9266994
//! @layer L1
//! @updated 2026-06-27
//!
//! Módulo `color` — tipo `color` (P736/P1143) com constructors, operadores e
//! as 18 cores predefinidas do vanilla ratificado.
//! **P742** — semântica dos operadores corrigida por medição vanilla
//! (lighten/darken no espaço da cor, saturate/desaturate via HSV, negate em
//! Oklab) + despacho de métodos de instância (`red.lighten(20%)`).

use ecow::EcoString;
use std::sync::Arc;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::color::{ColorSpaceError, SpotColor, SpotColorant};
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::layout_types::{Color, ColorSpace};
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::{Type, Value};

use super::err;

fn err_typed<T>(msg: impl Into<String>) -> SourceResult<T> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

/// Devolve o valor associado a `field` no tipo `color` (P736).
///
/// `color` é `Value::Type(Type::Color)` no scope global (paridade vanilla
/// — medido: `type(color)` → `type`); os fields resolvem-se por field
/// access em `Value::Type` (`eval/bindings.rs`), que delega aqui.
///
/// **P742 — 17 fields** (paridade do inventário medido P736): 8 constructors
/// (cinco também globais: `rgb`, `luma`, `cmyk`, `oklab`, `oklch`; três somente
/// qualificados: `linear-rgb`, `hsl`, `hsv`) + 9 operadores. Os nomes das
/// funcs são os **nomes plain do vanilla** — medido: `repr(color.rgb)` →
/// `rgb`, `repr(color.lighten)` → `lighten`; `color.rgb == rgb` → `true`
/// (igualdade por nome, `entities/func.rs`). `None` para campo inexistente
/// — o chamador emite "type color does not contain field `<f>`" (verbatim
/// vanilla).
///
/// Histórico: até P736 era `make_color_module() -> Value` (`Value::Dict`
/// com os 6 operadores; os constructors não eram acessíveis via `color.*`).
pub fn color_type_field(field: &str) -> Option<Value> {
    use super::foundations::{
        native_cmyk, native_hsl, native_hsv, native_linear_rgb, native_luma,
        native_oklab, native_oklch, native_rgb,
    };
    if field == "map" {
        return Some(make_color_map_module());
    }
    let function = match field {
        "rgb" => Value::Func(Func::native("rgb", native_rgb)),
        "linear-rgb" => Value::Func(Func::native("linear-rgb", native_linear_rgb)),
        "luma" => Value::Func(Func::native("luma", native_luma)),
        "cmyk" => Value::Func(Func::native("cmyk", native_cmyk)),
        "hsl" => Value::Func(Func::native("hsl", native_hsl)),
        "hsv" => Value::Func(Func::native("hsv", native_hsv)),
        "oklab" => Value::Func(Func::native("oklab", native_oklab)),
        "oklch" => Value::Func(Func::native("oklch", native_oklch)),
        "lighten" => Value::Func(Func::native("lighten", native_color_lighten)),
        "darken" => Value::Func(Func::native("darken", native_color_darken)),
        "mix" => Value::Func(Func::native("mix", native_color_mix)),
        "negate" => Value::Func(Func::native("negate", native_color_negate)),
        "saturate" => Value::Func(Func::native("saturate", native_color_saturate)),
        "desaturate" => Value::Func(Func::native("desaturate", native_color_desaturate)),
        "rotate" => Value::Func(Func::native("rotate", native_color_rotate)),
        "components" => Value::Func(Func::native("components", native_color_components)),
        "space" => Value::Func(Func::native("space", native_color_space)),
        "to-hex" => Value::Func(Func::native("to-hex", native_color_to_hex)),
        "transparentize" => {
            Value::Func(Func::native("transparentize", native_color_transparentize))
        }
        "opacify" => Value::Func(Func::native("opacify", native_color_opacify)),
        // **P1473** — `color.spot` é o tipo-valor `SpotColorant`, paralelo a
        // como `color` é `Value::Type(Type::Color)`. `color.spot(name,
        // fallback)` despacha como construtor via `Value::Type(t) => match
        // t {...}` em `eval/call_dispatch.rs` (mesmo mecanismo de `int`/
        // `str`/`stroke`/etc. — sem novo caminho de dispatch).
        "spot" => Value::Type(Type::SpotColorant),
        _ => return predefined_color(field).map(Value::Color),
    };
    Some(function)
}

const COLOR_MAP_DATA: &str = include_str!("color_maps.hex");

fn make_color_map_module() -> Value {
    let mut scope = Scope::new();
    for line in COLOR_MAP_DATA.lines() {
        let mut fields = line.split_ascii_whitespace();
        let Some(name) = fields.next() else {
            continue;
        };
        let colors = fields
            .map(|token| {
                let rgba = u32::from_str_radix(token, 16)
                    .expect("color_maps.hex is generated from the pinned vanilla source");
                Value::Color(Color::rgba(
                    (rgba >> 24) as u8,
                    (rgba >> 16) as u8,
                    (rgba >> 8) as u8,
                    rgba as u8,
                ))
            })
            .collect();
        scope.define(name, Value::Array(colors));
    }
    Value::Module(Module::new("map", scope))
}

/// **P742** — os 9 métodos de instância de `Value::Color` (despacho P506
/// em `eval/closures.rs`). Método fora desta lista cai no caminho genérico
/// de field access (comportamento pré-P742 preservado).
pub fn is_color_instance_method(method: &str) -> bool {
    matches!(
        method,
        "lighten"
            | "darken"
            | "mix"
            | "negate"
            | "saturate"
            | "desaturate"
            | "rotate"
            | "components"
            | "space"
            | "to-hex"
            | "transparentize"
            | "opacify"
    )
}

/// **P1143** — fonte canônica das 18 cores ratificadas, compartilhada pelo
/// scope global e pelo namespace `color`. As quatro escalas de cinza preservam
/// o espaço Luma observável; as demais preservam sRGB.
const PREDEFINED_COLORS: &[(&str, Color)] = &[
    ("black", Color::Luma { l: 0.0, a: 1.0 }),
    ("gray", Color::Luma { l: 170.0 / 255.0, a: 1.0 }),
    ("silver", Color::Luma { l: 221.0 / 255.0, a: 1.0 }),
    ("white", Color::Luma { l: 1.0, a: 1.0 }),
    ("navy", Color::Srgb { r: 0.0, g: 31.0 / 255.0, b: 63.0 / 255.0, a: 1.0 }),
    ("blue", Color::Srgb { r: 0.0, g: 0.454902, b: 0.85098, a: 1.0 }),
    ("aqua", Color::Srgb { r: 127.0 / 255.0, g: 219.0 / 255.0, b: 1.0, a: 1.0 }),
    (
        "teal",
        Color::Srgb {
            r: 57.0 / 255.0,
            g: 204.0 / 255.0,
            b: 204.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "eastern",
        Color::Srgb {
            r: 35.0 / 255.0,
            g: 157.0 / 255.0,
            b: 173.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "purple",
        Color::Srgb {
            r: 177.0 / 255.0,
            g: 13.0 / 255.0,
            b: 201.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "fuchsia",
        Color::Srgb {
            r: 240.0 / 255.0,
            g: 18.0 / 255.0,
            b: 190.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "maroon",
        Color::Srgb {
            r: 133.0 / 255.0,
            g: 20.0 / 255.0,
            b: 75.0 / 255.0,
            a: 1.0,
        },
    ),
    ("red", Color::Srgb { r: 1.0, g: 0.254902, b: 0.211765, a: 1.0 }),
    ("orange", Color::Srgb { r: 1.0, g: 133.0 / 255.0, b: 27.0 / 255.0, a: 1.0 }),
    ("yellow", Color::Srgb { r: 1.0, g: 220.0 / 255.0, b: 0.0, a: 1.0 }),
    (
        "olive",
        Color::Srgb {
            r: 61.0 / 255.0,
            g: 153.0 / 255.0,
            b: 112.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "green",
        Color::Srgb {
            r: 46.0 / 255.0,
            g: 204.0 / 255.0,
            b: 64.0 / 255.0,
            a: 1.0,
        },
    ),
    ("lime", Color::Srgb { r: 1.0 / 255.0, g: 1.0, b: 112.0 / 255.0, a: 1.0 }),
];

fn predefined_color(name: &str) -> Option<Color> {
    PREDEFINED_COLORS
        .iter()
        .find_map(|(candidate, color)| (*candidate == name).then_some(color.clone()))
}

/// Cores nomeadas globais para injeção no scope de eval.
///
/// **P687** — paridade vanilla 0.15.0 (969087ec): as 18 cores oficiais
/// (`lib.rs:359-376` / `visualize/color.rs:291-322`) com bytes sRGB exactos
/// (confirmados via `#repr`). `cyan`/`magenta`/`none` são extras pré-P687 mantidos
/// sem regressão (não fazem parte do conjunto oficial vanilla).
pub fn predefined_color_bindings() -> Vec<(EcoString, Value)> {
    PREDEFINED_COLORS
        .iter()
        .map(|(name, color)| ((*name).into(), Value::Color(color.clone())))
        .chain([
            // Extras cristalinos preservados somente no global.
            ("cyan".into(), Value::Color(Color::rgb(0x00, 0xB3, 0xB3))),
            ("magenta".into(), Value::Color(Color::rgb(0xE5, 0x00, 0xE5))),
            ("none".into(), Value::None),
        ])
        .collect()
}

fn extract_color_arg(val: &Value, fn_name: &str, arg_name: &str) -> SourceResult<Color> {
    match val {
        Value::Color(c) => Ok(c.clone()),
        other => err_typed(format!(
            "{}: argumento '{}' deve ser Color, recebeu {}",
            fn_name,
            arg_name,
            other.type_name()
        )),
    }
}

fn extract_ratio_arg(val: &Value, fn_name: &str, arg_name: &str) -> SourceResult<f32> {
    match val {
        Value::Float(f) => Ok(*f as f32),
        Value::Int(i) => Ok(*i as f32 / 100.0),
        Value::Relative(r) if r.abs.is_zero() => Ok(r.rel as f32),
        // P842 (#32) — o literal percentual é agora `Value::Ratio`.
        Value::Ratio(r) => Ok(r.get() as f32),
        other => err_typed(format!(
            "{}: argumento '{}' deve ser Float ou Percentage, recebeu {}",
            fn_name,
            arg_name,
            other.type_name()
        )),
    }
}

/// **P744** — extrai um `ColorSpace` de um `Value::Func` cujo nome é um dos
/// constructors de cor. Erro verbatim-style do vanilla para valores inválidos.
fn extract_color_space_arg(
    val: &Value,
    _fn_name: &str,
    _arg_name: &str,
) -> SourceResult<crate::entities::color::ColorSpace> {
    use crate::entities::color::ColorSpace;
    match val {
        Value::SpotColorant(colorant) => Ok(ColorSpace::Spot(colorant.clone())),
        Value::Func(f) => match f.name() {
            Some("rgb") => Ok(ColorSpace::Srgb),
            Some("luma") => Ok(ColorSpace::Luma),
            Some("cmyk") => Ok(ColorSpace::Cmyk),
            Some("oklab") => Ok(ColorSpace::Oklab),
            Some("oklch") => Ok(ColorSpace::Oklch),
            Some("linear-rgb") => Ok(ColorSpace::LinearRgb),
            Some("hsl") => Ok(ColorSpace::Hsl),
            Some("hsv") => Ok(ColorSpace::Hsv),
            _ => err_typed(format!(
                "expected `rgb`, `luma`, `cmyk`, `oklab`, `oklch`, `color.linear-rgb`, `color.hsl`, `color.hsv`, or spot colorant, found {}",
                vanilla_type_name(val)
            )),
        },
        other => err_typed(format!(
            "expected `rgb`, `luma`, `cmyk`, `oklab`, `oklch`, `color.linear-rgb`, `color.hsl`, `color.hsv`, or spot colorant, found {}",
            vanilla_type_name(other)
        )),
    }
}

fn color_space_diagnostic(error: ColorSpaceError) -> Vec<SourceDiagnostic> {
    let message = match error {
        ColorSpaceError::ProcessToSpot { .. } => {
            "cannot convert process color to spot colorant space"
        }
        ColorSpaceError::SpotToOther { .. } => {
            "cannot convert spot color to different spot colorant"
        }
    };
    vec![SourceDiagnostic::error(Span::detached(), message)]
}

fn check_space(color: &Color, space: &ColorSpace) -> SourceResult<()> {
    color
        .clone()
        .to_space(space.clone())
        .map(|_| ())
        .map_err(color_space_diagnostic)
}

/// `color.lighten(col, amount)` — aumenta luminância por `amount` **no
/// espaço da própria cor** (semântica vanilla corrigida em P742; era Oklch).
pub(crate) fn native_color_lighten(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.lighten() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col, "color.lighten", "col")?;
            let a = extract_ratio_arg(amount, "color.lighten", "amount")?;
            Ok(Value::Color(c.lighten(a)))
        }
        _ => err(format!(
            "color.lighten() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.darken(col, amount)` — diminui luminância por `amount` **no
/// espaço da própria cor** (semântica vanilla corrigida em P742; era Oklch).
pub(crate) fn native_color_darken(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.darken() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col, "color.darken", "col")?;
            let a = extract_ratio_arg(amount, "color.darken", "amount")?;
            Ok(Value::Color(c.darken(a)))
        }
        _ => err(format!(
            "color.darken() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.mix(col1, col2, weight: 0.5, space: auto)` — interpolação linear
/// no espaço indicado (default Oklab; resultado no espaço indicado, P744).
pub(crate) fn native_color_mix(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let space = match args.named.get("space") {
        Some(v) => Some(extract_color_space_arg(v, "color.mix", "space")?),
        None => None,
    };
    for key in args.named.keys() {
        if !matches!(key.as_str(), "weight" | "space") {
            return err(format!("color.mix(): argumento nomeado inesperado '{}'", key));
        }
    }
    if let Some(weight_value) = args.named.get("weight") {
        let weight = extract_ratio_arg(weight_value, "color.mix", "weight")?;
        return match args.items.as_slice() {
            [Value::Color(first), Value::Color(second)] => {
                if let Some(target) = &space {
                    check_space(first, target)?;
                    check_space(second, target)?;
                }
                Ok(Value::Color(first.clone().mix(second.clone(), weight, space)))
            }
            _ => err("color.mix(weight:) requer exatamente duas cores sem pesos"),
        };
    }

    fn entry(value: &Value) -> SourceResult<(Color, f32)> {
        match value {
            Value::Color(color) => Ok((color.clone(), 1.0)),
            Value::Array(values) if values.len() == 2 => {
                let Value::Color(color) = &values[0] else {
                    return err_typed("expected a color or color-weight pair");
                };
                let weight = match &values[1] {
                    Value::Float(value) => *value as f32,
                    Value::Int(value) => *value as f32,
                    Value::Ratio(value) => value.get() as f32,
                    other => {
                        return err_typed(format!(
                            "expected float or ratio, found {}",
                            vanilla_type_name(other)
                        ))
                    }
                };
                Ok((color.clone(), weight))
            }
            _ => err_typed("expected a color or color-weight pair"),
        }
    }

    let colors: Vec<(Color, f32)> =
        args.items.iter().map(entry).collect::<SourceResult<_>>()?;
    if let Some(target) = &space {
        for (color, _) in &colors {
            check_space(color, target)?;
        }
    }
    if matches!(
        space,
        Some(ColorSpace::Hsl) | Some(ColorSpace::Hsv) | Some(ColorSpace::Oklch)
    ) && colors.len() > 2
    {
        return err("cannot mix more than two colors in a hue-based space");
    }
    match Color::mix_weighted(&colors, space) {
        Some(color) => Ok(Value::Color(color)),
        None => {
            let total: f32 = colors.iter().map(|(_, weight)| *weight).sum();
            if total <= 0.0 || colors.is_empty() {
                err("sum of weights must be positive")
            } else {
                err("cannot mix more than two colors in a hue-based space")
            }
        }
    }
}

/// `color.negate(col, space: auto)` — negação no espaço indicado (default
/// Oklab) com conversão de volta ao espaço original (P744).
pub(crate) fn native_color_negate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let space = match args.named.get("space") {
        Some(v) => Some(extract_color_space_arg(v, "color.negate", "space")?),
        None => None,
    };
    for key in args.named.keys() {
        if key.as_str() != "space" {
            return err(format!(
                "color.negate(): argumento nomeado inesperado '{}'",
                key
            ));
        }
    }
    match args.items.as_slice() {
        [col] => {
            let c = extract_color_arg(col, "color.negate", "col")?;
            if let Some(target) = &space {
                check_space(&c, target)?;
                if matches!(&c, Color::Spot(_)) && !matches!(target, ColorSpace::Spot(_))
                {
                    return err_typed(
                        "cannot convert process color to spot colorant space",
                    );
                }
            }
            Ok(Value::Color(c.negate(space)))
        }
        _ => err(format!(
            "color.negate() requer 1 argumento (col), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.saturate(col, amount)` — aumenta a saturação por `amount`
/// (**P742**: semântica vanilla via HSV; Luma → erro verbatim medido).
pub(crate) fn native_color_saturate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.saturate() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col, "color.saturate", "col")?;
            let a = extract_ratio_arg(amount, "color.saturate", "amount")?;
            if matches!(&c, Color::Spot(_)) {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "cannot saturate spot color",
                )
                .with_hint("try using the fallback color instead")]);
            }
            match c.saturate(a) {
                Some(sat) => Ok(Value::Color(sat)),
                // Verbatim vanilla (medido P742) + hint.
                None => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "cannot saturate grayscale color",
                )
                .with_hint("try converting your color to RGB first")]),
            }
        }
        _ => err(format!(
            "color.saturate() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.desaturate(col, amount)` — diminui a saturação por `amount`
/// (**P742**: semântica vanilla via HSV; Luma → erro verbatim medido).
pub(crate) fn native_color_desaturate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.desaturate() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col, "color.desaturate", "col")?;
            let a = extract_ratio_arg(amount, "color.desaturate", "amount")?;
            if matches!(&c, Color::Spot(_)) {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "cannot desaturate spot color",
                )
                .with_hint("try using the fallback color instead")]);
            }
            match c.desaturate(a) {
                Some(desat) => Ok(Value::Color(desat)),
                // Verbatim vanilla (medido P742) + hint.
                None => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "cannot desaturate grayscale color",
                )
                .with_hint("try converting your color to RGB first")]),
            }
        }
        _ => err(format!(
            "color.desaturate() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P744** — `color.rotate(col, angle, space: auto)` — rotação de hue no
/// espaço indicado (default Oklch). Só espaços com hue são válidos.
pub(crate) fn native_color_rotate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if matches!(args.named.get("space"), Some(Value::SpotColorant(_))) {
        return err_typed("expected `rgb`, `luma`, `cmyk`, `oklab`, `oklch`, `color.linear-rgb`, `color.hsl`, or `color.hsv`, found spot colorant");
    }
    let space = match args.named.get("space") {
        Some(v) => Some(extract_color_space_arg(v, "color.rotate", "space")?),
        None => None,
    };
    for key in args.named.keys() {
        if key.as_str() != "space" {
            return err(format!(
                "color.rotate(): argumento nomeado inesperado '{}'",
                key
            ));
        }
    }
    match args.items.as_slice() {
        [col, angle] => {
            let c = extract_color_arg(col, "color.rotate", "col")?;
            if matches!(&c, Color::Spot(_)) {
                return err_typed("cannot convert process color to spot colorant space");
            }
            let deg = match angle {
                Value::Angle(a) => a.to_deg() as f32,
                other => {
                    return err_typed(format!(
                        "color.rotate: argumento 'angle' deve ser Angle, recebeu {}",
                        other.type_name()
                    ))
                }
            };
            match c.rotate(deg, space) {
                Some(rot) => Ok(Value::Color(rot)),
                None => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "this color space does not support hue rotation",
                )]),
            }
        }
        _ => err(format!(
            "color.rotate() requer 2 argumentos (col, angle), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P742** — `color.components(col, alpha: true)` — componentes da cor
/// como `Array` de `Ratio`/`Float`/`Angle` (paridade vanilla medida:
/// `red.components()` → `(100%, 25.49%, 21.18%, 100%)`).
pub(crate) fn native_color_components(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::color::ColorComponent;
    use crate::entities::layout_types::{Angle, Ratio};

    let alpha = match args.named.get("alpha") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return err_typed(format!(
                "color.components: argumento 'alpha' deve ser Bool, recebeu {}",
                other.type_name()
            ))
        }
        None => true,
    };
    for key in args.named.keys() {
        if key.as_str() != "alpha" {
            return err(format!(
                "color.components(): argumento nomeado inesperado '{}'",
                key
            ));
        }
    }
    match args.items.as_slice() {
        [col] => {
            let c = extract_color_arg(col, "color.components", "col")?;
            if let Color::Spot(spot) = &c {
                // ColorComponent usa f32 para processo; tint spot conserva Ratio f64.
                return Ok(Value::Array(vec![Value::Ratio(spot.tint)]));
            }
            let items = c
                .components(alpha)
                .into_iter()
                .map(|comp| match comp {
                    ColorComponent::Ratio(v) => {
                        Value::Ratio(Ratio::from_percent(v as f64 * 100.0))
                    }
                    ColorComponent::Float(v) => Value::Float(v as f64),
                    ColorComponent::Angle(v) => Value::Angle(Angle::deg(v as f64)),
                })
                .collect();
            Ok(Value::Array(items))
        }
        _ => err(format!(
            "color.components() requer 1 argumento (col), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P742** — `color.space(col)` — devolve a função construtora do espaço
/// da cor (paridade vanilla medida: `red.space() == rgb` → `true`,
/// `repr(red.space())` → `rgb`; a igualdade por nome de nativas, P742 em
/// `entities/func.rs`, torna `Func::native("rgb", ..)` igual ao binding
/// global `rgb`).
pub(crate) fn native_color_space(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::color::ColorSpace;

    if !args.named.is_empty() {
        return err("color.space() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col] => {
            let c = extract_color_arg(col, "color.space", "col")?;
            use super::foundations::{
                native_cmyk, native_hsl, native_hsv, native_linear_rgb, native_luma,
                native_oklab, native_oklch, native_rgb,
            };
            let value = match c.space() {
                ColorSpace::Srgb => Value::Func(Func::native("rgb", native_rgb)),
                ColorSpace::Luma => Value::Func(Func::native("luma", native_luma)),
                ColorSpace::LinearRgb => {
                    Value::Func(Func::native("linear-rgb", native_linear_rgb))
                }
                ColorSpace::Oklab => Value::Func(Func::native("oklab", native_oklab)),
                ColorSpace::Oklch => Value::Func(Func::native("oklch", native_oklch)),
                ColorSpace::Cmyk => Value::Func(Func::native("cmyk", native_cmyk)),
                ColorSpace::Hsl => Value::Func(Func::native("hsl", native_hsl)),
                ColorSpace::Hsv => Value::Func(Func::native("hsv", native_hsv)),
                ColorSpace::Spot(colorant) => Value::SpotColorant(colorant),
            };
            Ok(value)
        }
        _ => err(format!(
            "color.space() requer 1 argumento (col), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P744** — `color.to-hex(col)` → `Str` com representação hex sRGB.
pub(crate) fn native_color_to_hex(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.to-hex() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col] => {
            let c = extract_color_arg(col, "color.to-hex", "col")?;
            Ok(Value::Str(c.to_hex().into()))
        }
        _ => err(format!(
            "color.to-hex() requer 1 argumento (col), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P744** — `color.transparentize(col, factor)` → Color com alpha reduzido.
pub(crate) fn native_color_transparentize(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.transparentize() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, factor] => {
            let c = extract_color_arg(col, "color.transparentize", "col")?;
            let f = extract_ratio_arg(factor, "color.transparentize", "factor")?;
            if matches!(&c, Color::Spot(_)) {
                return err_typed("spot colors do not have an alpha component");
            }
            match c.transparentize(f) {
                Some(t) => Ok(Value::Color(t)),
                None => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "CMYK does not have an alpha component",
                )]),
            }
        }
        _ => err(format!(
            "color.transparentize() requer 2 argumentos (col, factor), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P744** — `color.opacify(col, factor)` → Color com alpha aumentado.
pub(crate) fn native_color_opacify(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.opacify() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, factor] => {
            let c = extract_color_arg(col, "color.opacify", "col")?;
            let f = extract_ratio_arg(factor, "color.opacify", "factor")?;
            if matches!(&c, Color::Spot(_)) {
                return err_typed("spot colors do not have an alpha component");
            }
            match c.opacify(f) {
                Some(o) => Ok(Value::Color(o)),
                None => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "CMYK does not have an alpha component",
                )]),
            }
        }
        _ => err(format!(
            "color.opacify() requer 2 argumentos (col, factor), recebeu {}",
            args.items.len()
        )),
    }
}

// ── P1473 — `color.spot(name, fallback)` (escopo reduzido, sem `.tint()`) ───

/// `color.spot(name, fallback)` — constructor de `Value::SpotColorant`.
/// Despachado por `Value::Type(Type::SpotColorant) => ...` em
/// `eval/call_dispatch.rs` (mesmo mecanismo de `int(...)`/`str(...)`/
/// `stroke(...)`, sem novo caminho de dispatch). Ambos os argumentos são
/// posicionais (paridade vanilla: `#[func(constructor)] fn construct(name:
/// Option<Str>, fallback: ProcessColor)`, sem `#[named]`).
///
/// `name` (`Str` ou `none`, opcional — default `none`, genuinamente
/// opcional na língua): tipo errado → `` expected "all", string, or none,
/// found <tipo> ``. `fallback` (`Color` de processo, obrigatório): ausente
/// → `missing argument: fallback`; `spot`-de-`spot` → `expected color,
/// found spot colorant` (não há spot-de-spot); outro tipo errado →
/// `expected color, found <tipo>`.
pub(crate) fn native_spot_colorant_construct(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        return err_typed(format!("unexpected argument: {key}"));
    }
    if args.items.len() > 2 {
        return err_typed("unexpected argument");
    }

    // `name` é genuinamente opcional (positional trailing-omittable); um
    // único positional que já é `Color` é o `fallback` sozinho (`name`
    // omitido), não um `name` malformado.
    let (name_arg, fallback_arg): (Option<&Value>, Option<&Value>) =
        match args.items.len() {
            0 => (None, None),
            1 => match &args.items[0] {
                v @ Value::Color(_) => (None, Some(v)),
                other => (Some(other), None),
            },
            _ => (Some(&args.items[0]), Some(&args.items[1])),
        };

    let name: Option<EcoString> = match name_arg {
        None => None,
        Some(Value::None) => None,
        Some(Value::Str(s)) => Some(s.clone()),
        Some(other) => {
            return err_typed(format!(
                "expected \"all\", string, or none, found {}",
                vanilla_type_name(other)
            ));
        }
    };

    let fallback = match fallback_arg {
        None => return err_typed("missing argument: fallback"),
        Some(Value::Color(Color::Spot(_))) => {
            return err_typed("spot color found where process color is required");
        }
        Some(Value::Color(c)) => c.clone(),
        Some(Value::SpotColorant(_)) => {
            return err_typed("expected color, found spot colorant");
        }
        Some(other) => {
            return err_typed(format!(
                "expected color, found {}",
                vanilla_type_name(other)
            ));
        }
    };

    Ok(Value::SpotColorant(crate::entities::color::SpotColorant { name, fallback }))
}

/// Constrói uma cor spot sem perder a identidade do colorante.
pub(crate) fn native_spot_tint(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        let message = if key == "value" {
            "the argument `value` is positional".to_string()
        } else {
            format!("unexpected argument: {key}")
        };
        return Err(vec![SourceDiagnostic::error(args.span, message)]);
    }
    let colorant = match args.items.first() {
        Some(Value::SpotColorant(colorant)) => colorant,
        Some(other) => {
            return err_typed(format!(
                "expected spot colorant, found {}",
                vanilla_type_name(other)
            ))
        }
        None => return err_typed("missing argument: self"),
    };
    let tint = match args.items.get(1) {
        Some(Value::Ratio(value)) => *value,
        Some(other) => {
            return err_typed(format!(
                "expected ratio, found {}",
                vanilla_type_name(other)
            ))
        }
        None => return err_typed("missing argument: value"),
    };
    if args.items.len() > 2 {
        return err_typed("unexpected argument");
    }
    if tint.get() < 0.0 {
        return err_typed("spot color tint must be positive");
    }
    if tint.get() > 1.0 {
        return err_typed("spot color tint must not exceed 100%");
    }
    Ok(Value::Color(Color::Spot(SpotColor {
        colorant: Arc::new(colorant.clone()),
        tint,
    })))
}

/// Injeta o receptor de uma chamada `spot.tint(value)` sem alterar a ABI
/// da nativa estática `color.spot.tint(spot, value)`.
pub(crate) fn dispatch_spot_colorant_tint(
    colorant: &SpotColorant,
    args: Args,
    ctx: &mut EvalContext,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    let synth = if args.occurrences.is_some() {
        let mut occurrences = args.occurrence_sequence();
        occurrences.insert(
            0,
            ArgOccurrence {
                name: None,
                value: Value::SpotColorant(colorant.clone()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        );
        Args::from_occurrences(args.span, occurrences)
    } else {
        let mut items = args.items;
        items.insert(0, Value::SpotColorant(colorant.clone()));
        Args::from_parts(items, args.named, args.span)
    };
    native_spot_tint(ctx, &synth, world, current_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes_of(name: &str) -> Option<(u8, u8, u8)> {
        for (n, v) in predefined_color_bindings() {
            if n == name {
                if let Value::Color(c) = v {
                    let (r, g, b, _a) = c.to_srgb();
                    return Some((r, g, b));
                }
            }
        }
        None
    }

    fn has(name: &str) -> bool {
        predefined_color_bindings().iter().any(|(n, _)| n == name)
    }

    #[test]
    fn p687_cores_vanilla_18_srgb_exacto() {
        let esperado: [(&str, (u8, u8, u8)); 18] = [
            ("black", (0x00, 0x00, 0x00)),
            ("gray", (0xAA, 0xAA, 0xAA)),
            ("silver", (0xDD, 0xDD, 0xDD)),
            ("white", (0xFF, 0xFF, 0xFF)),
            ("navy", (0x00, 0x1F, 0x3F)),
            ("blue", (0x00, 0x74, 0xD9)),
            ("aqua", (0x7F, 0xDB, 0xFF)),
            ("teal", (0x39, 0xCC, 0xCC)),
            ("eastern", (0x23, 0x9D, 0xAD)),
            ("purple", (0xB1, 0x0D, 0xC9)),
            ("fuchsia", (0xF0, 0x12, 0xBE)),
            ("maroon", (0x85, 0x14, 0x4B)),
            ("red", (0xFF, 0x41, 0x36)),
            ("orange", (0xFF, 0x85, 0x1B)),
            ("yellow", (0xFF, 0xDC, 0x00)),
            ("olive", (0x3D, 0x99, 0x70)),
            ("green", (0x2E, 0xCC, 0x40)),
            ("lime", (0x01, 0xFF, 0x70)),
        ];
        for (nome, bytes) in esperado {
            assert_eq!(
                bytes_of(nome),
                Some(bytes),
                "cor '{nome}' deve estar ligada com sRGB exacto {bytes:?}"
            );
        }
    }

    #[test]
    fn p687_cores_inexistentes_no_vanilla_ausentes() {
        // Confirmado por sonda: vanilla 0.15.0 não tem `ostrich` nem `pink`.
        assert!(!has("ostrich"));
        assert!(!has("pink"));
    }

    #[test]
    fn p687_extras_pre_p687_sem_regressao() {
        // Mantidos para não regredir documentos/testes anteriores a P687.
        assert_eq!(bytes_of("cyan"), Some((0x00, 0xB3, 0xB3)));
        assert_eq!(bytes_of("magenta"), Some((0xE5, 0x00, 0xE5)));
        assert!(has("none"));
    }
}

/// P1473 (Agente B) — contrato RED de `color.spot` (escopo reduzido: sem
/// `.tint()`, sem variante em `Color`). Fonte:
/// `00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/compiler/stdlib/color.md` §P1473 +
/// `00_nucleo/prompts/entities/color.md` §P1473 +
/// `00_nucleo/prompts/entities/value.md` §P1473. Escrito sem ver a
/// implementação (isolamento Agente A/B).
///
/// `Type::SpotColorant`/`Value::SpotColorant` (ou equivalente) ainda não
/// existem neste baseline. Para que o RED seja "member missing"/runtime
/// (não "compilação partida" — o que impediria confirmar o RED de
/// qualquer outro teste no mesmo crate), **todos** os observáveis abaixo
/// passam exclusivamente por `eval_expression` (avaliação de expressões
/// Typst) e por mecanismos genéricos já existentes sobre `Value`
/// (`Type::name()`, `repr()`, `==`/`!=`) — nunca por construção Rust
/// directa de `SpotColorant`/`Type::SpotColorant`. Isto é deliberado, não
/// uma limitação contornada: `type(color.spot(...))` já devolve
/// `Value::Type(t)` através do `native_type` genérico existente
/// (`Ok(Value::Type(v.type_of()))`, `foundations/ty.rs:28`), e `t.name()`
/// funciona para **qualquer** variante de `Type` sem eu precisar de a
/// nomear — nem sequer depois de o Agente A a acrescentar.
#[cfg(test)]
mod p1473_tests {
    use super::*;
    use crate::compiler::eval::eval_expression;
    use crate::entities::source_result::SourceResult;

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1473).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    fn eval(src: &str) -> SourceResult<Value> {
        eval_expression(&NullWorld::default(), src).0
    }

    fn expect_ok(src: &str) -> Value {
        match eval(src) {
            Ok(v) => v,
            Err(errors) => panic!("{src}: esperava Ok, obteve Err({errors:?})"),
        }
    }

    fn expect_ok_str(src: &str) -> String {
        match expect_ok(src) {
            Value::Str(s) => s.to_string(),
            other => panic!("{src}: esperava Value::Str, obteve {other:?}"),
        }
    }

    fn expect_err_message(src: &str) -> String {
        match eval(src) {
            Err(errors) => {
                assert_eq!(
                    errors.len(),
                    1,
                    "{src}: esperava exactamente 1 diagnóstico, obteve {errors:?}"
                );
                errors[0].message.clone()
            }
            Ok(v) => panic!("{src}: esperava Err, obteve Ok({v:?})"),
        }
    }

    const FALLBACK: &str = "cmyk(0%, 0%, 0%, 0%)";

    #[test]
    fn p1473_color_spot_bare_is_a_type_value() {
        // Medido no L0: type(color.spot) → "type" no vanilla — `color.spot`
        // não chamado é um valor-tipo, tal como `color`/`int`/`str`.
        assert_eq!(expect_ok("type(color.spot) == type"), Value::Bool(true));
    }

    #[test]
    fn p1473_color_spot_constructed_instance_has_spot_type_name() {
        // Medido no L0: type(color.spot("X", cmyk(..))) → "spot" — sem
        // tint ainda não é Color. `t.name()` é genérico: não preciso de
        // nomear `Type::SpotColorant` para o chamar.
        match eval(&format!(r#"type(color.spot("X", {FALLBACK}))"#)) {
            Ok(Value::Type(t)) => assert_eq!(t.name(), "spot"),
            other => panic!("esperava Ok(Value::Type(_)), obteve {other:?}"),
        }
    }

    #[test]
    fn p1473_color_spot_repr_with_name() {
        let fallback_repr = expect_ok_str(&format!("repr({FALLBACK})"));
        let expected = format!(r#"color.spot("X", {fallback_repr})"#);
        assert_eq!(
            expect_ok_str(&format!(r#"repr(color.spot("X", {FALLBACK}))"#)),
            expected
        );
    }

    #[test]
    fn p1473_color_spot_accepts_none_name_and_repr_preserves_it() {
        let fallback_repr = expect_ok_str(&format!("repr({FALLBACK})"));
        let expected = format!("color.spot(none, {fallback_repr})");
        assert_eq!(
            expect_ok_str(&format!("repr(color.spot(none, {FALLBACK}))")),
            expected
        );
    }

    #[test]
    fn p1473_color_spot_equality_by_value() {
        assert_eq!(
            expect_ok(&format!(
                r#"color.spot("X", {FALLBACK}) == color.spot("X", {FALLBACK})"#
            )),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!(
                r#"color.spot("X", {FALLBACK}) != color.spot("Y", {FALLBACK})"#
            )),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1473_color_spot_rejects_spot_of_spot_fallback() {
        assert_eq!(
            expect_err_message(&format!(
                r#"color.spot("X", color.spot("Y", {FALLBACK}))"#
            )),
            "expected color, found spot colorant"
        );
    }

    #[test]
    fn p1473_color_spot_name_wrong_type() {
        assert_eq!(
            expect_err_message(&format!("color.spot(5, {FALLBACK})")),
            "expected \"all\", string, or none, found integer"
        );
    }

    #[test]
    fn p1473_color_spot_missing_fallback() {
        assert_eq!(
            expect_err_message(r#"color.spot("X")"#),
            "missing argument: fallback"
        );
    }

    #[test]
    fn p1473_color_parent_keeps_type_kind_and_direct_call() {
        // Não-regressão: `color` continua `type()==type` e os fields
        // pré-existentes (`color.rgb`, etc.) continuam a funcionar.
        assert_eq!(expect_ok("type(color) == type"), Value::Bool(true));
        eval("color.rgb(255, 0, 0)")
            .unwrap_or_else(|e| panic!("color.rgb(..) directo regrediu: {e:?}"));
    }
    #[test]
    fn p1482_tint_is_a_color_with_stable_repr_and_equality() {
        let source = format!(r#"color.spot("Brand", {FALLBACK})"#);
        assert_eq!(
            expect_ok(&format!("type({source}.tint(50%)) == color")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(50%))")),
            format!("{source}.tint(50%)")
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%) == {source}.tint(50%)")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%) != {source}.tint(25%)")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!("color.spot.tint({source}, 50%) == {source}.tint(50%)")),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1482_tint_bounds_and_errors() {
        let source = format!(r#"color.spot("Brand", {FALLBACK})"#);
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(0%))")),
            format!("{source}.tint(0%)")
        );
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(100%))")),
            format!("{source}.tint(100%)")
        );
        assert_eq!(
            expect_err_message(&format!("{source}.tint(-1%)")),
            "spot color tint must be positive"
        );
        assert_eq!(
            expect_err_message(&format!("{source}.tint(101%)")),
            "spot color tint must not exceed 100%"
        );
        assert_eq!(
            expect_err_message(&format!("{source}.tint(0.5)")),
            "expected ratio, found float"
        );
    }

    #[test]
    fn p1482_tint_process_projection_and_operations() {
        let source = format!(r##"color.spot("Brand", rgb("#ff4136"))"##);
        assert_eq!(expect_ok_str(&format!("{source}.tint(0%).to-hex()")), "#ffffff");
        assert_eq!(expect_ok_str(&format!("{source}.tint(100%).to-hex()")), "#ff4136");
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(50%).lighten(20%))")),
            format!("{source}.tint(40%)")
        );
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(50%).darken(20%))")),
            format!("{source}.tint(60%)")
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%).space() == {source}")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%).components() == (50%,)")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!(
                "{source}.tint(12.3456789%).components().at(0) == 12.3456789%"
            )),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1482_mix_and_gradient_keep_matching_spot_identity() {
        let spot = r#"color.spot("Brand", red)"#;
        let a = format!("{spot}.tint(50%)");
        let b = format!("{spot}.tint(25%)");
        let repr = expect_ok_str(&format!("repr({spot})"));
        assert_eq!(
            expect_ok_str(&format!("repr({a}.mix({b}))")),
            format!("{repr}.tint(37.5%)")
        );
        assert_eq!(
            expect_ok(&format!("gradient.linear({a}, {b}).space() == {spot}")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok_str(&format!("repr(gradient.linear({a}, {b}).sample(50%))")),
            format!("{repr}.tint(37.5%)")
        );
        assert_eq!(
            expect_ok(&format!("gradient.linear({a}, blue).space() == color.oklab")),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1482_rejects_incompatible_spot_space_and_operators() {
        let spot = r#"color.spot("Brand", red)"#;
        let a = format!("{spot}.tint(50%)");
        assert_eq!(
            expect_err_message(&format!("{a}.mix(blue, space: {spot})")),
            "cannot convert process color to spot colorant space"
        );
        assert_eq!(
            expect_err_message(&format!("gradient.linear({a}, blue, space: {spot})")),
            "cannot convert process color to spot colorant space"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.saturate(20%)")),
            "cannot saturate spot color"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.desaturate(20%)")),
            "cannot desaturate spot color"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.transparentize(20%)")),
            "spot colors do not have an alpha component"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.opacify(20%)")),
            "spot colors do not have an alpha component"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.rotate(90deg)")),
            "cannot convert process color to spot colorant space"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.negate(space: rgb)")),
            "cannot convert process color to spot colorant space"
        );
    }

    #[test]
    fn p1482_fallback_cast_and_tint_call_contract() {
        let spot = r##"color.spot("Brand", rgb("#ff4136"))"##;
        let a = format!("{spot}.tint(50%)");
        assert_eq!(expect_ok_str(&format!("rgb({a}).to-hex()")), "#ffa09a");
        assert_eq!(
            expect_err_message(&format!("{spot}.tint(value: 50%)")),
            "the argument `value` is positional"
        );
        assert_eq!(
            expect_err_message(&format!("{spot}.tint()")),
            "missing argument: value"
        );
        assert_eq!(
            expect_err_message("color.spot.tint(red, 50%)"),
            "expected spot colorant, found color"
        );
        assert_eq!(
            expect_err_message(&format!(r#"color.spot("X", {a})"#)),
            "spot color found where process color is required"
        );
    }
}
