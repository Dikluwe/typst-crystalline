//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/gradients/parsing.md
//! @prompt-hash 608ae388
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// P273 — Parser do named arg `relative` cross-variant (ADR-0091 §"Anotação
/// cumulativa P273").
///
/// Aceita `Value::Str("self" | "parent" | "auto")`. Default (sem named arg)
/// = `None` (Auto = `Self_` resolved). ADR-0064 §Caso A (`Smart<T>` →
/// `Option<T>` cristalino).
pub(super) fn parse_relative_named(
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
pub(super) fn parse_space_named(args: &Args, fn_name: &str) -> SourceResult<ColorSpace> {
    match args.named.get("space") {
        None => Ok(ColorSpace::Oklab),
        Some(Value::Auto) => Ok(ColorSpace::Oklab),
        Some(Value::SpotColorant(colorant)) => Ok(ColorSpace::Spot(colorant.clone())),
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

/// Escolhe o colorante como espaço automático somente quando todos os stops
/// carregam a mesma identidade spot.
pub(super) fn select_space(
    args: &Args,
    fn_name: &str,
    stops: &[GradientStop],
) -> SourceResult<ColorSpace> {
    if matches!(args.named.get("space"), None | Some(Value::Auto)) {
        if let Some(Color::Spot(first)) = stops.first().map(|stop| &stop.color) {
            if stops.iter().all(|stop| {
                matches!(&stop.color, Color::Spot(spot) if spot.colorant == first.colorant)
            }) {
                return Ok(ColorSpace::Spot((*first.colorant).clone()));
            }
        }
    }
    parse_space_named(args, fn_name)
}

pub(super) fn convert_stops_to_space(
    stops: &mut [GradientStop],
    space: &ColorSpace,
    args: &Args,
) -> SourceResult<()> {
    for stop in stops {
        stop.color = stop.color.clone().to_space(space.clone()).map_err(|error| {
            let message = match error {
                ColorSpaceError::ProcessToSpot { .. } => {
                    "cannot convert process color to spot colorant space"
                }
                ColorSpaceError::SpotToOther { .. } => {
                    "cannot convert spot color to different spot colorant"
                }
            };
            vec![SourceDiagnostic::error(args.span, message)]
        })?;
    }
    Ok(())
}

/// Parse cada item posicional para `GradientStop`.
///
/// Aceita:
/// - `Value::Color(c)` → `GradientStop::unspaced(c)` (offset auto).
/// - `Value::Array([Color, Ratio])` → `GradientStop::new(c, r)`.
pub(super) fn parse_stops(items: &[Value]) -> SourceResult<Vec<GradientStop>> {
    let mut stops = Vec::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        let stop = match item {
            Value::Color(c) => GradientStop::unspaced(c.clone()),
            Value::Array(arr) if arr.len() == 2 => {
                let c = match &arr[0] {
                    Value::Color(c) => c.clone(),
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

/// Helper privado partilhado: parse `Value::Ratio` ou `Float`
/// para `Ratio`.
pub(super) fn parse_ratio(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<Ratio> {
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
