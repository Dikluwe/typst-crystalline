//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/color/inspect.md
//! @prompt-hash d9a6f036
//! @layer L1
//! @updated 2026-09-30

use super::*;

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
            use crate::compiler::stdlib::foundations::{
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

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_color_behavior() {
        let args = Args::positional(vec![Value::Color(Color::rgb(255, 0, 0))]);
        assert!(matches!(call(native_color_to_hex, args).unwrap(), Value::Str(_)));
    }
}
