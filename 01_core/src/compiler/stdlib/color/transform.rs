//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/color/transform.md
//! @prompt-hash 0aacab3b
//! @layer L1
//! @updated 2026-09-30

use super::*;

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

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_color_behavior() {
        let args = Args::positional(vec![Value::Color(Color::rgb(255, 0, 0))]);
        assert!(matches!(call(native_color_negate, args).unwrap(), Value::Color(_)));
    }
}
