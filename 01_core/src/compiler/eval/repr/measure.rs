//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/repr/measure.md
//! @prompt-hash 3f0ef450
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn repr_duration(d: &crate::entities::duration::Duration) -> String {
    const MINUTE: i128 = 60;
    const HOUR: i128 = 60 * MINUTE;
    const DAY: i128 = 24 * HOUR;
    const WEEK: i128 = 7 * DAY;

    if d.nanos == 0 {
        return "duration()".to_string();
    }

    let negative = d.nanos < 0;
    let mut rem = (d.nanos / 1_000_000_000).abs();
    let weeks = rem / WEEK;
    rem %= WEEK;
    let days = rem / DAY;
    rem %= DAY;
    let hours = rem / HOUR;
    rem %= HOUR;
    let minutes = rem / MINUTE;
    let seconds = rem % MINUTE;

    let mut parts: Vec<String> = Vec::with_capacity(5);
    let push = |parts: &mut Vec<String>, name: &str, value: i128| {
        if value != 0 {
            parts.push(format!("{name}: {value}"));
        }
    };
    let sign = if negative { -1 } else { 1 };
    push(&mut parts, "weeks", sign * weeks);
    push(&mut parts, "days", sign * days);
    push(&mut parts, "hours", sign * hours);
    push(&mut parts, "minutes", sign * minutes);
    push(&mut parts, "seconds", sign * seconds);
    format!("duration{}", pretty_array_like(&parts, false))
}

/// Representação de um comprimento relativo (`Rel<Length>`).
///
/// `Value::Relative` é `Rel<Length>` causal e conserva sempre as duas
/// componentes. Percentagem literal pura é `Value::Ratio`.
pub(super) fn repr_relative(rel: &Rel<Length>) -> String {
    format!(
        "{} + {}",
        format_float_with_unit(rel.rel * 100.0, "%"),
        repr_length(&rel.abs)
    )
}

/// `math.lr.size` é declarado como `Rel<Length>` no vanilla; a morfologia
/// do field conserva as duas componentes, inclusive quando uma delas é zero.
pub(super) fn repr_math_lr_size(rel: &Rel<Length>) -> String {
    format!(
        "{} + {}",
        format_float_with_unit(rel.rel * 100.0, "%"),
        repr_length(&rel.abs)
    )
}

// ── P721 — repr Typst (paridade vanilla), não Debug do Rust ────────────────

/// Arredondamento "half away from zero" a `precision` casas decimais —
/// paridade vanilla `round_with_precision` (typst-utils/src/round.rs:26).
/// Valores não-finitos passam inalterados.
pub(super) fn round_with_precision(value: f64, precision: i32) -> f64 {
    if !value.is_finite()
        || precision >= 15
        || (precision >= 0 && value.abs() >= 9_007_199_254_740_992.0)
    {
        return value;
    }
    if precision < -308 {
        return value * 0.0;
    }
    let offset = libm::exp10(f64::from(precision.abs()));
    if precision > 0 {
        (value * offset).round() / offset
    } else {
        (value / offset).round() * offset
    }
}

/// Float formatado com unidade, arredondado a 2 casas decimais — paridade
/// vanilla `repr::format_float_with_unit` (foundations/repr.rs:128).
/// NaN/Inf seguem a forma `float.nan * 1{unit}` / `float.inf * 1{unit}`.
pub(super) fn format_float_with_unit(value: f64, unit: &str) -> String {
    format_float_rounded(value, 2, unit)
}

/// Componente float de cor (a/b de Oklab, chroma de Oklch), arredondado a
/// 3 casas decimais — paridade vanilla `repr::format_float_component`
/// (foundations/repr.rs:122).
pub(super) fn format_float_component(value: f64) -> String {
    format_float_rounded(value, 3, "")
}

pub(super) fn format_float_rounded(value: f64, precision: i32, unit: &str) -> String {
    let value = round_with_precision(value, precision);
    let multiplication = if unit.is_empty() { "" } else { " * 1" };
    if value.is_nan() {
        format!("float.nan{multiplication}{unit}")
    } else if value.is_infinite() {
        let sign = if value < 0.0 { "-" } else { "" };
        format!("{sign}float.inf{multiplication}{unit}")
    } else {
        format!("{value}{unit}")
    }
}

/// Representação de um `Length` — paridade vanilla `Length::repr`
/// (layout/length.rs:173): `"{abs} + {em}"` quando ambos não-zero;
/// só `em` quando abs é zero; só `abs` nos restantes casos.
pub(super) fn repr_length(l: &Length) -> String {
    let abs = format_float_with_unit(l.abs.to_pt(), "pt");
    let em = format_float_with_unit(l.em, "em");
    match (l.abs.is_zero(), l.em == 0.0) {
        (false, false) => format!("{abs} + {em}"),
        (true, false) => em,
        (_, true) => abs,
    }
}

/// Representação de um `Ratio` — paridade vanilla `Ratio::repr`
/// (layout/ratio.rs:135).
pub(super) fn repr_ratio(r: Ratio) -> String {
    format_float_with_unit(r.get() * 100.0, "%")
}

/// Representação de um `Angle` em graus — paridade vanilla `Angle::repr`
/// (layout/angle.rs:171).
pub(super) fn repr_angle(a: Angle) -> String {
    format_float_with_unit(a.to_deg(), "deg")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_large_representations_preserve_value() {
        for value in [1e308, -1e308, 9_007_199_254_740_992.0] {
            for unit in ["em", "pt", "deg", "%"] {
                let text = format_float_with_unit(value, unit);
                let restored: f64 = text.strip_suffix(unit).unwrap().parse().unwrap();
                assert!(restored.is_finite(), "{text}");
                assert_eq!(restored, value);
            }
            let text = format_float_component(value);
            assert_eq!(text.parse::<f64>().unwrap(), value);
        }
    }

    #[test]
    fn extreme_precision_preserves_value_or_signed_zero() {
        for value in [1.25, -1.25] {
            assert_eq!(round_with_precision(value, i32::MAX), value);
            let zero = round_with_precision(value, i32::MIN);
            assert_eq!(zero, 0.0);
            assert_eq!(zero.is_sign_negative(), value.is_sign_negative());
        }
    }

    #[test]
    fn coarse_precision_avoids_intermediate_underflow() {
        for (value, expected) in [(1.4e308, 1e308), (-1.4e308, -1e308)] {
            assert_eq!(round_with_precision(value, -308), expected);
        }
        let zero = round_with_precision(-1e-300, -309);
        assert_eq!(zero, 0.0);
        assert!(zero.is_sign_negative());
    }

    #[test]
    fn representation_keeps_ties_units_and_nonfinite_policy() {
        for (value, expected) in [(1.125, "1.13em"), (-1.125, "-1.13em"), (-0.0, "-0em")]
        {
            assert_eq!(format_float_with_unit(value, "em"), expected);
        }
        assert_eq!(format_float_component(0.0625), "0.063");
        assert_eq!(format_float_with_unit(f64::NAN, "em"), "float.nan * 1em");
        assert_eq!(format_float_component(f64::NAN), "float.nan");
        for (value, expected) in
            [(f64::INFINITY, "float.inf * 1em"), (f64::NEG_INFINITY, "-float.inf * 1em")]
        {
            assert_eq!(format_float_with_unit(value, "em"), expected);
            assert_eq!(round_with_precision(value, i32::MIN), value);
        }
        assert_eq!(format_float_rounded(1.6e308, -308, "em"), "float.inf * 1em");
    }

    #[test]
    fn duration_preserves_nonzero_components() {
        let duration = crate::entities::duration::Duration::from_seconds(90);
        assert_eq!(repr_duration(&duration), "duration(minutes: 1, seconds: 30)");
    }
}
