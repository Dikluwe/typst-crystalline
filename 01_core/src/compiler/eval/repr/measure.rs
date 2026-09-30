//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/repr/measure.md
//! @prompt-hash 38b72e99
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
    push(&mut parts, "weeks", sign * weeks as i128);
    push(&mut parts, "days", sign * days as i128);
    push(&mut parts, "hours", sign * hours as i128);
    push(&mut parts, "minutes", sign * minutes as i128);
    push(&mut parts, "seconds", sign * seconds as i128);
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
    if !value.is_finite() {
        return value;
    }
    let offset = 10f64.powi(precision);
    (value * offset).round() / offset
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
    fn duration_preserves_nonzero_components() {
        let duration = crate::entities::duration::Duration::from_seconds(90);
        assert_eq!(repr_duration(&duration), "duration(minutes: 1, seconds: 30)");
    }
}
