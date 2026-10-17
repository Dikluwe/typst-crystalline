//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/color/mix.md
//! @prompt-hash 5482b9b8
//! @layer L1
//! @updated 2026-09-30

use super::*;

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

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_color_behavior() {
        let args = Args::positional(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        assert!(matches!(call(native_color_mix, args).unwrap(), Value::Color(_)));
    }
}
