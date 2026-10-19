//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/color/arguments.md
//! @prompt-hash 52dce5a3
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn extract_color_arg(
    val: &Value,
    fn_name: &str,
    arg_name: &str,
) -> SourceResult<Color> {
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

pub(super) fn extract_ratio_arg(
    val: &Value,
    fn_name: &str,
    arg_name: &str,
) -> SourceResult<f32> {
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
pub(super) fn extract_color_space_arg(
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

pub(super) fn color_space_diagnostic(error: ColorSpaceError) -> Vec<SourceDiagnostic> {
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

pub(super) fn check_space(color: &Color, space: &ColorSpace) -> SourceResult<()> {
    color
        .clone()
        .to_space(space.clone())
        .map(|_| ())
        .map_err(color_space_diagnostic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_color_behavior() {
        assert!(extract_color_arg(&Value::Int(1), "color.test", "col").is_err());
    }
}
