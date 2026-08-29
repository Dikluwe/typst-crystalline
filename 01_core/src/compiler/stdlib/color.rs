//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/color.md
//! @prompt-hash d7d15154
//! @layer L1
//! @updated 2026-06-27
//!
//! Módulo `color` — tipo `color` (P736/P1143) com constructors, operadores e
//! as 18 cores predefinidas do vanilla ratificado.
//! **P742** — semântica dos operadores corrigida por medição vanilla
//! (lighten/darken no espaço da cor, saturate/desaturate via HSV, negate em
//! Oklab) + despacho de métodos de instância (`red.lighten(20%)`).

use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::layout_types::Color;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

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
/// (as mesmas nativas registadas globalmente: `rgb`, `linear-rgb`, `luma`,
/// `cmyk`, `hsl`, `hsv`, `oklab`, `oklch`) + 9 operadores. Os nomes das
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
        _ => return predefined_color(field).map(Value::Color),
    };
    Some(function)
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
        .find_map(|(candidate, color)| (*candidate == name).then_some(*color))
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
        .map(|(name, color)| ((*name).into(), Value::Color(*color)))
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
        Value::Color(c) => Ok(*c),
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
    fn_name: &str,
    arg_name: &str,
) -> SourceResult<crate::entities::color::ColorSpace> {
    use crate::entities::color::ColorSpace;
    match val {
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
                "{}: argumento '{}' deve ser um espaço de cor, recebeu {}",
                fn_name,
                arg_name,
                val.type_name()
            )),
        },
        other => err_typed(format!(
            "{}: argumento '{}' deve ser um espaço de cor, recebeu {}",
            fn_name,
            arg_name,
            other.type_name()
        )),
    }
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
    let weight = match args.named.get("weight") {
        Some(v) => extract_ratio_arg(v, "color.mix", "weight")?,
        None => 0.5_f32,
    };
    let space = match args.named.get("space") {
        Some(v) => Some(extract_color_space_arg(v, "color.mix", "space")?),
        None => None,
    };
    for key in args.named.keys() {
        if !matches!(key.as_str(), "weight" | "space") {
            return err(format!("color.mix(): argumento nomeado inesperado '{}'", key));
        }
    }
    match args.items.as_slice() {
        [col1, col2] => {
            let c1 = extract_color_arg(col1, "color.mix", "col1")?;
            let c2 = extract_color_arg(col2, "color.mix", "col2")?;
            Ok(Value::Color(c1.mix(c2, weight, space)))
        }
        _ => err(format!(
            "color.mix() requer 2 argumentos posicionais (col1, col2), recebeu {}",
            args.items.len()
        )),
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
            let func = match c.space() {
                ColorSpace::Srgb => Func::native("rgb", native_rgb),
                ColorSpace::Luma => Func::native("luma", native_luma),
                ColorSpace::LinearRgb => Func::native("linear-rgb", native_linear_rgb),
                ColorSpace::Oklab => Func::native("oklab", native_oklab),
                ColorSpace::Oklch => Func::native("oklch", native_oklch),
                ColorSpace::Cmyk => Func::native("cmyk", native_cmyk),
                ColorSpace::Hsl => Func::native("hsl", native_hsl),
                ColorSpace::Hsv => Func::native("hsv", native_hsv),
            };
            Ok(Value::Func(func))
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
